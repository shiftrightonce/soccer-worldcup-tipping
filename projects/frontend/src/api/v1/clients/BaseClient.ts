import { LoadingBar } from "quasar";

export type ApiResponse<T> = {
  data: T | null;
  meta: {
    page?: {
      next: string;
      previous: string;
      total: number;
    } | null;
  };
  error: string | null;
};

export type PaginateResult<T> = ApiResponse<Array<T>>;

export class BaseClient {
  constructor(
    protected authHeader: Headers,
    protected baseUrl: string,
  ) { }

  protected async getOne<T> (
    endPoint: string,
    params: URLSearchParams | null = null,
  ): Promise<ApiResponse<T>> {
    return await fetch(this.buildUrl(endPoint, params), {
      headers: this.authHeader,
    })
      .then((response) => {
        return response.json();
      })
      .catch((e) => {
        throw e;
      });
  }

  protected async getMany<T> (
    endPoint: string,
    params: URLSearchParams | null = null,
  ): Promise<ApiResponse<T[]>> {
    LoadingBar.start()
    return await fetch(this.buildUrl(endPoint, params), {
      headers: this.authHeader,
    })
      .then((response) => {
        LoadingBar.stop()
        return response.json();
      })
      .catch((e) => {
        LoadingBar.stop()
        throw e;
      });
  }

  protected async fetchPage<T> (
    endPoint: string,
    params: URLSearchParams | null = null,
  ): Promise<PaginateResult<T>> {
    const response = await fetch(this.buildUrl(endPoint, params), {
      headers: this.authHeader,
    });

    return await response.json();
  }

  protected async post<T> (endPoint: string, payload: string, params: URLSearchParams | null = null): Promise<ApiResponse<T>> {
    LoadingBar.start()
    const response = await fetch(this.buildUrl(endPoint, params), {
      method: 'POST',
      headers: this.authHeader,
      body: payload,
    });

    LoadingBar.stop()
    return await response.json();
  }

  protected async put<T> (endPoint: string, payload: string, params: URLSearchParams | null = null): Promise<ApiResponse<T>> {
    LoadingBar.start()
    const response = await fetch(this.buildUrl(endPoint, params), {
      method: 'PUT',
      headers: this.authHeader,
      body: payload,
    });
    LoadingBar.stop()
    return await response.json();
  }

  protected buildUrl (endPoint: string, params: URLSearchParams | null = null) {
    const full = this.baseUrl + `${endPoint.trim() ? '/' + endPoint : ''}?` + (params || '').toString();
    return full;
  }
}
