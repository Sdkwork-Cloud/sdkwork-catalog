//! API assembly for sdkwork-catalog.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, ApiAssembly};

pub async fn assemble_api_router_from_env() -> Result<ApiAssembly, String> {
    let host = sdkwork_catalog_service_host::CatalogServiceHost::from_env().await?;
    Ok(assemble_api_router(std::sync::Arc::new(host)).await)
}

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
