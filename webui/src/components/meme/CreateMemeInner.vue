<template>
  <div>
    <q-input v-model='memeInfo.name' :label='$t("MSG_NAME")' hide-bottom-space :error='nameError' />
    <q-input v-model='memeInfo.symbol' :label='$t("MSG_TICKER")' hide-bottom-space :error='tickerError' />
    <div
      :class='[ "file-upload-area vertical-inner-y-margin", imageError ? "file-upload-area-error shake" : "" ]'
      @dragover.prevent
      @drop.prevent='onFileDrop'
      @click='$refs.fileInput.click()'
    >
      <div v-if='memeInfo.logo' class='image-preview'>
        <q-img :src='memeInfo.logo' fit='scale-down' width='360px' height='100%' />
      </div>
      <q-item-label v-else class='text-h6 text-grey-6'>
        <q-icon name='bi-image' size='32px' />
        <div>{{ $t('MSG_CLICK_OR_DRAG_IMAGE') }}</div>
      </q-item-label>
      <input
        ref='fileInput'
        type='file'
        accept='image/*'
        @change='onFileChange'
        style='display: none;'
      >
      <div v-if='imageError' class='error-message'>{{ errorMessage }}</div>
    </div>
    <q-input v-model='memeInfo.description' type='textarea' filled :label='$t("MSG_DESCRIPTION")' />
    <div class='vertical-inner-y-margin'>
      <q-toggle dense v-model='memeInfo.mintable' :label='$t("MSG_MINTABLE_WITH_CAPTION")' />
    </div>
    <q-expansion-item
      dense
      expand-icon-toggle
      expand-separator
      :label='$t("MSG_MORE_OPTIONS")'
      v-model='expanded'
      class='decorate-border-bottom vertical-inner-y-margin text-grey-8'
    >
      <q-card>
        <q-input v-model='memeInfo.website' :label='$t("MSG_OFFICIAL_WEBSITE") + " (" + $t("MSG_OPTIONAL") + ")"' />
        <q-input v-model='memeInfo.twitter' :label='$t("MSG_TWITTER") + " (" + $t("MSG_OPTIONAL") + ")"' />
        <q-input v-model='memeInfo.telegram' :label='$t("MSG_TELEGRAM") + " (" + $t("MSG_OPTIONAL") + ")"' />
        <q-input v-model='memeInfo.discord' :label='$t("MSG_DISCORD") + " (" + $t("MSG_OPTIONAL") + ")"' />
        <q-input v-model='memeInfo.github' :label='$t("MSG_GITHUB") + " (" + $t("MSG_OPTIONAL") + ")"' />
        <q-input v-model='memeInfo.initialSupply' type='number' :label='$t("MSG_INITIAL_SUPPLY")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-input v-model='memeInfo.decimals' type='number' :label='$t("MSG_DECIMALS")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-input v-model='memeInfo.initialCurrency' type='number' :label='$t("MSG_INITIAL_CURRENCY")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-input v-model='memeInfo.feePercent' type='number' :label='$t("MSG_FEE_PERCENT")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-toggle v-model='memeInfo.fixedCurrency' :label='$t("MSG_FIXED_CURRENCY_WITH_CAPTION")' />
        <q-input v-model='initPoolLiquidity.amount0Initial' type='number' :label='$t("MSG_AMOUNT_0_INITIAL")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-input v-model='initPoolLiquidity.amount1Initial' type='number' :label='$t("MSG_AMOUNT_1_INITIAL")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-input v-model='initPoolLiquidity.amount0Virtual' type='number' :label='$t("MSG_AMOUNT_0_VIRTUAL")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
        <q-input v-model='initPoolLiquidity.amount1Virtual' type='number' :label='$t("MSG_AMOUNT_1_VIRTUAL")' :rules='[val => !!val || "Field is required"]' hide-bottom-space />
      </q-card>
    </q-expansion-item>
    <q-btn rounded flat class='border-red-4 full-width vertical-section-y-margin' :label='$t("MSG_CREATE_MEME_TOKEN")' @click='onCreateMemeTokenClick' />
  </div>
</template>
<script setup lang='ts'>
import { computed, ref } from 'vue'
import { NewMemeInfo, ChainApp, InitPoolLiquidity } from 'src/stores/memeInfo'
import gql from 'graphql-tag'
import { graphqlResult } from 'src/utils'
import { useUserStore } from 'src/mystore/user'
import * as constants from 'src/const'

const user = useUserStore()
const account = computed(() => user.account?.trim())
const chainId = computed(() => user.chainId?.trim())
const expanded = ref(false)

