use axum::Router;
use sqlx::{PgPool, SqlitePool};

pub fn build_catalog_app_router_with_sqlite_pool(pool: SqlitePool) -> Router {
    crate::app_catalog_router::app_catalog_router_with_sqlite_pool(pool)
}

pub fn build_catalog_app_router_with_postgres_pool(pool: PgPool) -> Router {
    crate::app_catalog_router::app_catalog_router_with_postgres_pool(pool)
}

pub async fn build_catalog_app_router_with_framework_sqlite(pool: SqlitePool) -> Router {
    crate::web_bootstrap::wrap_router_with_web_framework_from_env(
        build_catalog_app_router_with_sqlite_pool(pool),
    )
    .await
}

pub async fn build_catalog_app_router_with_framework_postgres(pool: PgPool) -> Router {
    crate::web_bootstrap::wrap_router_with_web_framework_from_env(
        build_catalog_app_router_with_postgres_pool(pool),
    )
    .await
}
