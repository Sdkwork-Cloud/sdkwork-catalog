//! Catalog browse read-model adapter crate.
//!
//! Browse/open HTTP in `sdkwork-catalog` uses merchandise persistence through the explicit
//! adapter boundary defined in [`read_adapter`].

mod read_adapter;

pub use read_adapter::{
    PostgresCommerceCatalogReadStore as PostgresCommerceCatalogStore,
    SqliteCommerceCatalogReadStore as SqliteCommerceCatalogStore,
};
