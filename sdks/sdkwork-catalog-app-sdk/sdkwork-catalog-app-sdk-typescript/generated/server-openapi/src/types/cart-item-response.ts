import type { CartItemData } from './cart-item-data';

export interface CartItemResponse {
  code: 0;
  data: unknown & CartItemData;
  traceId: string;
}
