<template>
  <q-expansion-item
    expand-separator
    icon='assessment'
    label='Add Liquidity'
  >
    <div class='bg-white vertical-card-padding'>
      <q-separator />
      <q-card flat class='bg-red-1 border-radius-8px popup-padding vertical-inner-y-margin'>
        <div class='row'>
          <q-space />
          <div class='row'>
            <q-icon name='bi-wallet-fill text-grey-8 swap-amount-icon' size='16px' />
            <div class='swap-amount-label text-grey-9 text-bold'>
              {{ Number(outBalance).toFixed(2) }}
            </div>
            <div class='text-grey-8'>
              {{ swapStore.SelectedToken?.Symbol }}
            </div>
          </div>
        </div>
        <div class='row vertical-card-align swap-token'>
          <div>
            <div class='text-bold'>
              {{ swapStore.SelectedToken?.Symbol }}
            </div>
            <div class='text-grey-8' :title='swapStore.SelectedToken?.Address'>
              {{ shortId(swapStore.SelectedToken?.Address || '', 8) }}
            </div>
          </div>
          <q-space />
          <q-input
            class='swap-amount-input text-grey-8' dense v-model.number='tokenZeroAmount' reverse-fill-mask
            input-class='text-right'
          />
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
          <q-space />
          <div class='row'>
            <q-icon name='bi-wallet-fill text-grey-8 swap-amount-icon' size='16px' />
            <div class='swap-amount-label text-grey-9 text-bold'>
              {{ Number(inBalance).toFixed(2) }}
            </div>
            <div class='text-grey-8'>
              {{ swapStore.SelectedTokenPair?.TokenOneSymbol }}
            </div>
          </div>
        </div>
        <div class='row vertical-card-align swap-token'>
          <div>
            <div class='text-bold'>
              {{ swapStore.SelectedTokenPair?.TokenOneSymbol }}
            </div>
            <div class='text-grey-8' :title='swapStore.SelectedTokenPair?.TokenOneAddress'>
              {{ shortId(swapStore.SelectedTokenPair?.TokenOneAddress || '', 8) }}
            </div>
          </div>
          <q-space />
          <q-input
            class='swap-amount-input' dense v-model.number='tokenOneAmount' reverse-fill-mask
            input-class='text-right'
          />
        </div>
      </q-card>
      <q-btn
        rounded flat :label='$t("MSG_ADD_LIQUIDITY")' class='full-width border-red-4 vertical-inner-y-margin vertical-inner-y-margin-bottom'
        @click='onAddLiquidity'
      />
    </div>
  </q-expansion-item>
</template>

<script setup lang='ts'>
import { dbModel } from 'src/model'
import { useNotificationStore } from 'src/mystore/notification'
import { useSwapStore } from 'src/mystore/swap'
import { useUserStore } from 'src/mystore/user'
import { useWalletStore } from 'src/mystore/wallet'
import { shortId } from 'src/utils/shortid'
import { ref, watch } from 'vue'

const triggerOutAmount = ref(true)
const triggerInAmount = ref(true)

const tokenZeroAmount = ref(0)
const tokenOneAmount = ref(0)

const outBalance = ref(0)
const inBalance = ref(0)

const swapStore = useSwapStore()
const walletStore = useWalletStore()
const userStore = useUserStore()
const notificationStore = useNotificationStore()

const onAddLiquidity = () => {
  if (swapStore.SelectedToken === null) {
    return
  }
  if (swapStore.SelectedTokenPair === null) {
    return
  }
  if (tokenZeroAmount.value === null || tokenZeroAmount.value < 0) {
    return
  }
  if (tokenOneAmount.value === null || tokenOneAmount.value < 0) {
    return
  }
  dbModel.ownerFromPublicKey(userStore.account).then(() => {
    walletStore.addLiquidity(
      swapStore.SelectedTokenPair?.TokenZeroAddress || '',
      swapStore.SelectedTokenPair?.TokenOneAddress || '',
      userStore.account,
      tokenZeroAmount.value,
      tokenOneAmount.value
    ).then().catch((e) => {
      notificationStore.pushNotification({
        Title: 'add Liquidity',
        Message: e as string,
        Description: 'please retry'
      })
    })
  }).catch((e) => {
    notificationStore.pushNotification({
      Title: 'gen account from user',
      Message: e as string,
      Description: 'please connect plugin and retry'
    })
  })
}

watch(() => swapStore.SelectedToken, (selected) => {
  if (selected === null) {
    tokenZeroAmount.value = 0
    return
  }

  dbModel.ownerFromPublicKey(userStore.account).then((v) => {
    walletStore.getBalance(selected.Address, userStore.chainId, v, (error, balance) => {
      if (error) {
        return
      }
      outBalance.value = Number(balance)
    })
  }).catch((e) => {
    notificationStore.pushNotification({
      Title: 'gen account from user',
      Message: e as string,
      Description: 'please connect plugin and retry'
    })
  })
})

watch(() => swapStore.SelectedTokenPair, (selected) => {
  if (!selected) {
    tokenOneAmount.value = 0
    return
  }

  if (!userStore.account) {
    return
  }

  dbModel.ownerFromPublicKey(userStore.account).then((v) => {
    walletStore.getBalance(selected.TokenOneAddress, userStore.chainId, v, (error, balance) => {
      if (error) {
        return
      }
      inBalance.value = Number(balance)
    })
  }).catch((e) => {
    notificationStore.pushNotification({
      Title: 'gen account from user',
      Message: e as string,
      Description: 'please connect plugin and retry'
    })
  })
})

watch(tokenZeroAmount, (amount) => {
  if (amount === null || amount < 0) {
    tokenOneAmount.value = 0
    return
  }
  triggerOutAmount.value = true
})

watch(tokenOneAmount, (amount) => {
  if (amount === null || amount < 0) {
    tokenZeroAmount.value = 0
    return
  }
  triggerInAmount.value = true
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

:deep(.swap-token)
  .q-select
    .q-icon
      font-size: 16px

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
