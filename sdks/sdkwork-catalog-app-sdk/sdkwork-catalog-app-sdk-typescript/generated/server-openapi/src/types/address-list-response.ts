import type { AddressPageData } from './address-page-data';

export interface AddressListResponse {
  code: 0;
  data: unknown & AddressPageData;
  traceId: string;
}
