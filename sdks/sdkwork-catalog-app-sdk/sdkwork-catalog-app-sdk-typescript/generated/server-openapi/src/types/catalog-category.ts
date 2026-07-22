export interface CatalogCategory {
  id: string;
  categoryNo: string;
  parentId?: string | null;
  path: string;
  levelNo: string;
  name: string;
  sortOrder: string;
  status: string;
  createdAt: string;
  updatedAt: string;
}
