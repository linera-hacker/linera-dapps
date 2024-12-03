
export enum API {
  GetTokens = 'http://172.16.31.58:30100/v1/get/tokens',
  GetTokenPairs = 'http://172.16.31.58:30100/v1/get/token/pairs',
  GetBalance = 'http://172.16.31.58:30100/v1/get/tokens',
  GetKPointTypes = 'http://172.16.31.58:30100/v1/get/kpoint/types',
}

export interface Token {
  ID: number;
  Address: string;
  Site: string;
  Icon: string;
  Name: string;
  Symbol: string;
  CreatedAt: number;
  UpdatedAt: number;
}

export interface GetTokensResponse {
  Infos: Token[];
  Total: number;
}

export interface TokenPair {
  ID: number;
  PoolID: number;
  TokenZeroID: number;
  TokenZeroName: string;
  TokenZeroAddress: string;
  TokenZeroIcon: string;
  TokenZeroSymbol: string;
  TokenOneID: number;
  TokenOneName: string;
  TokenOneAddress: string;
  TokenOneIcon: string;
  TokenOneSymbol: string;
  Remark: string;
  CreatedAt: number;
  UpdatedAt: number;
}

export interface GetTokenPairsResponse {
  Infos: TokenPair[];
  Total: number;
}

export interface Balance {
  balanceOf: string;
}

export interface BalanceResponse {
  data: Balance;
}

export interface CalSwapAmount {
  calculateSwapAmount: number;
}

export interface CalSwapAmountResponse {
  data: CalSwapAmount;
}

export interface KPointTypeInfo {
  KPointType: string
  ShortName: string
  Seconds: number
}

export interface GetKPointTypesResponse {
  Infos: KPointTypeInfo[]
}
