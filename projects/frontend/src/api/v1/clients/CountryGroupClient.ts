import type { CountryGroup } from "src/api/CountryGroup";
import type { ApiResponse, PaginateResult } from "./BaseClient";
import { BaseClient } from "./BaseClient";
import type { CountryGroupPayload } from "../CountryGroupPayload";

export type CountryGroupPaginateResult = PaginateResult<CountryGroup>;

export class CountryGroupClient extends BaseClient {

  constructor(authHeader: Headers, tournamentId: string) {
    const baseUrl = `/api/v1/country-groups/${tournamentId}`
    super(authHeader, baseUrl)
  }

  public async paginate (params: URLSearchParams): Promise<CountryGroupPaginateResult> {
    return await this.fetchPage('', params)
  }

  public async all (params: URLSearchParams): Promise<ApiResponse<CountryGroup[]>> {
    return await this.getMany('all', params)
  }

  public async byId (id: string, params?: URLSearchParams): Promise<ApiResponse<CountryGroup>> {
    return await this.getOne(id, params)
  }

  public async byGroupId (groupId: string, params?: URLSearchParams): Promise<ApiResponse<CountryGroup[]>> {
    return await this.getMany(`by-group/${groupId}`, params)
  }


  public async save (payload: CountryGroupPayload, id = ''): Promise<ApiResponse<CountryGroup>> {
    const body = JSON.stringify(payload)
    if (id) {
      return await this.put(id, body)
    }

    return await this.post('', body)
  }
}

export const makeNewPayload = (): CountryGroupPayload => ({
  tournamentId: null,
  countryId: null,
  groupId: null,
  isOut: false,
  points: 0
})

export default (authHeader: Headers, tournamentId: string): CountryGroupClient => {
  return new CountryGroupClient(authHeader, tournamentId)
}
