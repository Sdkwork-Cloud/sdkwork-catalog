use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use sdkwork_catalog_service_host::CatalogServiceHost;

pub fn build_catalog_backend_router(_host: Arc<CatalogServiceHost>) -> Router {
    Router::new().route(
        "/backend/v3/api/catalog/health",
        get(|| async { "ok" }),
    )
}

pub async fn build_catalog_backend_router_with_framework(host: Arc<CatalogServiceHost>) -> Router {
    build_catalog_backend_router(host)
}
