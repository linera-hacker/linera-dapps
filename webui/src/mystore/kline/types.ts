import { constants } from 'src/const'

export enum API {
  GetKPointsForLine = `${constants.klineEndpoint}/v1/get/kpoints/for/line`,
  GetTransactionsForLine = `${constants.klineEndpoint}/v1/get/transactions/for/line`,
}

export interface KPoint {
  Nums: number[];
  Times: number[];
  FormatTimes: string[];
}

export interface EchartKPoints {
  CategoryItems: string[];
  Nums: number[][];
}

export interface GetKPointsForLineResponse {
  KPointType: string;
  KPoints: KPoint[];
  Limit: number;
  Offset: number;
  OriginalTime: number;
  TokenPairID: number;
  Total: number;
}

export interface Transaction {
  ID: number;
  PoolID: number;
  TransactionID: number;
  TransactionType: string;
  Owner: string;
  AmountZeroIn: number;
  AmountOneIn: number;
  AmountZeroOut: number;
  AmountOneOut: number;
  Timestamp: number;
}

export interface GetTransactionsForLineResponse {
  OriginalTxID: number;
  PoolID: number;
  Transactions: Transaction[];
  Limit: number;
  Offset: number;
  Total: number;
}
