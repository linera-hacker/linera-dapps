import { defineStore } from 'pinia'
import { API, KPoint, GetKPointsForLineResponse, EchartKPoints, Transaction, GetTransactionsForLineResponse } from './types'
import { doActionWithError } from '../action'
import { NotifyType } from '../notification'

export const useKLineStore = defineStore('useKLineStore', {
  state: () => ({
    NeedInitKLine: true,
    NeedInitTxTable: true,
    SelectedKPType: null as string | null,
    SelectedTokenPairID: null as number | null,
    SelectedPoolID: null as number | null,
    OriginalTime: 0 as number,
    OriginalTxID: 0 as number,
    EchartPoinsData: {
      CategoryItems: [],
      Nums: []
    } as EchartKPoints,
    Transactions: [] as Transaction[],
    RefreshKlineHistoryLock: false,
    RefreshTableHistoryLock: false,
    EchartPoinsDataUpdate: 0,
    MaxKPoints: 1000,
    MaxTransactions: 1000,
    ResetKLineViewLock: 0,
    ResetTableViewLock: 0
  }),
  actions: {
    refreshNewKPoints (done?: (error: boolean, rows: KPoint[]) => void) {
      if (this.SelectedKPType === null || this.SelectedTokenPairID === null || this.SelectedTokenPairID < 0) {
        return
      }
      doActionWithError<unknown, GetKPointsForLineResponse>(
        API.GetKPointsForLine,
        {
          KPointType: this.SelectedKPType,
          Limit: 20,
          Offset: 0,
          OriginalTime: this.OriginalTime,
          TokenPairID: this.SelectedTokenPairID
        },
        {
          Error: {
            Title: 'Get Kpoints',
            Message: 'Failed to get kpoints',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetKPointsForLineResponse): void => {
          if (this.OriginalTime === 0) {
            this.OriginalTime = resp.OriginalTime
          }
          if (resp.KPoints.length > 0) {
            this.OriginalTime = resp.KPoints[resp.KPoints.length - 1].Times[1] + 1
            this._refreshEchartKPoints(resp.KPoints, false)
          }
          if (this.EchartPoinsData.CategoryItems.length > this.MaxKPoints) {
            this.EchartPoinsData.CategoryItems.splice(0, this.EchartPoinsData.CategoryItems.length - this.MaxKPoints)
            this.EchartPoinsData.Nums.splice(0, this.EchartPoinsData.Nums.length - this.MaxKPoints)
          }

          done?.(false, resp.KPoints)
        }, () => {
          done?.(true, [])
        }
      )
    },
    refreshHistoryKPoints (done?: (error: boolean, rows: KPoint[]) => void) {
      if (this.SelectedKPType === null || this.SelectedTokenPairID === null || this.SelectedTokenPairID < 0) {
        return
      }
      if (this.EchartPoinsData.CategoryItems.length >= this.MaxKPoints) {
        return
      }
      if (this.RefreshKlineHistoryLock) {
        return
      }
      this.RefreshKlineHistoryLock = true
      doActionWithError<unknown, GetKPointsForLineResponse>(
        API.GetKPointsForLine,
        {
          KPointType: this.SelectedKPType,
          Limit: -100,
          Offset: -this.EchartPoinsData.CategoryItems.length,
          OriginalTime: this.OriginalTime,
          TokenPairID: this.SelectedTokenPairID
        },
        {
          Error: {
            Title: 'Get Kpoints Histories',
            Message: 'Failed to get kpoints histories',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetKPointsForLineResponse): void => {
          if (resp.KPoints.length > 0) {
            this._refreshEchartKPoints(resp.KPoints, true)
          }
          if (this.OriginalTime === 0 && resp.KPoints.length > 0) {
            this.OriginalTime = resp.KPoints[0].Times[1] + 1
          }

          done?.(false, resp.KPoints)
        }, () => {
          done?.(true, [])
        }
      )

      setTimeout(() => {
        this.RefreshKlineHistoryLock = false
      }, 300)
    },
    _refreshEchartKPoints (infos: KPoint[], unshift: boolean) {
      for (let i = 0; i < infos.length; i++) {
        if (unshift) {
          this.EchartPoinsData.CategoryItems.unshift(infos[i].FormatTimes[0])
          this.EchartPoinsData.Nums.unshift(infos[i].Nums)
        } else {
          this.EchartPoinsData.CategoryItems.push(infos[i].FormatTimes[0])
          this.EchartPoinsData.Nums.push(infos[i].Nums)
        }
      }
    },
    refreshNewTransactions (done?: (error: boolean, rows: Transaction[]) => void) {
      if (this.SelectedPoolID === null || this.SelectedPoolID < 0) {
        return
      }
      doActionWithError<unknown, GetTransactionsForLineResponse>(
        API.GetTransactionsForLine,
        {
          Limit: 20,
          Offset: 0,
          OriginalTxID: this.OriginalTxID,
          PoolID: this.SelectedPoolID
        },
        {
          Error: {
            Title: 'Get Transactions',
            Message: 'Failed to fetch transactions',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetTransactionsForLineResponse): void => {
          if (this.OriginalTxID === 0) {
            this.OriginalTxID = resp.OriginalTxID
          }
          this._refreshTableTransactions(resp.Transactions, true)
          if (this.Transactions.length > this.MaxTransactions) {
            this.Transactions.splice(0, this.Transactions.length - this.MaxTransactions)
          }

          done?.(false, resp.Transactions)
        }, () => {
          done?.(true, [])
        }
      )
    },
    refreshHistoryTransactions (done?: (error: boolean, rows: Transaction[]) => void) {
      if (this.SelectedPoolID === null || this.SelectedPoolID < 0) {
        return
      }
      if (this.Transactions.length >= this.MaxTransactions) {
        return
      }
      if (this.RefreshTableHistoryLock) {
        return
      }
      this.RefreshTableHistoryLock = true
      doActionWithError<unknown, GetTransactionsForLineResponse>(
        API.GetTransactionsForLine,
        {
          Limit: -20,
          Offset: -this.Transactions.length,
          OriginalTxID: this.OriginalTxID,
          PoolID: this.SelectedPoolID
        },
        {
          Error: {
            Title: 'Get Transaction Histories',
            Message: 'Failed to get transactions histories',
            Popup: true,
            Type: NotifyType.Error
          }
        },
        (resp: GetTransactionsForLineResponse): void => {
          if (this.OriginalTxID === 0) {
            this.OriginalTxID = resp.OriginalTxID
          }

          if (resp.Transactions.length > 0) {
            this._refreshTableTransactions(resp.Transactions, false)
          }

          done?.(false, resp.Transactions)
        }, () => {
          done?.(true, [])
        }
      )

      setTimeout(() => {
        this.RefreshTableHistoryLock = false
      }, 300)
    },
    _refreshTableTransactions (infos: Transaction[], unshift: boolean) {
      for (let i = 0; i < infos.length; i++) {
        if (unshift) {
          this.Transactions.unshift(infos[i])
        } else {
          this.Transactions.push(infos[i])
        }
      }
      this.OriginalTxID = this.Transactions[0].TransactionID - 0 + 1
    }
  }
})
