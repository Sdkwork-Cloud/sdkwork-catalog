//! Merchandise-owned catalog table SQL exposed through the catalog browse read boundary.
//!
//! `sdkwork-merchandise` owns DDL and admin write paths. `sdkwork-catalog` consumes browse
//! read stores only via this adapter module — no duplicate catalog SQL in this repository.

pub use sdkwork_merchandise_repository_sqlx::{
    PostgresCommerceCatalogStore as PostgresCommerceCatalogReadStore,
    SqliteCommerceCatalogStore as SqliteCommerceCatalogReadStore,
};
