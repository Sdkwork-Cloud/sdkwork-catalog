import { HttpClient, createHttpClient } from './http/client';
import type { SdkworkAppConfig } from './types/common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';

import { CatalogApi, createCatalogApi } from './api/catalog';
import { CartApi, createCartApi } from './api/cart';
import { DeliveryApi, createDeliveryApi } from './api/delivery';

export class SdkworkAppClient {
  private httpClient: HttpClient;

  public readonly catalog: CatalogApi;
  public readonly cart: CartApi;
  public readonly delivery: DeliveryApi;

  constructor(config: SdkworkAppConfig) {
    this.httpClient = createHttpClient(config);
    this.catalog = createCatalogApi(this.httpClient);

    this.cart = createCartApi(this.httpClient);

    this.delivery = createDeliveryApi(this.httpClient);
  }
  setAuthToken(token: string): this {
    this.httpClient.setAuthToken(token);
    return this;
  }

  setAccessToken(token: string): this {
    this.httpClient.setAccessToken(token);
    return this;
  }

  setTokenManager(manager: AuthTokenManager): this {
    this.httpClient.setTokenManager(manager);
    return this;
  }

  get http(): HttpClient {
    return this.httpClient;
  }
}

export function createClient(config: SdkworkAppConfig): SdkworkAppClient {
  return new SdkworkAppClient(config);
}

export default SdkworkAppClient;
