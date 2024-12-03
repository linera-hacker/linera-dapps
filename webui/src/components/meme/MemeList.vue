<template>
  <q-page>
    <q-infinite-scroll @load='onLoad' :offset='300' :style='{padding: "0 8px"}'>
      <div class='row'>
        <div v-for='item in memeAppInfos' :key='item.appID' class='col-xs-12 col-sm-6 col-md-4'>
          <MemeCard :meme-info='item' />
        </div>
      </div>
    </q-infinite-scroll>
    <div class='q-pa-md q-gutter-xs flex flex-center'>
      <div class='q-gutter-md row justify-center'>
        <q-spinner-ball
          color='red'
          size='5.5em'
          v-if='loading'
        />
      </div>
    </div>
  </q-page>
</template>

<script setup lang='ts'>
import { onBeforeUnmount, onMounted, ref } from 'vue'
import { MemeAppRespInfo, MemeAppInfoSpec, MemeAppInfoDisplay } from 'src/stores/memeInfo'
import * as constants from 'src/const'
import { ApolloClient } from '@apollo/client/core'
import gql from 'graphql-tag'
import { graphqlResult } from 'src/utils'
import { getAppClientOptions } from 'src/apollo'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { Pool } from 'src/stores/pool'

import MemeCard from './MemeCard.vue'

const swapChainID = ref(constants.constants.swapCreationChainID)
const swapAppID = ref(constants.constants.swapAppID)
const swapEndPoint = ref(constants.constants.swapEndPoint)

const amsChainID = ref(constants.constants.amsCreationChainID)
const amsAppID = ref(constants.constants.amsAppID)
const amsEndPoint = ref(constants.constants.amsEndPoint)

const swapService = ref(swapEndPoint.value + '/chains/' + swapChainID.value + '/applications/' + swapAppID.value)
const amsService = ref(amsEndPoint.value + '/chains/' + amsChainID.value + '/applications/' + amsAppID.value)

const limit = ref(40)
const appPoolIDsMap = ref<Map<string, string>>(new Map())
const appIDsMap = ref<Map<string, string>>(new Map())
const memeAppInfos = ref([] as MemeAppInfoDisplay[])
const lastCreatedAt = ref(0)
const curPageSize = ref(limit.value)

const getPools = async (url: string): Promise<Array<Pool>> => {
  const appOptions = /* await */ getAppClientOptions(url)
  const appApolloClient = new ApolloClient(appOptions)
  const { /* result, refetch, fetchMore, */ onResult, onError } = provideApolloClient(appApolloClient)(() => useQuery(gql`
    query {
      getPools {
        id
        token0
        token1
      }
    }
  `, {
    endpoint: 'swap',
    chainId: swapChainID.value
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

const onGetAppPools = async () => {
  const url = swapService.value
  await getPools(url)
    .then((pools) => {
      for (let i = 0; i < pools.length; i++) {
        const token0 = pools[i].token0
        const token1 = pools[i].token1
        appPoolIDsMap.value.set(token0, pools[i].id.toString())
        appPoolIDsMap.value.set(token1, pools[i].id.toString())
      }
    })
    .catch((error) => {
      console.log('getPool error: ', error)
    })
}

const getApplicationInfos = async (url: string) => {
  const appOptions = /* await */ getAppClientOptions(url)
  const appApolloClient = new ApolloClient(appOptions)
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(appApolloClient)(() => useQuery(gql`
    query getApplications($createdAfter: Int!, $limit: Int!){
      applications(createdAfter: $createdAfter, limit: $limit)
    }
  `, {
    createdAfter: lastCreatedAt.value,
    limit: limit.value
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const apps = graphqlResult.data(res, 'applications') as Array<MemeAppRespInfo>
    for (let i = 0; i < apps.length; i++) {
      const poolId = appPoolIDsMap.value.get(apps[i].application_id)
      if (poolId === undefined || poolId === null || poolId === "") {
        console.log(apps[i].application_id + ' not in pools')
        continue
      }
      const checkExist = appIDsMap.value.get(apps[i].application_id)
      if (checkExist) {
        console.log(apps[i].application_id + ' exist')
        continue
      }
      appIDsMap.value.set(apps[i].application_id, apps[i].application_id)
      const parsedSpec: MemeAppInfoSpec = JSON.parse(apps[i].spec);
      const meme = {
        poolID: poolId,
        appID: apps[i].application_id,
        appName: apps[i].application_name,
        appType: apps[i].application_type,
        createdAt: apps[i].created_at,
        description: apps[i].description,
        discord: apps[i].discord,
        github: apps[i].github,
        logo: apps[i].logo,
        telegram: apps[i].telegram,
        twitter: apps[i].twitter,
        website: apps[i].website,
        ticker: parsedSpec.ticker,
        initialSupply: parsedSpec.initial_supply,
        mintable: parsedSpec.mintable
      } as MemeAppInfoDisplay
      memeAppInfos.value.push(meme)
      if (apps[i].created_at > lastCreatedAt.value) {
        lastCreatedAt.value = apps[i].created_at
      }
    }
    if (memeAppInfos.value.length >= curPageSize.value) {
      runningInterval.value = false
      curPageSize.value += limit.value
      return    
    } 
    runningInterval.value = true
  })
}

const sleep = async (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

const onGetAppInfos = async () => {
  await onGetAppPools()
  await sleep(1000)
  await getApplicationInfos(amsService.value)
} 

onMounted(async () => {
  await Promise.resolve()
  if (!memeAppInfos.value?.length) {
    onLoading()
    onGetAppInfos()
      .catch((error) => {
        console.log('onGetAppInfos error: ', error)
      })
    onHiding()
  }
})

const loading = ref(false)

const onLoading = () => {
  loading.value = true
}

const onHiding = () => {
  loading.value = false
}

const onLoad = (index, done) => {
  onLoading()
  setTimeout(() => {
    onGetAppInfos()
    onHiding()
    done()
  }, 2000)
}

const runningInterval = ref(true)

const useIntervalLoadData = () => {
  if (!runningInterval.value) return
  onGetAppInfos()
}

const interval = setInterval(useIntervalLoadData, 20 * 1000)

onBeforeUnmount(() => {
  clearInterval(interval)
})

</script>

<style lang='sass' scoped>
</style>
