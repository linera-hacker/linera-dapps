<template>
  <div class='bg-white vertical-card-padding'>
    <div>
      <strong>Swap</strong>
    </div>
    <q-separator />
    <q-card flat class='bg-red-1 border-radius-8px popup-padding vertical-inner-y-margin'>
      <div class='row'>
        <div class='text-bold'>
          {{ $t('MSG_YOU_ARE_SELLING') }}
        </div>
        <q-space />
        <div class='row'>
          <q-icon name='bi-wallet-fill text-grey-8 swap-amount-icon' size='16px' />
          <div class='swap-amount-label text-grey-9 text-bold'>{{ Number(swapStore.SelectedToken.Balance).toFixed(2) }}</div>
          <div class='text-grey-8'>{{ swapStore.SelectedToken.Symbol }}</div>
        </div>
      </div>
      <div class='row vertical-card-align swap-token'>
        <q-select dense filled v-model='swapStore.SelectedToken' :options='swapStore.Tokens' dropdown-icon='bi-chevron-down' class='swap-token-option'>
          <template #option='scope'>
            <q-item dense v-bind='scope.itemProps'>
              <q-img :src='scope.opt.Icon' width='24px' height='24px' />
              <div class='swap-token-list'>
                <div class='row'>
                  <div class='swap-token-name text-bold'>{{ scope.opt.Symbol }}</div>
                  <q-space />
                  <q-icon name='bi-wallet-fill text-grey-8 swap-amount-icon' size='16px' />
                  <div>{{ Number(scope.opt.Balance).toFixed(2) }}</div>
                </div>
                <div>{{ shortid.shortId(scope.opt.Address, 6) }}</div>
              </div>
            </q-item>
          </template>
          <template #selected>
            <div class='row'>
              <q-img :src='swapStore.SelectedToken.Icon' width='24px' height='24px' />
              <div class='swap-token-name text-bold swap-token-label'>{{ swapStore.SelectedToken.Symbol }}</div>
            </div>
          </template>
        </q-select>
        <q-input class='swap-amount-input' dense v-model.number='outAmount' reverse-fill-mask input-class='text-right' />
      </div>
    </q-card>
    <div class='row vertical-card-align'>
      <div class='decorate-border-bottom-bold exchange-separator' />
      <div class='exchange-symbol' size='28px'>
        <q-icon name='bi-arrow-down-up' size='14px' class='text-grey-6' />
      </div>
      <div class='decorate-border-bottom-bold exchange-separator' />
    </div>
    <q-card flat class='bg-red-1 border-radius-8px popup-padding vertical-card-align'>
      <div class='row'>
        <div class='text-bold'>
          {{ $t('MSG_YOU_ARE_BUYING') }}
        </div>
        <q-space />
        <div class='row'>
          <q-icon name='bi-wallet-fill text-grey-8 swap-amount-icon' size='16px' />
          <div class='swap-amount-label text-grey-9 text-bold'>{{ Number(swapStore.SelectedTokenPair.TokenOneBalance).toFixed(2) }}</div>
          <div class='text-grey-8'>{{ swapStore.SelectedTokenPair.TokenOneSymbol }}</div>
        </div>
      </div>
      <div class='row vertical-card-align swap-token'>
        <q-select dense filled v-model='swapStore.SelectedTokenPair' :options='swapStore.TokenPairs' dropdown-icon='bi-chevron-down' class='swap-token-option'>
          <template #option='scope'>
            <q-item dense v-bind='scope.itemProps'>
              <q-img :src='scope.opt.TokenOneIcon' width='24px' height='24px' />
              <div class='swap-token-list'>
                <div class='row'>
                  <div class='swap-token-name text-bold'>{{ scope.opt.TokenOneSymbol }}</div>
                  <q-space />
                  <q-icon name='bi-wallet-fill text-grey-8 swap-amount-icon' size='16px' />
                  <div>{{ Number(scope.opt.TokenOneBalance).toFixed(2) }}</div>
                </div>
                <div>{{ shortid.shortId(scope.opt.TokenOneAddress, 6) }}</div>
              </div>
            </q-item>
          </template>
          <template #selected>
            <div class='row'>
              <q-img :src='swapStore.SelectedToken.Icon' width='24px' height='24px' />
              <div class='swap-token-name text-bold swap-token-label'>{{ swapStore.SelectedTokenPair.TokenOneSymbol }}</div>
            </div>
          </template>
        </q-select>
        <q-input class='swap-amount-input' dense v-model.number='inAmount' reverse-fill-mask input-class='text-right' />
      </div>
    </q-card>
    <q-btn rounded flat :label='$t("MSG_SWAP")' class='full-width border-red-4 vertical-inner-y-margin vertical-inner-y-margin-bottom' />
  </div>
</template>

<script setup lang='ts'>
import { dbModel } from 'src/model'
import { useNotificationStore } from 'src/mystore/notification'
import { useSwapStore } from 'src/mystore/swap'
import { useUserStore } from 'src/mystore/user'
import { useWalletStore } from 'src/mystore/wallet'
import { shortid } from 'src/utils'
import { ref, onMounted, watch } from 'vue'

let triggerOutAmount = true
let triggerInAmount = true
const outAmount = ref(0)
const inAmount = ref(0)

const swapStore = useSwapStore()
const walletStore = useWalletStore()
const userStore = useUserStore()
const notificationStore = useNotificationStore()

