import type { CatalogSkuPageData } from './catalog-sku-page-data';

export interface CatalogSkuListResponse {
  code: 0;
  data: unknown & CatalogSkuPageData;
  traceId: string;
}
