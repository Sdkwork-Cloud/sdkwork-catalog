//! API assembly bootstrap for sdkwork-catalog.

use axum::Router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use std::sync::Arc;

pub struct ApiAssembly {
    pub router: Router,
}

pub async fn assemble_api_router(host: Arc<CatalogServiceHost>) -> ApiAssembly {
    let mut router = Router::new();
    router = router.merge(
        sdkwork_routes_catalog_app_api::build_catalog_app_router_with_framework(host.clone()).await,
    );
    ApiAssembly { router }
}
