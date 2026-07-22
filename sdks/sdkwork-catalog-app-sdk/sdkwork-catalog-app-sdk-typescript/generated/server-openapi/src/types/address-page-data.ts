import type { Address } from './address';
import type { PageInfo } from './page-info';

export interface AddressPageData {
  items: Address[];
  pageInfo: PageInfo;
}
