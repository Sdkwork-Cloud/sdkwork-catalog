import type { CatalogProductData } from './catalog-product-data';

export interface CatalogProductResponse {
  code: 0;
  data: unknown & CatalogProductData;
  traceId: string;
}