const GetTokenPairs = () => {
  dbModel.ownerFromPublicKey(userStore.account).then((v) => {
    swapStore.getTokenPairsByTokenZeroID((error) => {
      for (const info of swapStore.TokenPairs) {
        if (error) {
          continue
        }
        walletStore.getBalance(info.TokenOneAddress, v, (error, balance) => {
          if (error) {
            return
          }
          info.TokenOneBalance = Number(balance)
        })
      }
    })
  }).catch((e) => {
    notificationStore.pushNotification({
      Title: 'gen account from user',
      Message: e as string,
      Description: 'please connect plugin and retry'
    })
  })
}

const CalSwapInAmount = (_outAmount?: number, _inAmount?: number) => {
  if (swapStore.SelectedToken === null || swapStore.SelectedToken.ID < 0 || _inAmount === 0) {
    outAmount.value = 0
    return
  }
  if (swapStore.SelectedTokenPair === null || swapStore.SelectedTokenPair.ID < 0 || _outAmount === 0) {
    inAmount.value = 0
    return
  }

  if (_outAmount !== undefined) {
    walletStore.calSwapAmount(
      swapStore.SelectedTokenPair.TokenZeroAddress,
      swapStore.SelectedTokenPair.TokenOneAddress,
      outAmount.value,
      (_, amount) => {
        triggerInAmount = false
        inAmount.value = amount
      }
    )
  }

  if (_inAmount !== undefined) {
    walletStore.calSwapAmount(
      swapStore.SelectedTokenPair.TokenOneAddress,
      swapStore.SelectedTokenPair.TokenZeroAddress,
      inAmount.value,
      (_, amount) => {
        triggerOutAmount = false
        outAmount.value = amount
      }
    )
  }
}

// const SwapAmount = () => {
//   if (swapStore.SelectedToken === null || swapStore.SelectedToken.ID < 0) {
//     return
//   }
//   if (swapStore.SelectedTokenPair === null || swapStore.SelectedTokenPair.ID < 0) {
//     return
//   }
//   if (outAmount.value === null || outAmount.value < 0) {
//     return
//   }

//   walletStore.swapAmount(
//     swapStore.SelectedTokenPair.TokenZeroAddress,
//     swapStore.SelectedTokenPair.TokenOneAddress,
//     outAmount.value,
//     (error, amount) => {
//       if (error) { inAmount.value = amount }
//     }
//   )
// }

watch(() => swapStore.SelectedToken, (selected) => {
  if (selected === null || selected.ID < 0) {
    outAmount.value = 0
    return
  }
  GetTokenPairs()
  CalSwapInAmount(outAmount.value, undefined)
})

watch(() => swapStore.SelectedTokenPair, (selected) => {
  if (selected === null || selected.ID < 0) {
    inAmount.value = 0
    return
  }
  CalSwapInAmount(undefined, inAmount.value)
})

watch(outAmount, (amount) => {
  if (amount === null || amount < 0) {
    inAmount.value = 0
    return
  }
  if (triggerOutAmount) {
    CalSwapInAmount(amount, undefined)
  }
  triggerOutAmount = true
})

watch(inAmount, (amount) => {
  if (amount === null || amount < 0) {
    outAmount.value = 0
    return
  }
  if (triggerInAmount) {
    CalSwapInAmount(undefined, amount)
  }
  triggerInAmount = true
})

// eslint-disable-next-line @typescript-eslint/no-unused-vars
const setSpecifyTokenPair = (t0Addr: string, t1Addr: string) => {
  swapStore.SelectedToken.ID = -1
  for (const item of swapStore.Tokens) {
    if (item.Address === t0Addr) {
      swapStore.SelectedToken = item
      break
    }
  }

  GetTokenPairs()

  swapStore.SelectedTokenPair.ID = -1
  for (const item of swapStore.TokenPairs) {
    if (item.TokenOneAddress === t1Addr) {
      swapStore.SelectedTokenPair = item
    }
  }
}

onMounted(() => {
  if (swapStore.IsInitilazed) {
    return
  }
  swapStore.IsInitilazed = true
  swapStore.getTokens((error) => {
    dbModel.ownerFromPublicKey(userStore.account).then((v) => {
      for (const info of swapStore.Tokens) {
        if (error) {
          continue
        }
        walletStore.getBalance(info.Address, v, (error, balance) => {
          if (error) {
            return
          }
          info.Balance = Number(balance)
        })
      }
    }).catch((e) => {
      notificationStore.pushNotification({
        Title: 'gen account from user',
        Message: e as string,
        Description: 'please connect plugin and retry'
      })
    })
  })
})

</script>

<style scoped lang='sass'>
.swap-amount-label
  font-size: 20px
  margin-right: 4px
  margin-top: -6px

.swap-amount-icon
  margin-right: 4px
  margin-top: 2px

.swap-token-name
  line-height: 24px

:deep(.swap-token)
  .q-select
    .q-icon
      font-size: 16px

.swap-token-list
  min-width: 160px

.swap-token-option
  width: 160px
  border-radius: 4px
  background: $red-2

.swap-token-label
  margin-left: 6px
  width: 84px
  overflow: hidden

.swap-amount-input
  width: calc(100% - 160px)

.exchange-symbol
  border: 2px solid $grey-4
  border-radius: 50%
  width: 28px
  height: 28px
  padding: 2px 5px

.exchange-separator
  width: calc(50% - 14px)
  margin-bottom: 12px
</style>
