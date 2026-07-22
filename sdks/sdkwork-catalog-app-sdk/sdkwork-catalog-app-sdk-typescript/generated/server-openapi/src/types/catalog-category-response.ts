import type { CatalogCategoryData } from './catalog-category-data';

export interface CatalogCategoryResponse {
  code: 0;
  data: unknown & CatalogCategoryData;
  traceId: string;
}
