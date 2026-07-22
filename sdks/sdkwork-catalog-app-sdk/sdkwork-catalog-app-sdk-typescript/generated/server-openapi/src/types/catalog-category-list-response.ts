import type { CatalogCategoryPageData } from './catalog-category-page-data';

export interface CatalogCategoryListResponse {
  code: 0;
  data: unknown & CatalogCategoryPageData;
  traceId: string;
}
