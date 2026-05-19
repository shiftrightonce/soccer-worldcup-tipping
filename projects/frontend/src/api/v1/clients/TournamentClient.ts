import type { Tournament } from 'src/api/Tournament';
import type { ApiResponse, PaginateResult } from './BaseClient';
import { BaseClient } from './BaseClient';
import type { TournamentPayload } from '../TournamentPayload';

export type TournamentPaginateResult = PaginateResult<Tournament>;
export type TournamentResult = ApiResponse<Tournament>;

export class TournamentClient extends BaseClient {
  constructor(authHeader: Headers, baseUrl = '/api/v1/tournaments') {
    super(authHeader, baseUrl);
  }

  public async paginate (params?: URLSearchParams): Promise<TournamentPaginateResult> {
    return await this.fetchPage('', params);
  }

  public async save (payload: TournamentPayload, id = ''): Promise<ApiResponse<Tournament>> {
    const body = JSON.stringify(payload);
    if (id) {
      return await this.put(id, body);
    }

    return await this.post('', body);
  }

  public async getById (id: string): Promise<TournamentResult> {
    if (!id.trim()) {
      throw new Error('Tournament ID cannot be empty');
    }

    return await this.getOne(id);
  }
}

export const makeNewPayload = (): TournamentPayload => ({
  description: '',
  label: '',
  status: 'pending',
});

export default (authHeader: Headers): TournamentClient => {
  return new TournamentClient(authHeader);
};
