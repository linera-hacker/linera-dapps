import { constants } from 'src/const'

export enum API {
  GetTransactionsForLine = `${constants.klineEndpoint}/v1/get/transactions/for/line`,
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