const nameError = ref(false)
const tickerError = ref(false)
const imageError = ref(false)

const applicationIds = ref([] as string[])

const initPoolLiquidity = ref({
  amount0Initial: '5',
  amount1Initial: '1',
  amount0Virtual: '5',
  amount1Virtual: '1'
} as InitPoolLiquidity)

const memeInfo = ref({
  initialSupply: '21000000',
  decimals: '6',
  initialCurrency: '0.00001',
  feePercent: '0',
  fixedCurrency: false,
  mintable: true,
  website: '',
  twitter: '',
  telegram: '',
  discord: '',
  github: '',
  description: "Creator didn't leave any information about this token. You should know if you interact with malfunction application, you may lose your assets!"
} as NewMemeInfo)

const onCheckSymbol = async () => {
  const appID = constants.constants.amsAppID
  const symbol = memeInfo.value.symbol
  await getApplicationExistBySymbol(appID, symbol)
    .then((exist) => {
      if (exist) {
        throw new Error('Invalid same symbol')
      }
    })
    .catch((error) => {
      console.log('getApplicationSymbol error: ', error)
      throw error
    })
}

const MAXSIZE = 4 * 1024 * 1024
const errorMessage = ref('')
const onFileDrop = (event: DragEvent): void => {
  const files = event.dataTransfer?.files
  const file = files?.[0]
  if (file) {
    if (file.size > MAXSIZE) {
      imageError.value = true
      errorMessage.value = 'The image size must not exceed 4MB.'
      throw new Error(errorMessage.value)
    }
    errorMessage.value = ''
    imageError.value = false
    const reader = new FileReader()
    reader.onload = (e: ProgressEvent<FileReader>): void => {
      if (e.target) {
        memeInfo.value.logo = e.target.result as string
      }
    }
    reader.readAsDataURL(file)
  }
}

const onFileChange = (event: Event): void => {
  const target = event.target as HTMLInputElement
  const file = target.files?.[0]
  if (file) {
    if (file.size > MAXSIZE) {
      imageError.value = true
      errorMessage.value = 'The image size must not exceed 4MB.'
      throw new Error(errorMessage.value)
    }
    errorMessage.value = ''
    imageError.value = false
    const reader = new FileReader()
    reader.onload = (e: ProgressEvent<FileReader>): void => {
      if (e.target) {
        memeInfo.value.logo = e.target.result as string
      }
    }
    reader.readAsDataURL(file)
  }
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const getApplicationIds = async (): Promise<string[]> => {
  const publicKey = account.value
  const query = gql`
    query applications ($chainId: String!) {
      applications(chainId: $chainId) {
        id
      }
    }`
  return new Promise((resolve, reject) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    window.linera.request({
      method: 'linera_graphqlQuery',
      params: {
        publicKey: publicKey,
        query: {
          query: query.loc?.source?.body,
          variables: {
            chainId: chainId.value
          }
        }
      }
    }).then((result) => {
      const applications = graphqlResult.keyValue(result, 'applications') as ChainApp[]
      resolve(applications.map((el) => el.id))
    }).catch((e) => {
      reject(e)
    })
  })
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const getApplicationExistBySymbol = async (appID: string, symbol: string): Promise<boolean> => {
  const publicKey = account.value
  const query = gql`
    query applications ($spec: String!, $limit: Int!) {
      applications(spec: $spec, limit: $limit)
    }`
  return new Promise((resolve, reject) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    window.linera.request({
      method: 'linera_graphqlQuery',
      params: {
        applicationId: appID,
        publicKey: publicKey,
        query: {
          query: query.loc?.source?.body,
          variables: {
            spec: symbol,
            limit: 1
          }
        }
      }
    }).then((result) => {
      const applications = graphqlResult.keyValue(result, 'applications') as []
      resolve(applications.length > 0)
    }).catch((e) => {
      reject(e)
    })
  })
}

const validateParams = (): boolean => {
  nameError.value = !memeInfo.value.name?.length
  tickerError.value = !memeInfo.value.symbol?.length
  imageError.value = !memeInfo.value.logo?.length
  return !(nameError.value || tickerError.value || imageError.value)
}

interface InstantiationArgument {
  initial_supply: string
  name: string
  symbol: string
  decimals: number
  initial_currency: string
  fixed_currency: boolean
  fee_percent: string
}

interface ChainAccount {
  chain_id: string
  owner: string
}

interface TokenMetadata {
  logo: string
  twitter: string
  discord: string
  telegram: string
  website: string
  github: string
  mintable: boolean
  description: string
}

