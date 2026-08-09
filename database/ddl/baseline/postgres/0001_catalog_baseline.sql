-- sdkwork:baseline
-- module: catalog
--
-- Buyer-side commerce tables owned by the catalog capability (browsing cart
-- and delivery addresses). Product master data (commerce_product_spu/sku and
-- the category/attribute/price-list tables) is owned by sdkwork-merchandise;
-- the catalog app surface reads it through the merchandise catalog store and
-- never declares those tables here (no duplicate DDL).
-- Column shape mirrors the merchandise repository SQL
-- (crates/sdkwork-merchandise-repository-sqlx/src/postgres_catalog.rs):
-- text ids, text timestamps read back through string_cell.
CREATE TABLE IF NOT EXISTS commerce_cart (
    id TEXT NOT NULL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    owner_user_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_commerce_cart_owner
    ON commerce_cart (tenant_id, owner_user_id, status);

CREATE TABLE IF NOT EXISTS commerce_cart_item (
    id TEXT NOT NULL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    cart_id TEXT NOT NULL,
    sku_id TEXT NOT NULL,
    quantity BIGINT NOT NULL DEFAULT 1,
    selected_options_hash TEXT NOT NULL DEFAULT '',
    selected_options_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT ux_commerce_cart_item_sku_options
        UNIQUE (tenant_id, cart_id, sku_id, selected_options_hash)
);

CREATE INDEX IF NOT EXISTS idx_commerce_cart_item_cart
    ON commerce_cart_item (tenant_id, cart_id);

CREATE TABLE IF NOT EXISTS commerce_user_address (
    id TEXT NOT NULL PRIMARY KEY,
    tenant_id TEXT NOT NULL,
    owner_user_id TEXT NOT NULL,
    receiver_name TEXT NOT NULL,
    receiver_phone TEXT NOT NULL,
    country_code TEXT NOT NULL,
    province TEXT NOT NULL,
    city TEXT NOT NULL,
    detail_address TEXT NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_commerce_user_address_owner
    ON commerce_user_address (tenant_id, owner_user_id, status);
