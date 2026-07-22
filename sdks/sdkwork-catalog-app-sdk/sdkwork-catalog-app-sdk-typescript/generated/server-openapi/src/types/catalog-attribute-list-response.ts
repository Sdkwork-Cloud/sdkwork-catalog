import type { CatalogAttributePageData } from './catalog-attribute-page-data';

export interface CatalogAttributeListResponse {
  code: 0;
  data: unknown & CatalogAttributePageData;
  traceId: string;
}
