//! API assembly bootstrap for sdkwork-catalog.
//!
//! The assembly exports the indivisible `ApiAssemblyContribution` contract
//! (API_ASSEMBLY_SPEC.md section 4); the platform cloud gateway composes the
//! contribution with its process-shared PostgreSQL pool.

use axum::Router;
use sdkwork_catalog_service_host::CatalogServiceHost;
use sdkwork_database_sqlx::DatabasePool;
use sdkwork_web_bootstrap::{
    ApiAssemblyContribution, DatabasePoolReadinessCheck, ReadinessCheck,
};
use std::sync::Arc;

/// Indivisible host-neutral API assembly contribution (web-bootstrap contract).
pub type ApiAssembly = ApiAssemblyContribution;

fn contribution_from(
    router: Router,
    readiness_check: Arc<dyn ReadinessCheck>,
) -> Result<ApiAssembly, String> {
    ApiAssemblyContribution::from_manifest(
        "sdkwork-catalog",
        "SDKWork Catalog API",
        router,
        sdkwork_routes_catalog_app_api::gateway_route_manifest(),
        Vec::new(),
        readiness_check,
    )
}

pub async fn assemble_api_router(host: Arc<CatalogServiceHost>) -> ApiAssembly {
    let DatabasePool::Postgres(pool, _) = host.database_pool() else {
        panic!("catalog app router requires a PostgreSQL database pool");
    };
    let router = sdkwork_routes_catalog_app_api::build_catalog_app_router_with_postgres_pool(
        pool.clone(),
    );
    contribution_from(
        router,
        Arc::new(sdkwork_web_bootstrap::AlwaysReady),
    )
    .expect("catalog contribution contract is valid")
}

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let host = Arc::new(CatalogServiceHost::from_env().await?);
    Ok(assemble_api_router(host).await)
}

/// Assemble the Catalog contribution against a caller-provided database pool so
/// the platform cloud gateway can share its process-wide PostgreSQL pool.
pub async fn assemble_api_router_with_pool(pool: DatabasePool) -> Result<ApiAssembly, String> {
    let postgres = pool
        .as_postgres()
        .ok_or_else(|| "catalog requires a PostgreSQL database pool".to_owned())?
        .clone();
    let router =
        sdkwork_routes_catalog_app_api::build_catalog_app_router_with_postgres_pool(postgres);
    contribution_from(
        router,
        Arc::new(DatabasePoolReadinessCheck::new(pool)),
    )
}
