pub mod app_catalog_router;
pub mod routes;
pub mod subject;
pub mod web_bootstrap;

pub use app_catalog_router::{
    app_catalog_router_with_postgres_pool, app_catalog_router_with_sqlite_pool,
    build_app_catalog_router,
};
pub use routes::{
    build_catalog_app_router_with_framework_postgres, build_catalog_app_router_with_framework_sqlite,
    build_catalog_app_router_with_postgres_pool, build_catalog_app_router_with_sqlite_pool,
};
pub use web_bootstrap::wrap_router_with_web_framework_from_env;

use axum::Router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use sdkwork_database_sqlx::DatabasePool;
use std::sync::Arc;

/// Standalone catalog api-server entry: resolves browse/open routes from the service host pool.
pub async fn build_catalog_app_router_with_framework(host: Arc<CatalogServiceHost>) -> Router {
    match host.database_pool() {
        DatabasePool::Sqlite(pool, _) => {
            build_catalog_app_router_with_framework_sqlite(pool.clone()).await
        }
        DatabasePool::Postgres(pool, _) => {
            build_catalog_app_router_with_framework_postgres(pool.clone()).await
        }
    }
}
