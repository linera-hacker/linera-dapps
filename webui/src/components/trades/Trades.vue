<template>
  <q-table
    flat
    :columns='(columns as never)'
    :rows='transactions'
    :rows-per-page-options='[10]'
  />
</template>

<script setup lang='ts'>
import { watch, onMounted, computed, onBeforeUnmount } from 'vue'
import { useSwapStore } from 'src/mystore/swap'
import { Transaction, useKLineStore } from 'src/mystore/kline'
import { date } from 'quasar'
import { useI18n } from 'vue-i18n'

const { t } = useI18n({ useScope: 'global' })

const swapStore = useSwapStore()
const klineStore = useKLineStore()
let intervalID: number

const transactions = computed(() => klineStore.Transactions.sort((a, b) => b.Timestamp - a.Timestamp))

watch(() => swapStore.SelectedTokenPair, (selected) => {
  if (selected === null) {
    return
  }
  klineStore.SelectedPoolID = selected.PoolID
  initKPointsStore()
})

const initKPointsStore = () => {
  klineStore.OriginalTxID = 0
  klineStore.Transactions = []
  klineStore.ResetTableViewLock = 0
  klineStore.refreshHistoryTransactions()
}

onMounted(() => {
  if (klineStore.NeedInitTxTable) {
    klineStore.refreshHistoryTransactions()
    klineStore.NeedInitTxTable = false
  }

  intervalID = window.setInterval(() => {
    klineStore.refreshNewTransactions()
    if (klineStore.ResetKLineViewLock + 60000 < new Date().getTime()) {
      klineStore.ResetKLineViewLock = new Date().getTime()
    }
  }, 3000)
})

onBeforeUnmount(() => {
  window.clearInterval(intervalID)
})

const columns = computed(() => [
  {
    name: 'TransactionType',
    label: t('MSG_TRANSACTION_TYPE'),
    align: 'left',
    field: 'TransactionType'
  },
  {
    name: 'Address',
    label: t('MSG_ADDRESS'),
    align: 'center',
    field: 'Owner'
  },
  {
    name: 'Action',
    label: t('MSG_ACTION'),
    align: 'center',
    field: (row: Transaction) => row.AmountZeroIn > 0 ? 'Deposit' : 'Withdraw'
  },
  {
    name: 'Amount',
    label: t('MSG_AMOUNT'),
    align: 'center',
    field: (row: Transaction) => row.AmountZeroIn > 0 ? row.AmountZeroIn : row.AmountOneIn
  },
  {
    name: 'Date',
    label: t('MSG_DATE'),
    align: 'center',
    field: (row: Transaction) => date.formatDate(row.Timestamp * 1000, 'YYYY/MM/DD HH:mm:ss')
  }
])

</script>

<style scoped lang='sass'>
:deep(td)
  height: 36px !important
</style>
