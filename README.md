# sdkwork-catalog

repository-kind: application

SDKWork commerce catalog capability repository. It provides tenant-scoped product browsing,
authenticated cart and delivery-address APIs, SQL-backed read models, standalone hosting, and the
owner-only Catalog App SDK.

- Standards: `../sdkwork-specs/README.md`
- App API authority: `sdkwork-catalog-app-api`
- App SDK family: `sdks/sdkwork-catalog-app-sdk`
- API assembly: `crates/sdkwork-api-catalog-assembly`
- Standalone gateway: `crates/sdkwork-api-catalog-standalone-gateway`

## Canonical App Surface

- Catalog browse: `/app/v3/api/catalog/categories`, `/attributes`, `/products`, and product SKUs
- Cart: `/app/v3/api/cart/items`
- Delivery addresses: `/app/v3/api/addresses`
- Typed SDK package: `@sdkwork/catalog-app-sdk`

List operations use offset pagination with `page` and `page_size`, return
`SdkWorkApiResponse.data.items` plus `pageInfo`, and execute filtering and paging in SQL.

## Verification

```powershell
cargo test --workspace
pnpm sdk:generate
node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-sdk-standard.mjs --workspace .
```

## Documentation Canon

- [docs/README.md](docs/README.md)
- [docs/product/prd/PRD.md](docs/product/prd/PRD.md)
- [docs/architecture/tech/TECH_ARCHITECTURE.md](docs/architecture/tech/TECH_ARCHITECTURE.md)
