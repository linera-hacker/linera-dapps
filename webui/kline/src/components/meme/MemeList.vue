<template>
  <q-page>
    <div class='row'>
      <div v-for='item in memeInfos' :key='item.appID' class='col-xs-12 col-sm-6 col-md-4'>
        <MemeCard :meme-info='item' />
      </div>
    </div>
    <div class='q-pa-md q-gutter-xs flex flex-center'>
      <div class='q-gutter-md row justify-center'>
        <q-spinner-ball
          color='red'
          size='5.5em'
          v-if='loading'
        />
      </div>
    </div>
    <div v-if='maxPage > 1' class='q-pa-lg flex flex-center'>
      <q-pagination
        v-model='currentPage'
        color='black'
        :max='maxPage'
        :max-pages='5'
        :boundary-numbers='false'
      />
    </div>
  </q-page>
</template>

<script setup lang='ts'>
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { MemeInfo } from 'src/stores/memeInfo'
import * as constants from 'src/const'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { graphqlResult } from 'src/utils'
import { getAppClientOptions } from 'src/apollo'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { Pool } from 'src/stores/pool'

import MemeCard from './MemeCard.vue'

const currentPage = ref(1)
const maxPage = ref(10)
const limit = ref(40)

const memeInfos = ref([] as MemeInfo[])

const loading = ref(false)

const onLoading = () => {
  loading.value = true
}

const onHiding = () => {
  loading.value = false
}

watch(currentPage, () => {
  onLoadPageData()
    .catch((error) => {
      console.log('onGetApplicationsByPage error: ', error)
    })
})

const onGetApplicationsByPage = async () => {
  await Promise.resolve()
  memeInfos.value.length = 0
  const offset = currentPage.value - 1
  const startCount = offset * limit.value
  const nextCount = appIDs.value.length - startCount
  const size = (nextCount > limit.value) ? limit.value : nextCount
  for (let i = startCount; i < (startCount + size); i++) {
    const appID = appIDs.value[i]
    await getAppInfo(appID)
      .catch((error) => {
        console.log('getAppInfos error: ', error)
      })
  }
}

const curChainID = ref(constants.constants.swapCreationChainID)

const swapAppID = ref(constants.constants.swapAppID)

const curEndPoint = ref(constants.constants.swapEndPoint)

const swapService = ref(curEndPoint.value + '/chains/' + curChainID.value + '/applications/' + swapAppID.value)

const appIDsMap = ref<Map<string, string>>(new Map())
const appIDs = ref<string[]>([])

const sleep = async (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

const onGetPools = async () => {
  const url = swapService.value
  await getPools(url)
    .then((pools) => {
      for (let i = 0; i < pools.length; i++) {
        const token0 = pools[i].token0
        const token1 = pools[i].token1
        appIDsMap.value.set(token0, pools[i].id.toString())
        appIDsMap.value.set(token1, pools[i].id.toString())
      }
      appIDsMap.value.forEach((value, key) => {
        appIDs.value.push(key)
      })
      maxPage.value = Math.ceil(appIDs.value.length / limit.value)
    })
    .catch((error) => {
      console.log('getPool error: ', error)
    })
}

const getPools = async (url: string): Promise<Array<Pool>> => {
  const appOptions = /* await */ getAppClientOptions(url)
  const appApolloClient = new ApolloClient(appOptions)
  const { /* result, refetch, fetchMore, */ onResult, onError } = provideApolloClient(appApolloClient)(() => useQuery(gql`
    query {
      getPools {
        id
        token0
        token1
        virtualInitialLiquidity
        amount0Initial
        amount1Initial
        reserve0
        reserve1
        poolFeePercent
        protocolFeePercent
        erc20 {
          totalSupply
          balances
        }
        feeTo
        feeToSetter
        price0Cumulative
        price1Cumulative
        kLast
        blockTimestamp
      }
    }
  `, {
    endpoint: 'swap',
    chainId: curChainID.value
  }, {
    fetchPolicy: 'network-only'
  }))

  return new Promise((resolve, reject) => {
    onResult((res) => {
      if (res.loading) return
      const pools = graphqlResult.data(res, 'getPools') as Array<Pool>
      resolve(pools)
    })
    onError((error) => {
      reject(error)
    })
  })
}

const getAppInfo = async (appID: string) => {
  const url = curEndPoint.value + '/chains/' + curChainID.value + '/applications/' + appID
  console.log('url: ', url)
  const appOptions = /* await */ getAppClientOptions(url)
  const appApolloClient = new ApolloClient(appOptions)
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(appApolloClient)(() => useQuery(gql`
    query {
      name,
      symbol,
      totalSupply,
      decimals
      tokenMetadata
    }
  `, {
    endpoint: 'main',
    chainId: curChainID.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const memeInfo = res.data as MemeInfo
    const poolId = appIDsMap.value.get(appID)
    const meme = {
      appID: appID,
      link: url,
      name: memeInfo.name,
      symbol: memeInfo.symbol,
      totalSupply: memeInfo.totalSupply,
      decimals: memeInfo.decimals,
      tokenMetadata: memeInfo.tokenMetadata,
      poolId: poolId,
    } as MemeInfo
    if (meme.tokenMetadata === null) {
      // DO NOTHING
    } else {
      memeInfos.value.push(meme)
    }
  })
}

onMounted(async () => {
  await Promise.resolve()
  if (!memeInfos.value?.length) {
    onLoadData()
      .catch((error) => {
        console.log('onLoadData error: ', error)
      })
  }
})

const onLoadPageData = async () => {
  memeInfos.value.length = 0
  onLoading()
  try {
    await onGetApplicationsByPage()
  } catch (error) {
    console.log('onLoadPageData error: ', onLoadPageData)
  }
  onHiding()
}

const onLoadData = async () => {
  onLoading()
  onGetPools()
    .catch((error) => {
      console.log('onGetPools error: ', error)
    })
  await sleep(1000)
  onLoadPageData()
    .catch((error) => {
      console.log('onLoadPageData error: ', error)
    })
}

const curPageCountChange = ref(false)
const checkCurPageCountChange = () => {
  const offset = currentPage.value - 1
  const startCount = offset * limit.value
  const nextCount = appIDs.value.length - startCount
  const size = (nextCount > limit.value) ? limit.value : nextCount
  if (size < limit.value) {
    curPageCountChange.value = true
  }
}

const onRefreshData = async () => {
  const url = swapService.value
  await getPools(url)
    .then(async (pools) => {
      for (let i = 0; i < pools.length; i++) {
        const token0 = pools[i].token0
        const token1 = pools[i].token1
        const app0 = appIDsMap.value.get(token0) as string
        const app1 = appIDsMap.value.get(token1) as string
        if (app0 == undefined || app0 == null || app0 == '') {
          appIDsMap.value.set(token0, pools[i].id.toString())
          appIDs.value.push(token0)
          await checkCurPageCountChange()
        }
        if (app1 == undefined || app1 == null || app1 == '') {
          appIDsMap.value.set(token1, pools[i].id.toString())
          appIDs.value.push(token1)
          await checkCurPageCountChange()
        }
      }
      maxPage.value = Math.ceil(appIDs.value.length / limit.value)
      if (curPageCountChange.value) {
        await onLoadPageData()
      }
      curPageCountChange.value = false
    })
    .catch((error) => {
      console.log('getPool error: ', error)
    })
}

const interval = setInterval(onRefreshData, 20 * 1000)

onBeforeUnmount(() => {
  clearInterval(interval)
})

</script>

<style lang='sass' scoped>
</style>
