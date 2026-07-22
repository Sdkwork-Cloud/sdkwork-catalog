import type { CatalogProduct } from './catalog-product';
import type { PageInfo } from './page-info';

export interface CatalogProductPageData {
  items: CatalogProduct[];
  pageInfo: PageInfo;
}
