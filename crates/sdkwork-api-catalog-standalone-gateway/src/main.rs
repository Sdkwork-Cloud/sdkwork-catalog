use sdkwork_api_catalog_assembly::assemble_api_router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use sdkwork_web_bootstrap::{service_router, ServiceRouterConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let host = Arc::new(CatalogServiceHost::new().await);
    let business = assemble_api_router(host).await.router
        .layer(sdkwork_web_bootstrap::application_cors_layer_from_env(
            &["SDKWORK_CATALOG_ENVIRONMENT"],
            &["SDKWORK_CATALOG_CORS_ALLOWED_ORIGINS", "SDKWORK_CORS_ALLOWED_ORIGINS"],
        ));
    let app = service_router(business, ServiceRouterConfig::default().with_always_ready());
    let addr = std::env::var("CATALOG_API_BIND").unwrap_or_else(|_| "0.0.0.0:18099".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("bind");
    axum::serve(listener, app).await.expect("serve");
}
