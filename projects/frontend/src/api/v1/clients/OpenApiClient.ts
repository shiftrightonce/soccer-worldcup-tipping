import type { SignInPayload } from "../SignInPayload";
import type { SignInReponse } from "../SignInReponse";
import type { SignupPayload } from "../SignupPayload";
import type { ApiResponse } from "./BaseClient";
import { BaseClient } from "./BaseClient";

export class OpenApiClient extends BaseClient {
  constructor(baseUrl = '') {
    super(
      new Headers([
        ['Accept', 'application/json'],
        ['Content-Type', 'application/json'],
      ])
      , baseUrl)
  }

  public async signup (payload: SignupPayload): Promise<ApiResponse<SignInReponse>> {
    const data = JSON.stringify(payload);
    return await this.post('signup', data)
  }

  public async login (username: string, password: string): Promise<ApiResponse<SignInReponse>> {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    const payload: SignInPayload = {
      username: null,
      email: null,
      password
    };
    if (emailRegex.test(username.trim())) {
      payload.email = username;
    } else {
      payload.username = username;
    }

    return await this.post('login', JSON.stringify(payload))
  }

}

export const makeNewSignupPayload = () => ({
  username: '',
  password: '',
  email: '',
  confirmPassword: ''
});

export default (endpoint = "_open/v1"): OpenApiClient => {
  return new OpenApiClient(endpoint)
}