interface ApplicationParameters {
  initial_balances: Map<string, string>
  swap_application_id: string
  token_metadata: TokenMetadata
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const createApplication = async (): Promise<any> => {
  const instantiationArgument = {
    initial_supply: '21000000',
    name: memeInfo.value.name,
    symbol: memeInfo.value.symbol,
    decimals: 6,
    initial_currency: '0.00001',
    fixed_currency: false,
    fee_percent: '0'
  } as InstantiationArgument
  const applicationParameters = {
    initial_balances: new Map([
      [
        JSON.stringify({
          chain_id: constants.constants.swapCreationChainID,
          owner: 'User:' + constants.constants.swapCreationOwner
        } as ChainAccount),
        '5000000.'
      ]
    ]),
    swap_application_id: constants.constants.swapAppID,
    token_metadata: {
      logo: memeInfo.value.logo,
      twitter: memeInfo.value.twitter,
      telegram: memeInfo.value.telegram,
      discord: memeInfo.value.discord,
      website: memeInfo.value.website,
      github: memeInfo.value.github,
      mintable: memeInfo.value.mintable,
      description: memeInfo.value.description
    } as TokenMetadata
  } as ApplicationParameters

  const publicKey = account.value
  const query = gql`
    mutation createApplication($chainId: String!, $bytecodeId: String!, $parameters: String!, $instantiationArgument: String!, $requiredApplicationIds: String!, ) {
      createApplication(chainId: $chainId, bytecodeId: $bytecodeId, parameters: $parameters, instantiationArgument: $instantiationArgument, requiredApplicationIds: $requiredApplicationIds)
    }`
  return new Promise((resolve, reject) => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-unsafe-member-access
    window.linera.request({
      method: 'linera_graphqlMutation',
      params: {
        publicKey: publicKey,
        query: {
          query: query.loc?.source?.body,
          variables: {
            chainId: chainId.value,
            bytecodeId: constants.constants.erc20BID,
            parameters: JSON.stringify(applicationParameters, (key, value) => {
              if (value instanceof Map) {
                return Object.fromEntries(value)
              }
              return value
            }),
            instantiationArgument: JSON.stringify(instantiationArgument),
            requiredApplicationIds: []
          },
          operationName: 'createApplication'
        }
      }
    }).then((result) => {
      resolve(result)
    }).catch((e) => {
      reject(e)
    })
  })
}

const emit = defineEmits<{(ev: 'created', applicationId: string): void,
  (ev: 'creating'): void,
  (ev: 'error', error: string),
  (ev: 'initPoolLiquidity', initPoolLiquidity: InitPoolLiquidity)
}>()

const getCreatedApplicationId = (retries: number) => {
  if (retries > 5) {
    emit('error', 'Failed create token. Please contact support@linera-hacker.io for support.')
    return
  }
  getApplicationIds().then((_applicationIds) => {
    if (_applicationIds.length === applicationIds.value.length) {
      setTimeout(() => getCreatedApplicationId(retries + 1), 1000)
      return
    }
    emit('created', _applicationIds.filter((el) => !applicationIds.value.includes(el)).join(','))
  }).catch((e) => {
    emit('error', `Failed get applicationIds: ${e}`)
  })
}

const onCreateMemeTokenClick = async () => {
  if (!validateParams()) return
  try {
    await onCheckSymbol()
  } catch (error) {
    tickerError.value = true
    throw error
  }

  emit('creating')

  getApplicationIds().then((_applicationIds) => {
    applicationIds.value = _applicationIds
    createApplication()
      .then(() => {
        emit('initPoolLiquidity', initPoolLiquidity.value)
        setTimeout(() => getCreatedApplicationId(0), 1000)
      }).catch((e) => {
        emit('error', `Failed create application: ${e}`)
      })
  }).catch((e) => {
    emit('error', `Failed get applicationIds: ${e}`)
  })
}

</script>

<style lang='sass' scoped>
.file-upload-area
  border: 2px dashed #ccc
  padding: 20px
  text-align: center
  cursor: pointer
  margin-bottom: 20px
  display: flex
  flex-direction: column
  justify-content: center
  align-items: center
  height: 180px
  width: 100%
  background: #ebebeb

.file-upload-area-error
  border: 2px dashed $red-6

.image-preview
  top: 0
  left: 0
  right: 0
  bottom: 0
  display: flex
  justify-content: center
  align-items: center

.image-preview .q-img
  max-width: 100%
  max-height: 200px
  object-fit: contain

:deep(.q-item, .q-item--dense)
    padding: 0 !important

.error-message
  color: red
  margin-top: 10px
  
</style>
