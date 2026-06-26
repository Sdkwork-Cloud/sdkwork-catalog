use axum::Router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use std::sync::Arc;

pub mod routes;
pub mod web_bootstrap;

pub use routes::build_catalog_backend_router_with_framework;

pub async fn gateway_mount(host: Arc<CatalogServiceHost>) -> Router {
    build_catalog_backend_router_with_framework(host).await
}
