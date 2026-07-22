import type { CatalogSku } from './catalog-sku';
import type { PageInfo } from './page-info';

export interface CatalogSkuPageData {
  items: CatalogSku[];
  pageInfo: PageInfo;
}
