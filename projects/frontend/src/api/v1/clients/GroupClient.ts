import type { Group } from "src/api/Group";
import type { ApiResponse } from "./BaseClient";
import { BaseClient } from "./BaseClient";

export class GroupClient extends BaseClient {

  constructor(authHeader: Headers) {
    const baseUrl = `/api/v1/groups`
    super(authHeader, baseUrl)
  }

  public async all (params?: URLSearchParams): Promise<ApiResponse<Group[]>> {
    return await this.getMany('', params)
  }
}

export default (authHeader: Headers): GroupClient => {
  return new GroupClient(authHeader)
}
