import type { TipStrategy } from "src/api/TipStrategy";
import type { TipStrategyPayload } from "../TipStrategyPayload";
import type { ApiResponse } from "./BaseClient";
import { BaseClient } from "./BaseClient";
import type { StrategyResultPayload } from "../StrategyResultPayload";
import type { StrategyResult } from "src/api/StrategyResult";

export type TipStrategyResult = ApiResponse<TipStrategy>;

export class TipStrategyClient extends BaseClient {

  constructor(authHeader: Headers, tournamentId: string) {
    const baseUrl = `/api/v1/tip-strategies/${tournamentId}`
    super(authHeader, baseUrl)
  }

  public async all (params?: URLSearchParams): Promise<ApiResponse<TipStrategy[]>> {
    return await this.getMany('all', params)
  }

  public async open (params?: URLSearchParams): Promise<ApiResponse<TipStrategy[]>> {
    return await this.getMany('open', params)
  }

  public async closed (params?: URLSearchParams): Promise<ApiResponse<TipStrategy[]>> {
    return await this.getMany('closed', params)
  }

  public async byId (id: string, params?: URLSearchParams): Promise<TipStrategyResult> {
    return await this.getOne(id, params)
  }

  public async saveResults (id: string, payload: StrategyResultPayload, params?: URLSearchParams): Promise<ApiResponse<StrategyResult>> {
    const data = JSON.stringify(payload)
    return await this.post(`results/${id}`, data, params)
  }

  public async save (payload: TipStrategyPayload, id?: string,): Promise<TipStrategyResult> {
    const data = JSON.stringify(payload)
    if (id) {
      return await this.put(id, data)
    } else {
      return await this.post('', data)
    }
  }
}


export const makeNewPayload = (): TipStrategyPayload => ({
  tournamentId: '',
  label: '',
  description: '',
  gameId: null,
  groupId: null,
  opensAt: '',
  endsAt: '',
  calculatePointsOn: '',
  completed: false,
  strategyTypes: []
})

export const makeNewResultPayload = (): StrategyResultPayload => ({
  strategyResults: []
})

export const TipStrategyToPayload = (data: TipStrategy): TipStrategyPayload => {
  const payload = makeNewPayload()
  payload.calculatePointsOn = (new Date(data.calculatePointsOn as string)).toLocaleString()
  payload.endsAt = (new Date(data.endsAt as string)).toLocaleString();
  payload.description = data.description

  return payload;
}

export default (authHeader: Headers, tournamentId: string,): TipStrategyClient => {
  return new TipStrategyClient(authHeader, tournamentId)
}
