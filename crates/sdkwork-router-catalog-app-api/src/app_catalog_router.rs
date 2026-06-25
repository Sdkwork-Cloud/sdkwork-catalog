//! App browse/open catalog HTTP routes (owned by catalog capability).

use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, patch, post};
use axum::{Json, Router};
use sdkwork_commerce_merchandise_service::{
    AddCartItemCommand, AddressListQuery, AttributeListQuery, CartRetrieveQuery, CategoryListQuery,
    CategoryRetrieveQuery, CreateAddressCommand, DeleteAddressCommand, ProductSkuRetrieveQuery,
    ProductSpuListQuery, ProductSpuRetrieveQuery, RemoveCartItemCommand, SetDefaultAddressCommand,
    SkuPriceRetrieveQuery, UpdateAddressCommand, UpdateCartItemCommand,
};
use sdkwork_commerce_merchandise_repository_sqlx::{
    PostgresCommerceCatalogStore, SqliteCommerceCatalogStore,
};
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_router_merchandise_app_api::{
    catalog_system_response, map_address, map_attribute, map_cart_item, map_category,
    map_price_list_item, map_sku, map_spu, not_found_response, unauthorized_response,
    validation_response, AddCartItemBody, AttributeQueryParams, CatalogApiResult, CatalogState,
    CategoryQueryParams, CommerceCatalogStore, CreateAddressBody, SpuListQueryParams,
    UpdateAddressBody, UpdateCartItemBody,
};
use sqlx::{PgPool, SqlitePool};

use crate::subject::app_runtime_subject_from_extension;

pub fn app_catalog_router_with_sqlite_pool(pool: SqlitePool) -> Router {
    build_app_catalog_router(Arc::new(SqliteCommerceCatalogStore::new(pool)))
}

pub fn app_catalog_router_with_postgres_pool(pool: PgPool) -> Router {
    build_app_catalog_router(Arc::new(PostgresCommerceCatalogStore::new(pool)))
}

