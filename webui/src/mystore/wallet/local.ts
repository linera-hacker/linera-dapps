import { defineStore } from 'pinia'
import { doActionWithError } from '../action'
import { NotifyType } from '../notification'
import { CalSwapAmountResponse, GetBalanceResponse } from './types'
import { gql } from '@apollo/client'
import { constants } from 'src/const'

export const useWalletStore = defineStore('useWalletStore', {
  state: () => ({}),
  actions: {
    getBalance (erc20AppAddr: string, accountChainID: string, accountAddr: string, done?: (error: boolean, balance: string) => void) {
      const url = `${constants.swapEndPoint}/chains/${constants.swapCreationChainID}/applications/${erc20AppAddr}`
      const req = { query: `query{\n  balanceOf(owner:{\n    chain_id: "${accountChainID}"\n    owner: "User:${accountAddr}"\n  })\n}` }
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
      const url = `${constants.swapEndPoint}/chains/${constants.swapCreationChainID}/applications/${constants.swapAppID}`
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
    swapAmount (token0: string, token1: string, accountChainID: string, publicKey: string, accountAddr: string, outAmount: number) {
      const owner = `"User:${accountAddr}"`
      const mutate = gql`
        mutation swap ($token0: String!, $token1: String!, $outAmount: Float!, $chainID: String!, $owner: String!) {
          swap (
            token0: $token0,
            token1: $token1,
            amount0In: $outAmount,
            to:{
              chain_id: $chainID
              owner: $owner
            }
          )
        }
      `
      return new Promise((resolve, reject) => {
        window.linera.request({
          method: 'linera_graphqlMutation',
          params: {
            publicKey: publicKey,
            applicationId: constants.swapAppID,
            query: {
              query: mutate.loc?.source?.body,
              variables: {
                token0: token0,
                token1: token1,
                outAmount: outAmount,
                chainID: accountChainID,
                owner: owner
              }
            }
          }
        }).then((result) => {
          resolve(result)
        }).catch((e) => {
          reject(e)
        })
      })
    }
  }
})
