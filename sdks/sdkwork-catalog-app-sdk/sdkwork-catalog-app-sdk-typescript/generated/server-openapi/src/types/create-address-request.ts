export interface CreateAddressRequest {
  receiverName: string;
  receiverPhone: string;
  countryCode: string;
  province: string;
  city: string;
  detailAddress: string;
  isDefault?: boolean;
}
