import type { User } from 'src/api/User';
import { BaseClient } from './BaseClient';

export default class UserClient extends BaseClient {
  constructor(authHeader: Headers, baseUrl = '') {
    super(authHeader, baseUrl);
  }

  public async myInfo() {
    return await this.getOne<User>('api/auth/v1/me');
  }
}
