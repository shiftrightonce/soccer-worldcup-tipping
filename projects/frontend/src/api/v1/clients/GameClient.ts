import type { Game } from "src/api/Game";
import type { ApiResponse } from "./BaseClient";
import { BaseClient } from "./BaseClient";
import type { GameStatus } from "src/api/GameStatus";
import type { GamePayload } from "../GamePayload";


export type GameResult = ApiResponse<Game>;

export class GameClient extends BaseClient {

  constructor(authHeader: Headers, tournamentId: string) {
    const baseUrl = `/api/v1/games/${tournamentId}`
    super(authHeader, baseUrl)
  }

  public async all (params?: URLSearchParams): Promise<ApiResponse<Game[]>> {
    return await this.getMany('all', params)
  }

  public async byStatus (status: GameStatus, params?: URLSearchParams): Promise<ApiResponse<Game[]>> {
    return await this.getMany(`by-status/${status}`, params)
  }

  public async save (payload: GamePayload, id = ''): Promise<GameResult> {
    payload.count = Number(payload.count)
    payload.countryAGoals = Number(payload.countryAGoals)
    payload.countryBGoals = Number(payload.countryBGoals)
    payload.countryAPenaltyGoals = Number(payload.countryAPenaltyGoals)
    payload.countryBPenaltyGoals = Number(payload.countryBPenaltyGoals)
    payload.label = payload.label || `Game ${payload.count}`
    const body = JSON.stringify(payload);
    if (id) {
      return await this.put(id, body)
    }

    return await this.post('', body)
  }

  public async byId (id: string): Promise<GameResult> {
    return await this.getOne(id)
  }

}


export const makeNewPayload = (tournamentId: string): GamePayload => {
  return {
    tournamentId,
    label: '',
    count: 0,
    stage: null,
    status: 'pending',
    countryAId: null,
    countryBId: null,
    penalty: false,
    countryAGoals: 0,
    countryBGoals: 0,
    countryAPenaltyGoals: 0,
    countryBPenaltyGoals: 0,
    toConfigureOn: null,
    winnerId: null,
  }
}

export default (authHeader: Headers, tournamentId: string): GameClient => {
  return new GameClient(authHeader, tournamentId)
}
