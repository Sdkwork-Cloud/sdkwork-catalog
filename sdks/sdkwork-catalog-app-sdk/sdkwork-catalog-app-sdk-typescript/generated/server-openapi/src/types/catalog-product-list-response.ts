import type { CatalogProductPageData } from './catalog-product-page-data';

export interface CatalogProductListResponse {
  code: 0;
  data: unknown & CatalogProductPageData;
  traceId: string;
}
