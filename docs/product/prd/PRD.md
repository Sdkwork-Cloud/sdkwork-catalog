# Catalog (browse/open) PRD

Status: active
Owner: SDKWork maintainers
Application: catalog
Updated: 2026-06-24
Specs: REQUIREMENTS_SPEC.md, DOCUMENTATION_SPEC.md

## Document Map

- Commerce repository dissolution: `../sdkwork-specs/MIGRATION_SPEC.md` §8

## 1. Background And Problem

Public and integrator-facing catalog browse surfaces should not share the same ownership boundary as merchant admin merchandise mutations.

This repository is a **T1 commerce capability building block**. The `sdkwork-commerce` monolith has been dissolved; this repository is self-contained with its own domain logic, persistence, HTTP route builders, API server, and IAM middleware for the **catalog** capability.

## 2. Target Users

Storefront buyers, integrators, and read-only catalog consumers.

## 3. Goals And Non-Goals

### Goals

- Provide browse/open catalog HTTP routes separate from merchandise admin ownership.
- Reuse merchandise read models through explicit adapter boundaries.

### Non-Goals

- Admin catalog mutations (owned by `sdkwork-merchandise`).
- Owning SPU/SKU master write models in this repository long term.

## 4. Scope

- App browse/open catalog routes: categories, products, SPUs, SKUs, cart, addresses.
- Merchandise read stores consumed via `sdkwork-commerce-catalog-repository-sqlx` read adapter (`read_adapter.rs`).

Primary API prefixes:

- App: `/app/v3/api/catalog`

Migration status: **complete**.

## 5. User Scenarios

- A storefront lists published SPUs without exposing admin mutation endpoints.

## 6. Success Metrics

- Browse routes owned here with zero admin write endpoints after split.

## 7. Phases

- Phase 0 (complete): repository scaffold and standalone-gateway health.
- Phase 3 (complete): browse/open app routes owned by catalog app router; read adapter in catalog repository crate.

## 8. Linked Requirements

- Commerce repository dissolution: `../sdkwork-specs/MIGRATION_SPEC.md` §8
- Component contract: `specs/component.spec.json` (when present)
- Machine contracts: local `specs/`, future `apis/`, and generated `sdks/`

## 9. Open Questions


