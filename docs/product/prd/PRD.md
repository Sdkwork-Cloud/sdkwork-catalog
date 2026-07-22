# Catalog Product Requirements

Status: active
Owner: SDKWork maintainers
Application: catalog
Updated: 2026-07-22
Specs: REQUIREMENTS_SPEC.md, API_SPEC.md, PAGINATION_SPEC.md, SDK_SPEC.md

## Purpose

Catalog provides the user-facing commerce discovery boundary used by SDKWork applications. Buyers
can browse active products and resolve a product to a purchasable SKU without receiving backend
administration operations or internal tenant and principal identifiers.

## Users

- Storefront buyers browsing active categories and products
- Application services resolving product IDs to typed SKU IDs before checkout
- Authenticated users managing cart items and delivery addresses

## Owned Scope

- Active category and attribute lists
- Active product list and product retrieval
- Active SKU retrieval and `catalog.products.skus.list`
- Authenticated cart item list/create/update/delete
- Authenticated delivery address list/create/update/delete/default selection
- Owner-only `sdkwork-catalog-app-api` OpenAPI and `sdkwork-catalog-app-sdk`

The app API does not expose SPU aliases, unimplemented price-list routes, merchandise backend
mutations, shop operations, order operations, or after-sales operations.

## Contract Requirements

- Every list uses `page` and `page_size` with a default of 20 and maximum of 200.
- SQL applies tenant/data-scope filters, stable ordering, `LIMIT`, and `OFFSET`; `COUNT(*)` supplies
  exact offset-page totals.
- App browse operations are restricted to the IAM context organization and active resources.
- Success responses use the SDKWork v3 envelope; errors use `ProblemDetail`.
- Generated SDK methods expose concrete product, SKU, cart, address, and page types.
- Checkout consumers use SKU IDs returned by `catalog.products.skus.list`; product IDs are never
  treated as SKU IDs.

## Success Criteria

- Rust routes and authority OpenAPI contain the same executable operation set.
- Catalog App SDK generation is owner-only and idempotent.
- No raw HTTP, generic response probing, duplicate product/SPU app routes, or historical SDK types
  remain in the application integration path.
- API, response-envelope, pagination, SDK, Cargo, and route-collision verification pass.
