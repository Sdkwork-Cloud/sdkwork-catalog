import type { CatalogSkuData } from './catalog-sku-data';

export interface CatalogSkuResponse {
  code: 0;
  data: unknown & CatalogSkuData;
  traceId: string;
}
