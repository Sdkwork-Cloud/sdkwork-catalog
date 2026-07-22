import type { AddressData } from './address-data';

export interface AddressResponse {
  code: 0;
  data: unknown & AddressData;
  traceId: string;
}
