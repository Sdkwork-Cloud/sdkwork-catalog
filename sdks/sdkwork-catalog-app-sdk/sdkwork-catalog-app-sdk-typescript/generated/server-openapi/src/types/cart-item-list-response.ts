import type { CartItemPageData } from './cart-item-page-data';

export interface CartItemListResponse {
  code: 0;
  data: unknown & CartItemPageData;
  traceId: string;
}
