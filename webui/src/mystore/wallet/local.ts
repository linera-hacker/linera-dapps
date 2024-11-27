import { defineStore } from 'pinia'
import { doActionWithError } from '../action'
import { NotifyType } from '../notification'
import { CalSwapAmountResponse, GetBalanceResponse, SwapAmountResponse } from './types'

export const useWalletStore = defineStore('useWalletStore', {
  state: () => ({
    ServerAddr: 'http://172.16.31.51:31130',
    ChainID: 'a64ce4eded0d5fed622bf4b6e7a92eaf627ca31a654fa83e63b93406478d46bb',
    SwapAppID: '5c2b4f09e319eee343b4fbf87909a4d0bda53b8806188179a63edae8c7f77991298371139571e8b133d653159aad8acab2acb430eefd87812ff5db6aaeebeced8eeb552ddc42dc3fce31f25f6f68a5b3c2e72e252a82b163f4f3ca86fd228188010000000000000000000000'
  }),
  actions: {
    getBalance (erc20AppAddr: string, accountAddr: string, done?: (error: boolean, balance: string) => void) {
      const url = `${this.ServerAddr}/chains/${this.ChainID}/applications/${erc20AppAddr}`
      const req = { query: `query{\n  balanceOf(owner:{\n    chain_id: "${this.ChainID}"\n    owner: "User:${accountAddr}"\n  })\n}` }
      doActionWithError<unknown, GetBalanceResponse>(
        url,
        req,
        {
          Error: {
            Title: 'get balance',
            Message: 'failed to get balance',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetBalanceResponse): void => {
          done?.(false, resp.data.balanceOf)
        },
        () => {
          done?.(true, '0')
        }
      )
    },
    calSwapAmount (tokenZeroAppAddr: string, tokenOneAppAddr: string, outAmount: number, done?: (error: boolean, amount: number) => void) {
      const url = `${this.ServerAddr}/chains/${this.ChainID}/applications/${this.SwapAppID}`
      const req = { query: `query{\n  calculateSwapAmount(token0:"${tokenZeroAppAddr}",token1:"${tokenOneAppAddr}",amount1:"${outAmount}")\n}` }
      doActionWithError<unknown, CalSwapAmountResponse>(
        url,
        req,
        {
          Error: {
            Title: 'calculate swap amount',
            Message: 'failed to calculate swap amount',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: CalSwapAmountResponse): void => {
          done?.(false, resp.data.calculateSwapAmount)
        },
        () => { done?.(true, 0) }
      )
    },
    swapAmount (tokenZeroAddr: string, tokenOneAddr: string, chainID: string, accountAddr: string, outAmount: number, done?: (error: boolean, inAmount: number) => void) {
      const url = `${this.ServerAddr}/chains/${this.ChainID}/applications/${this.SwapAppID}`
      const req = {
        query: `mutation {
          swap (
          token0:"${tokenZeroAddr}",
          token1:"${tokenOneAddr}",
          amount0In:"${outAmount}"),
          to:{
            chain_id: "${chainID}"
            owner: "User:${accountAddr}"
          }
        }`
      }
      doActionWithError<unknown, SwapAmountResponse>(
        url,
        req,
        {
          Error: {
            Title: 'swap amount',
            Message: 'failed to swap amount',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp) => {
          done?.(false, resp.data.calculateSwapAmount)
        },
        () => {
          done?.(true, 0)
        }
      )
    }
  }
})
