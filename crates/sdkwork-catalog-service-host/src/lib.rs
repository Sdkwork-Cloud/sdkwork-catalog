use sdkwork_database_sqlx::DatabasePool;
use sdkwork_catalog_database_host::{bootstrap_catalog_database_from_env, CatalogDatabaseHost};

pub struct CatalogServiceHost {
    database: CatalogDatabaseHost,
}

impl CatalogServiceHost {
    pub async fn new() -> Self {
        Self::from_env().await.expect("catalog service host bootstrap failed")
    }

    pub async fn from_env() -> Result<Self, String> {
        let database = bootstrap_catalog_database_from_env().await?;
        Ok(Self { database })
    }

    pub fn database_pool(&self) -> &DatabasePool {
        self.database.pool()
    }

    pub fn database_module(&self) -> std::sync::Arc<sdkwork_database_spi::DefaultDatabaseModule> {
        self.database.module()
    }
}