pub fn build_app_catalog_router(store: Arc<dyn CommerceCatalogStore>) -> Router {
    Router::new()
            .route("/app/v3/api/catalog/categories", get(app_list_categories))
            .route(
                "/app/v3/api/catalog/categories/{categoryId}",
                get(app_retrieve_category),
            )
            .route("/app/v3/api/catalog/attributes", get(app_list_attributes))
            .route("/app/v3/api/catalog/products", get(app_list_products))
            .route(
                "/app/v3/api/catalog/products/{productId}",
                get(app_retrieve_product),
            )
            .route("/app/v3/api/catalog/skus/{skuId}", get(app_retrieve_sku))
            .route(
                "/app/v3/api/catalog/skus/{skuId}/prices",
                get(app_retrieve_sku_prices),
            )
            .route("/app/v3/api/catalog/spus", get(app_list_spus))
            .route("/app/v3/api/catalog/spus/{spuId}", get(app_retrieve_spu))
            .route("/app/v3/api/cart/current", get(app_list_cart))
            .route("/app/v3/api/cart/items", post(app_add_cart_item))
            .route(
                "/app/v3/api/cart/items/{cartItemId}",
                patch(app_update_cart_item).delete(app_remove_cart_item),
            )
            .route(
                "/app/v3/api/addresses",
                get(app_list_addresses).post(app_create_address),
            )
            .route(
                "/app/v3/api/addresses/{addressId}",
                patch(app_update_address).delete(app_delete_address),
            )
            .route(
                "/app/v3/api/addresses/{addressId}/default_selection",
                post(app_set_default_address),
            )
            .with_state(CatalogState { store })
}
async fn app_list_categories(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<CategoryQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CategoryListQuery::new(
        &subject.tenant_id,
        subject
            .organization_id
            .as_deref()
            .or(params.organization_id.as_deref()),
        params.parent_id.as_deref(),
        params.status.as_deref(),
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_categories(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter().map(map_category).collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("category list is unavailable", error),
    }
}

async fn app_retrieve_category(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(category_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CategoryRetrieveQuery::new(&subject.tenant_id, &category_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message().to_string()),
    };
    match state.store.retrieve_category(query).await {
        Ok(Some(category)) => {
            Json(CatalogApiResult::success(map_category(category))).into_response()
        }
        Ok(None) => not_found_response("category was not found"),
        Err(error) => catalog_system_response("category read model is unavailable", error),
    }
}

async fn app_list_attributes(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<AttributeQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match AttributeListQuery::new(
        &subject.tenant_id,
        subject
            .organization_id
            .as_deref()
            .or(params.organization_id.as_deref()),
        params.status.as_deref(),
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_attributes(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter().map(map_attribute).collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("attribute list is unavailable", error),
    }
}

async fn app_list_products(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<SpuListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuListQuery::new(
        &subject.tenant_id,
        subject
            .organization_id
            .as_deref()
            .or(params.organization_id.as_deref()),
        params.category_id.as_deref(),
        params.product_type.as_deref(),
        Some("active"),
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_spus(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter().map(map_spu).collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("product list is unavailable", error),
    }
}

async fn app_retrieve_product(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuRetrieveQuery::new(&subject.tenant_id, &product_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.retrieve_spu(query).await {
        Ok(Some(data)) => Json(CatalogApiResult::success(map_spu(data))).into_response(),
        Ok(None) => not_found_response("product was not found"),
        Err(error) => catalog_system_response("product read model is unavailable", error),
    }
}

async fn app_retrieve_sku(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(sku_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSkuRetrieveQuery::new(&subject.tenant_id, &sku_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.retrieve_sku(query).await {
        Ok(Some(data)) => Json(CatalogApiResult::success(map_sku(data))).into_response(),
        Ok(None) => not_found_response("sku was not found"),
        Err(error) => catalog_system_response("sku read model is unavailable", error),
    }
}

async fn app_retrieve_sku_prices(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(sku_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match SkuPriceRetrieveQuery::new(&subject.tenant_id, &sku_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.retrieve_sku_prices(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter()
                .map(map_price_list_item)
                .collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("sku prices are unavailable", error),
    }
}

async fn app_list_spus(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<SpuListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuListQuery::new(
        &subject.tenant_id,
        subject
            .organization_id
            .as_deref()
            .or(params.organization_id.as_deref()),
        params.category_id.as_deref(),
        params.product_type.as_deref(),
        params.status.as_deref(),
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_spus(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter().map(map_spu).collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("spu list is unavailable", error),
    }
}

async fn app_retrieve_spu(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(spu_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuRetrieveQuery::new(&subject.tenant_id, &spu_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.retrieve_spu(query).await {
        Ok(Some(data)) => Json(CatalogApiResult::success(map_spu(data))).into_response(),
        Ok(None) => not_found_response("spu was not found"),
        Err(error) => catalog_system_response("spu read model is unavailable", error),
    }
}

async fn app_list_cart(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CartRetrieveQuery::new(&subject.tenant_id, &subject.user_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_cart_items(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter().map(map_cart_item).collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("cart read model is unavailable", error),
    }
}

async fn app_add_cart_item(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<AddCartItemBody>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = AddCartItemCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        sku_id: body.sku_id,
        quantity: body.quantity,
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.add_cart_item(command).await {
        Ok(data) => Json(CatalogApiResult::success(map_cart_item(data))).into_response(),
        Err(error) => catalog_system_response("failed to add cart item", error),
    }
}

async fn app_update_cart_item(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(cart_item_id): Path<String>,
    Json(body): Json<UpdateCartItemBody>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = UpdateCartItemCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        cart_item_id,
        quantity: body.quantity,
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.update_cart_item(command).await {
        Ok(data) => Json(CatalogApiResult::success(map_cart_item(data))).into_response(),
        Err(error) => catalog_system_response("failed to update cart item", error),
    }
}

async fn app_remove_cart_item(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(cart_item_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = RemoveCartItemCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        cart_item_id,
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.remove_cart_item(command).await {
        Ok(()) => Json(CatalogApiResult::success(())).into_response(),
        Err(error) => catalog_system_response("failed to remove cart item", error),
    }
}

async fn app_list_addresses(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match AddressListQuery::new(&subject.tenant_id, &subject.user_id) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_addresses(query).await {
        Ok(data) => Json(CatalogApiResult::success(
            data.into_iter().map(map_address).collect::<Vec<_>>(),
        ))
        .into_response(),
        Err(error) => catalog_system_response("address list is unavailable", error),
    }
}

async fn app_create_address(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Json(body): Json<CreateAddressBody>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = CreateAddressCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        address_id: body.address_id,
        receiver_name: body.receiver_name,
        receiver_phone: body.receiver_phone,
        country_code: body.country_code,
        province: body.province,
        city: body.city,
        detail_address: body.detail_address,
        is_default: body.is_default.unwrap_or(false),
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.create_address(command).await {
        Ok(data) => Json(CatalogApiResult::success(map_address(data))).into_response(),
        Err(error) => catalog_system_response("failed to create address", error),
    }
}

async fn app_update_address(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(address_id): Path<String>,
    Json(body): Json<UpdateAddressBody>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = UpdateAddressCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        address_id,
        receiver_name: body.receiver_name,
        receiver_phone: body.receiver_phone,
        province: body.province,
        city: body.city,
        detail_address: body.detail_address,
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.update_address(command).await {
        Ok(data) => Json(CatalogApiResult::success(map_address(data))).into_response(),
        Err(error) => catalog_system_response("failed to update address", error),
    }
}

async fn app_delete_address(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(address_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = DeleteAddressCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        address_id,
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.delete_address(command).await {
        Ok(()) => Json(CatalogApiResult::success(())).into_response(),
        Err(error) => catalog_system_response("failed to delete address", error),
    }
}

async fn app_set_default_address(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(address_id): Path<String>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let command = SetDefaultAddressCommand {
        tenant_id: subject.tenant_id,
        owner_user_id: subject.user_id,
        address_id,
    };
    match command.validate() {
        Ok(()) => {}
        Err(error) => return validation_response(error.message()),
    }
    match state.store.set_default_address(command).await {
        Ok(data) => Json(CatalogApiResult::success(map_address(data))).into_response(),
        Err(error) => catalog_system_response("failed to set default address", error),
    }
}
