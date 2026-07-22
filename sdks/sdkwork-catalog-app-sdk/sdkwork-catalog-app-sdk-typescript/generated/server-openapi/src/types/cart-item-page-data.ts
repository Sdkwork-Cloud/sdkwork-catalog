import type { CartItem } from './cart-item';
import type { PageInfo } from './page-info';

export interface CartItemPageData {
  items: CartItem[];
  pageInfo: PageInfo;
}
