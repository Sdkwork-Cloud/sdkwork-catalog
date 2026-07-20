//! API assembly for sdkwork-catalog.
//! Application bootstrap lives in `bootstrap.rs`; route inventory is in `assembly-manifest.json`.
// SDKWORK-ASSEMBLY-LIB-CUSTOM

mod bootstrap;
mod generated;

pub use bootstrap::{assemble_api_router, ApiAssembly};

pub fn assembly_route_count() -> usize {
    generated::ROUTE_CRATE_COUNT
}
