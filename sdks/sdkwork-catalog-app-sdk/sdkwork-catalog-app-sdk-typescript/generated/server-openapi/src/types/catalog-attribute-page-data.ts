import type { CatalogAttribute } from './catalog-attribute';
import type { PageInfo } from './page-info';

export interface CatalogAttributePageData {
  items: CatalogAttribute[];
  pageInfo: PageInfo;
}
