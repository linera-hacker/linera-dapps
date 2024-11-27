export interface Balance {
  balanceOf: string;
}

export interface GetBalanceResponse {
  data: Balance;
}

export interface CalSwapAmount {
  calculateSwapAmount: number;
}

export interface CalSwapAmountResponse {
  data: CalSwapAmount;
}

export interface SwapAmountResponse {
  data: CalSwapAmount;
}
