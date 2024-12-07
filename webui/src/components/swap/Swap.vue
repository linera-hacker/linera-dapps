<template>
  <div class='bg-white vertical-card-padding'>
    <div>
      <strong>Swap</strong>
    </div>
    <q-separator />
    <q-card flat class='bg-red-1 border-radius-8px popup-padding vertical-inner-y-margin'>
      <div class='row'>
        <div class='text-bold text-grey-8'>
          {{ $t('MSG_YOU_ARE_SELLING') }}
        </div>
        <q-space />
        <div class='row'>
          <q-icon name='bi-wallet-fill' class='text-grey-8 swap-amount-icon' size='16px' />
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
          class='swap-amount-input text-grey-8' dense v-model.number='outAmount' reverse-fill-mask
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
        <div class='text-bold text-grey-8'>
          {{ $t('MSG_YOU_ARE_BUYING') }}
        </div>
        <q-space />
        <div class='row'>
          <q-icon name='bi-wallet-fill' class='text-grey-8 swap-amount-icon' size='16px' />
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
          class='swap-amount-input' dense v-model.number='inAmount' reverse-fill-mask
          input-class='text-right'
        />
      </div>
    </q-card>
    <q-btn
      rounded flat :label='$t("MSG_SWAP")' class='full-width border-red-4 vertical-inner-y-margin vertical-inner-y-margin-bottom'
      @click='SwapAmount'
    />
  </div>
</template>

<script setup lang='ts'>
import { gql } from '@apollo/client'
import { constants } from 'src/const'
import { dbModel } from 'src/model'
import { useNotificationStore } from 'src/mystore/notification'
import { useSwapStore } from 'src/mystore/swap'
import { useUserStore } from 'src/mystore/user'
import { useWalletStore } from 'src/mystore/wallet'
import { shortId } from 'src/utils/shortid'
import { ref, watch } from 'vue'

let triggerOutAmount = true
let triggerInAmount = true
const outAmount = ref(0)
const inAmount = ref(0)

const outBalance = ref(0)
const inBalance = ref(0)

const swapStore = useSwapStore()
const walletStore = useWalletStore()
const userStore = useUserStore()
const notificationStore = useNotificationStore()

const CalSwapInAmount = (_outAmount?: number, _inAmount?: number) => {
  if (!swapStore.SelectedToken || _inAmount === 0) {
    outAmount.value = 0
    return
  }
  if (!swapStore.SelectedTokenPair || _outAmount === 0) {
    inAmount.value = 0
    return
  }

  if (_outAmount !== undefined) {
    walletStore.calSwapAmount(
      swapStore.SelectedTokenPair.TokenOneAddress,
      swapStore.SelectedTokenPair.TokenZeroAddress,
      outAmount.value,
      (_, amount) => {
        triggerInAmount = false
        inAmount.value = amount
      }
    )
  }

  if (_inAmount !== undefined) {
    walletStore.calSwapAmount(
      swapStore.SelectedTokenPair.TokenZeroAddress,
      swapStore.SelectedTokenPair.TokenOneAddress,
      inAmount.value,
      (_, amount) => {
        triggerOutAmount = false
        outAmount.value = amount
      }
    )
  }
}

const swapCreationChainID = ref(constants.swapCreationChainID)
const swapAppID = ref(constants.swapAppID)

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const approveToSwap = async (appID: string, publicKey: string, amount: string): Promise<any> => {
  const chainId = swapCreationChainID.value
  const owner = 'Application:' + swapAppID.value

  const query = gql`
    mutation approve($chainId: String!, $owner: String!, $amount: String!) {
      approve(spender: {chain_id: $chainId, owner: $owner}, value: $amount)
    }`
  return new Promise((resolve, reject) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    window.linera.request({
      method: 'linera_graphqlMutation',
      params: {
        applicationId: appID,
        publicKey: publicKey,
        query: {
          query: query.loc?.source?.body,
          variables: {
            chainId,
            owner,
            amount
          },
          operationName: 'approve'
        }
      }
    }).then((result) => {
      resolve(result)
    }).catch((e) => {
      reject(e)
    })
  })
}

const SwapAmount = () => {
  if (!swapStore.SelectedToken) {
    return
  }
  if (!swapStore.SelectedTokenPair) {
    return
  }
  if (!outAmount.value || outAmount.value < 0) {
    return
  }

  approveToSwap(
    swapStore.SelectedTokenPair?.TokenZeroAddress,
    userStore.account,
    outAmount.value.toString()
  ).then(() => {
    walletStore.swapAmount(
      swapStore.SelectedTokenPair?.TokenZeroAddress || '',
      swapStore.SelectedTokenPair?.TokenOneAddress || '',
      userStore.account,
      outAmount.value
    ).then().catch((e) => {
      notificationStore.pushNotification({
        Title: 'swap amount',
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
    outAmount.value = 0
    return
  }

  CalSwapInAmount(outAmount.value, undefined)

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
    inAmount.value = 0
    return
  }

  CalSwapInAmount(undefined, inAmount.value)
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
