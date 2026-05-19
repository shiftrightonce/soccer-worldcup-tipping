import { BaseClient } from './BaseClient';

export class CountryClient extends BaseClient {
  constructor(authHeader: Headers, baseUrl: string = '/api/v1/countries') {
    super(authHeader, baseUrl);
  }
}
