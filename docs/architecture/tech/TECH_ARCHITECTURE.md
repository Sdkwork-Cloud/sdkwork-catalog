# Catalog Technical Architecture

Status: active
Owner: SDKWork maintainers
Updated: 2026-07-22
Specs: ARCHITECTURE_DECISION_SPEC.md, RUST_CODE_SPEC.md, API_SPEC.md, PAGINATION_SPEC.md,
WEB_FRAMEWORK_SPEC.md, SDK_SPEC.md

## Architecture

```text
sdkwork-routes-catalog-app-api
  -> sdkwork-merchandise-web-support::CommerceCatalogStore
  -> PostgresCommerceCatalogStore / SqliteCommerceCatalogStore
  -> commerce_* tables

catalog-app-api.openapi.json
  -> owner-only .sdkgen.json
  -> @sdkwork/sdk-generator
  -> @sdkwork/catalog-app-sdk
```

`sdkwork-api-catalog-assembly` composes executable routers. The standalone gateway supplies the
SDKWork web-framework chain and IAM request context. Platform deployment may embed the same public
router exports without duplicating route ownership.

## Module Responsibilities

| Module | Responsibility |
| --- | --- |
| `sdkwork-routes-catalog-app-api` | Thin Axum extraction, IAM scope projection, validation, DTO mapping, and response selection |
| `sdkwork-merchandise-web-support` | Shared Catalog store port, web DTOs, mapping, and SDKWork response helpers |
| `sdkwork-merchandise-repository-sqlx` | Tenant-scoped Postgres/SQLite queries and exact offset-page counts |
| `sdkwork-api-catalog-assembly` | Host-neutral executable route composition |
| `sdkwork-api-catalog-standalone-gateway` | Independent runtime, framework bootstrap, and IAM middleware |
| `sdkwork-catalog-app-sdk` | Owner-only OpenAPI generation and composed TypeScript facade |

## API Model

The canonical app resources are `catalog.categories`, `catalog.attributes`, `catalog.products`,
`catalog.products.skus`, `catalog.skus`, `cart.items`, and `addresses`. The app surface uses product
terminology; technical SPU aliases are not duplicated in public routes.

Every list handler validates `page` and `page_size` before repository execution. Repository queries
apply the same tenant, organization, status, and resource filters to both the bounded data query and
the exact count query. Response construction uses `sdkwork-utils-rust::http_api::PageInfo`.

## Security And Privacy

- Tenant and principal scope comes only from `IamAppContext`.
- App browse operations do not accept organization overrides and expose only active resources.
- Response DTOs omit tenant, organization, and owner-user identifiers.
- Address and cart mutations scope SQL predicates to the authenticated tenant and user.
- Errors are mapped to `ProblemDetail` without SQL or internal dependency details.

## SDK Generation

The authority document is `apis/app-api/catalog/catalog-app-api.openapi.json`. The generation
wrapper copies it into the SDK family and materializes local response references only in the derived
`.sdkgen.json` input required by the strict generator. Generated transport remains under
`generated/server-openapi`; consumers import the composed `@sdkwork/catalog-app-sdk` facade.

The family manifest declares `sdkOwner: sdkwork-catalog`,
`apiAuthority: sdkwork-catalog-app-api`, 16 owner-only operations, and no dependency-owned API copies.

## Verification

```powershell
cargo fmt -- --check
cargo test --workspace
pnpm sdk:generate
node ../sdkwork-specs/tools/check-api-operation-patterns.mjs --workspace .
node ../sdkwork-specs/tools/check-api-response-envelope.mjs --workspace .
node ../sdkwork-specs/tools/check-pagination.mjs --workspace .
node ../sdkwork-specs/tools/check-sdk-standard.mjs --workspace .
node ../sdkwork-specs/tools/check-route-path-collisions.mjs --root .
```
