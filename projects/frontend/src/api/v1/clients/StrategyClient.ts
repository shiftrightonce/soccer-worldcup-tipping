import type { TipStrategy } from "src/api/TipStrategy";
import type { TipStrategyPayload } from "../TipStrategyPayload";
import type { ApiResponse } from "./BaseClient";
import { BaseClient } from "./BaseClient";

export type TipStrategyResult = ApiResponse<TipStrategy>;

export class TipStrategyClient extends BaseClient {

  constructor(authHeader: Headers, private tournamentId: string) {
    const baseUrl = `/api/v1/strategies/${tournamentId}`
    super(authHeader, baseUrl)
  }

  public async all (params?: URLSearchParams): Promise<ApiResponse<TipStrategy[]>> {
    return await this.getMany('all', params)
  }

  public async byId (id: string): Promise<TipStrategyResult> {
    return await this.getOne(id)
  }
}


export const makeNewPayload = (): TipStrategyPayload => ({
  tournament_id: '',
  label: '',
  description: '',
  game_id: '',
  opens_at: '',
  ends_at: '',
  calculate_points_on: '',
  completed: false,
  strategy_types: []
})

export default (authHeader: Headers, tournamentId: string,): TipStrategyClient => {
  return new TipStrategyClient(authHeader, tournamentId)
}
