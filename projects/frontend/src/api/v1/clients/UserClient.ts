import type { ApiResponse } from './BaseClient';
import { BaseClient } from './BaseClient';
import type { CredentialUpdatePayload } from '../CredentialUpdatePayload';
import type { SignInReponse } from '../SignInReponse';

export type AuthInfo = {
  username: string | null,
  emai: string | null
  password: string | null
}

export default class UserClient extends BaseClient {
  constructor(authHeader: Headers, baseUrl = '') {
    super(authHeader, baseUrl);
  }

  public async myInfo () {
    return await this.getOne<AuthInfo>('api/auth/v1/me');
  }

  public async updateMe (payload: CredentialUpdatePayload): Promise<ApiResponse<SignInReponse>> {
    const data = JSON.stringify(payload);
    return await this.put('api/v1/users/update/me', data)
  }
}

