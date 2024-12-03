import { defineStore } from 'pinia'
import { API, LastTranscation, GetLastTranscationRequest, GetLastTranscationResponse, GetLastTranscationsRequest, GetLastTranscationsResponse, ExistTokenRequest, ExistTokenResponse } from './types'
import { doActionWithError } from '../action'
import { NotifyType } from '../notification'

export const useUserStore = defineStore('user', {
  state: () => ({
    account: undefined as unknown as string,
    chainId: undefined as unknown as string,
    username: undefined as unknown as string,
    accountBalance: '0.',
    chainBalance: '0.'
  }),
  getters: {},
  actions: {
    getLastTranscation (req: GetLastTranscationRequest, done?: (errors: boolean, row: LastTranscation) => void) {
      doActionWithError<unknown, GetLastTranscationResponse>(
        API.GetLastTranscation,
        req,
        {
          Error: {
            Title: 'get lastTranscation',
            Message: 'failed to get lastTranscation',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetLastTranscationResponse): void => {
          done?.(false, resp.Info)
        }, () => {
          done?.(true, {} as LastTranscation)
        }
      )
    },
    getLastTranscations (req: GetLastTranscationsRequest, done?: (errors: boolean, rows: LastTranscation[]) => void) {
      doActionWithError<unknown, GetLastTranscationsResponse>(
        API.GetLastTranscations,
        req,
        {
          Error: {
            Title: 'get lastTranscations',
            Message: 'failed to get lastTranscations',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetLastTranscationsResponse): void => {
          done?.(false, resp.Infos)
        }, () => {
          done?.(true, [])
        }
      )
    },
    existToken (req: ExistTokenRequest, done?: (errors: boolean, row: boolean) => void) {
      doActionWithError<unknown, ExistTokenResponse>(
        API.ExistToken,
        req,
        {
          Error: {
            Title: 'check exist token',
            Message: 'failed to check exist token',
            Description: 'please retry',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: ExistTokenResponse): void => {
          done?.(false, resp.Exist)
        }, () => {
          done?.(true, true)
        }
      )
    }
  }
})
