import type { CatalogCategory } from './catalog-category';
import type { PageInfo } from './page-info';

export interface CatalogCategoryPageData {
  items: CatalogCategory[];
  pageInfo: PageInfo;
}
