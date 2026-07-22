export interface CatalogProduct {
  id: string;
  spuNo: string;
  title: string;
  subtitle?: string | null;
  description?: string | null;
  productType: string;
  categoryId?: string | null;
  status: string;
  publishedAt?: string | null;
  visibleSurfaces: string;
  createdAt: string;
  updatedAt: string;
}
