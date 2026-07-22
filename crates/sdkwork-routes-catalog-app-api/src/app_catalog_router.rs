//! App browse/open catalog HTTP routes (owned by catalog capability).

use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::response::Response;
use axum::routing::{get, patch, put};
use axum::{Json, Router};
use sdkwork_iam_context_service::IamAppContext;
use sdkwork_merchandise_repository_sqlx::{
    PostgresCommerceCatalogStore, SqliteCommerceCatalogStore,
};
use sdkwork_merchandise_service::{
    AddCartItemCommand, AddressListQuery, AttributeListQuery, CartRetrieveQuery, CategoryListQuery,
    CategoryRetrieveQuery, CreateAddressCommand, DeleteAddressCommand, ProductSkuListQuery,
    ProductSkuRetrieveQuery, ProductSpuListQuery, ProductSpuRetrieveQuery, RemoveCartItemCommand,
    SetDefaultAddressCommand, UpdateAddressCommand, UpdateCartItemCommand,
};
use sdkwork_merchandise_web_support::{
    catalog_system_response, map_address, map_attribute, map_cart_item, map_category, map_sku,
    map_spu, not_found_response, success_created_resource, success_no_content, success_offset_page,
    success_resource, unauthorized_response, validation_response, AddCartItemBody, CatalogState,
    CommerceCatalogStore, CreateAddressBody, UpdateAddressBody, UpdateCartItemBody,
};
use serde::Deserialize;
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
        .route(
            "/app/v3/api/catalog/products/{productId}/skus",
            get(app_list_product_skus),
        )
        .route("/app/v3/api/catalog/skus/{skuId}", get(app_retrieve_sku))
        .route(
            "/app/v3/api/cart/items",
            get(app_list_cart).post(app_add_cart_item),
        )
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
            put(app_set_default_address),
        )
        .with_state(CatalogState { store })
}
async fn app_list_categories(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<AppCategoryListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CategoryListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        params.parent_id.as_deref(),
        Some("active"),
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_categories_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_category).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
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
        Ok(Some(category)) => success_resource(map_category(category)),
        Ok(None) => not_found_response("category was not found"),
        Err(error) => catalog_system_response("category read model is unavailable", error),
    }
}

async fn app_list_attributes(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<OffsetListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match AttributeListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        Some("active"),
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_attributes_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_attribute).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_system_response("attribute list is unavailable", error),
    }
}

async fn app_list_products(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<AppProductListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSpuListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        params.category_id.as_deref(),
        params.product_type.as_deref(),
        Some("active"),
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_spus_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_spu).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_system_response("product list is unavailable", error),
    }
}

#[derive(Debug, Deserialize)]
struct ProductSkuListQueryParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct OffsetListQueryParams {
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct AppCategoryListQueryParams {
    parent_id: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct AppProductListQueryParams {
    category_id: Option<String>,
    product_type: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

async fn app_list_product_skus(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Path(product_id): Path<String>,
    Query(params): Query<ProductSkuListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match ProductSkuListQuery::new(
        &subject.tenant_id,
        subject.organization_id.as_deref(),
        Some(&product_id),
        Some("active"),
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_skus_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_sku).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
        Err(error) => catalog_system_response("product sku list is unavailable", error),
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
        Ok(Some(data)) => success_resource(map_spu(data)),
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
        Ok(Some(data)) => success_resource(map_sku(data)),
        Ok(None) => not_found_response("sku was not found"),
        Err(error) => catalog_system_response("sku read model is unavailable", error),
    }
}

async fn app_list_cart(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<OffsetListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match CartRetrieveQuery::new(
        &subject.tenant_id,
        &subject.user_id,
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_cart_items_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_cart_item).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
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
        Ok(data) => success_created_resource(map_cart_item(data)),
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
        Ok(data) => success_resource(map_cart_item(data)),
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
        Ok(()) => success_no_content(),
        Err(error) => catalog_system_response("failed to remove cart item", error),
    }
}

async fn app_list_addresses(
    State(state): State<CatalogState>,
    runtime_context: Option<Extension<IamAppContext>>,
    Query(params): Query<OffsetListQueryParams>,
) -> Response {
    let subject = match app_runtime_subject_from_extension(runtime_context) {
        Ok(subject) => subject,
        Err(message) => return unauthorized_response(message),
    };
    let query = match AddressListQuery::new(
        &subject.tenant_id,
        &subject.user_id,
        params.page,
        params.page_size,
    ) {
        Ok(query) => query,
        Err(error) => return validation_response(error.message()),
    };
    match state.store.list_addresses_page(query).await {
        Ok(data) => success_offset_page(
            data.items.into_iter().map(map_address).collect(),
            data.page,
            data.page_size,
            data.total_items,
        ),
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
        Ok(data) => success_created_resource(map_address(data)),
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
        Ok(data) => success_resource(map_address(data)),
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
        Ok(()) => success_no_content(),
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
        Ok(data) => success_resource(map_address(data)),
        Err(error) => catalog_system_response("failed to set default address", error),
    }
}
