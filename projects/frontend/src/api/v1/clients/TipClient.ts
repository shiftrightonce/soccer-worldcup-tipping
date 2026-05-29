import type { Tip } from "src/api/Tip";
import type { TipPayload } from "../TipPayload"
import type { ApiResponse } from "./BaseClient";
import { BaseClient } from "./BaseClient"

export class TipClient extends BaseClient {

  constructor(authHeader: Headers, tournamentId: string) {
    const baseUrl = `/api/v1/tips/${tournamentId}`
    super(authHeader, baseUrl)
  }

  public async myTips (params?: URLSearchParams): Promise<ApiResponse<TipPayload[]>> {
    return await this.getMany<TipPayload>('my-tips', params)
  }

  public async saveMyTip (payload: TipPayload, id?: string): Promise<ApiResponse<Tip>> {
    const body = JSON.stringify(payload)
    let response: ApiResponse<Tip>;
    if (id) {
      response = await this.put<Tip>(`my-tips/${id}`, body)
    } else {
      response = await this.post<Tip>('my-tips', body)
    }
    return response

  }

}

export const makeNewPayload = (tournamentId: string, tipStrategyId: string): TipPayload => {
  return {
    id: null,
    tournamentId,
    tipStrategyId,
    userId: null,
    strategies: []
  }
}


export default (authHeader: Headers, tournamentId: string): TipClient => {
  return new TipClient(authHeader, tournamentId)
}
