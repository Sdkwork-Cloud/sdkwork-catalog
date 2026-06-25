use axum::Router;
use sdkwork_router_catalog_app_api::build_catalog_app_router_with_framework;
use sdkwork_router_catalog_backend_api::build_catalog_backend_router_with_framework;
use sdkwork_catalog_api_server::catalog_health_router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let host = Arc::new(CatalogServiceHost::new().await);
    let app = Router::new()
        .merge(catalog_health_router())
        .merge(build_catalog_app_router_with_framework(host.clone()).await)
        .merge(build_catalog_backend_router_with_framework(host).await)
        .layer(CorsLayer::permissive());
    let addr = std::env::var("CATALOG_API_BIND").unwrap_or_else(|_| "0.0.0.0:18099".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
