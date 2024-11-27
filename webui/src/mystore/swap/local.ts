import { defineStore } from 'pinia'
import { API, Token, GetTokensResponse, TokenPair, GetTokenPairsResponse, KPointTypeInfo, GetKPointTypesResponse } from './types'
import { doActionWithError } from '../action'
import { NotifyType } from '../notification'

export const useSwapStore = defineStore('useSwapStore', {
  state: () => ({
    IsInitilazed: false,
    SelectedToken: { ID: -1 } as Token,
    SelectedTokenPair: { ID: -1 } as TokenPair,
    Tokens: [] as Token[],
    TokenPairs: [] as TokenPair[],
    KPointTypes: [] as KPointTypeInfo[]
  }),
  actions: {
    getTokens (done?: (error: boolean, rows: Token[]) => void) {
      doActionWithError<unknown, GetTokensResponse>(
        API.GetTokens,
        {},
        {
          Error: {
            Title: 'get tokens',
            Message: 'failed to get tokens',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetTokensResponse): void => {
          resp.Infos.sort()
          resp.Infos.forEach((info) => {
            info.Balance = 0
          })
          this.Tokens = resp.Infos
          this.SelectedToken = resp.Infos[0]
          this.SelectedTokenPair.ID = -1
          done?.(false, resp.Infos)
        }, () => {
          done?.(true, [])
        }
      )
    },
    getTokenPairsByTokenZeroID (done?: (error: boolean, rows: TokenPair[]) => void) {
      if (this.SelectedToken === null) {
        return
      }
      doActionWithError<unknown, GetTokenPairsResponse>(
        API.GetTokenPairs,
        { Conds: { TokenZeroID: { Op: 'eq', Value: this.SelectedToken.ID } } },
        {
          Error: {
            Title: 'get token pairs',
            Message: 'failed to get token pairs',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetTokenPairsResponse): void => {
          resp.Infos.sort()
          resp.Infos.forEach((info) => {
            info.TokenOneBalance = 0
          })
          this.TokenPairs = resp.Infos
          if (resp.Infos.length === 0) {
            this.SelectedTokenPair = { ID: -1 } as TokenPair
          } else {
            this.SelectedTokenPair = resp.Infos[0]
          }
          done?.(false, resp.Infos)
        }, () => {
          done?.(true, [])
        }
      )
    },
    getKPointTypes (done?: (error: boolean, infos: KPointTypeInfo[]) => void) {
      doActionWithError<unknown, GetKPointTypesResponse>(
        API.GetKPointTypes,
        {},
        {
          Error: {
            Title: 'get kpoint types',
            Message: 'failed to get kpoint types',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp) => {
          this.KPointTypes = resp.Infos
          done?.(false, resp.Infos)
        },
        () => { done?.(true, []) }
      )
    }
  }
})
