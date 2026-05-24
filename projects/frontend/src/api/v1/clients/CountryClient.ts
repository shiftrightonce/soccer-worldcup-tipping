import type { Country } from 'src/api/Country';
import type { ApiResponse } from './BaseClient';
import { BaseClient } from './BaseClient';

export class CountryClient extends BaseClient {
  constructor(authHeader: Headers, baseUrl: string = '/api/v1/countries') {
    super(authHeader, baseUrl);
  }

  public async all (params?: URLSearchParams): Promise<ApiResponse<Country[]>> {
    return await this.getMany('all', params)
  }
}

export default (authHeader: Headers): CountryClient => {
  return new CountryClient(authHeader)
}
