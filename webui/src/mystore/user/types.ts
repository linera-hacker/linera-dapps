
export enum API {
  GetLastTranscation = 'http://172.16.31.58:30100/v1/get/token/last/cond',
  ExistToken = 'http://172.16.31.58:30100/v1/exit/token/by/symbol'
}

export interface LastTranscation {
  PoolID: number;
  TokenZeroAddress: string;
  TokenOneAddress: string;
  LastTxAt: number;
  LastTxZeroAmount: string;
  LastTxOneAmount: string;
  OneDayZeroAmountVolumn: string;
  OneDayOneAmountVolumn: string;
  NowPrice: string;
  OneDayIncresePercent: string;
}

export interface GetLastTranscationRequest {
  PoolID: number;
  TokenZeroAddress: string;
  TokenOneAddress: string;
}

export interface GetLastTranscationResponse {
  Info: LastTranscation;
}

export interface ExistTokenRequest {
  Symbol: string;
}

export interface ExistTokenResponse {
  Exist: boolean;
}
