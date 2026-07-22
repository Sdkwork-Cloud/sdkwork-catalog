export interface CatalogSku {
  id: string;
  spuId: string;
  skuNo: string;
  name: string;
  title: string;
  priceAmount: string;
  originalPriceAmount?: string | null;
  currencyCode: string;
  fulfillmentType: string;
  inventoryTracking: string;
  status: string;
  publishedAt?: string | null;
  specJson?: string | null;
  createdAt: string;
  updatedAt: string;
}
