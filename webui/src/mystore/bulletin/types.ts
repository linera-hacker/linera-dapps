import { constants } from 'src/const'

export enum API {
  GetOneDayVolumn = `${constants.klineEndpoint}/v1/get/one/day/volumn`,
}

export interface TokenVolumn {
  PoolID: number
  Address: string
  Name: string
  Icon: string
  Symbol: string
  Amount: number
}

export interface GetOneDayVolumnResponse {
  Infos: TokenVolumn[];
}
