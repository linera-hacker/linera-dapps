<template>
  <div>
    <div>
      <strong>Holder bulletin</strong>
    </div>
    <q-separator />
    <div v-for='(item, index) in topList' :key='item.owner.chain_id' class='row decorate-dashed-border-bottom vertical-inner-y-margin'>
      <q-img
        v-if='index===0'
        :src='trophyNo1' width='15px' height='15px'
        class='cursor-pointer horizontal-inner-x-margin-right'
      />
      <q-img
        v-if='index===1'
        :src='trophyNo2' width='15px' height='15px'
        class='cursor-pointer horizontal-inner-x-margin-right'
      />
      <q-img
        v-if='index===2'
        :src='trophyNo3' width='15px' height='15px'
        class='cursor-pointer horizontal-inner-x-margin-right'
      />
      <q-img
        v-if='index>2'
        src='' width='15px' height='15px'
        class='cursor-pointer horizontal-inner-x-margin-right'
      />
      <div>{{ shortId(item.owner.chain_id, 10) }}</div>
      <q-space />
      <div>{{ Number(item.balance).toFixed(2) }} WLINERA</div>
    </div>
  </div>
</template>
<script setup lang='ts'>
import { onMounted, ref, watch } from 'vue'
import * as constants from 'src/const'
import gql from 'graphql-tag'
import { useRouter } from 'vue-router'
import { ApolloClient } from '@apollo/client/core'
import { graphqlResult } from 'src/utils'
import { getAppClientOptions } from 'src/apollo'
import { provideApolloClient, useQuery } from '@vue/apollo-composable'
import { OwnerBalance } from 'src/stores/memeInfo'
import { trophyNo1 } from 'src/assets'
import { trophyNo2 } from 'src/assets'
import { trophyNo3 } from 'src/assets'
import { useSwapStore } from 'src/mystore/swap'
import { shortId } from 'src/utils/shortid'

const router = useRouter()
const swapStore = useSwapStore()

const topList = ref([] as Array<OwnerBalance>)

const token0 = ref('')

watch(() => swapStore.SelectedTokenPair, (selected) => {
  if (!selected) {
    return
  }
  token0.value = selected.TokenZeroAddress
  getBalanceTopList(10)
})

onMounted(() => {
  if (router.currentRoute.value.query.token0 != null) {
    token0.value = router.currentRoute.value.query.token0 as string
    getBalanceTopList(10)
  }
})

const swapChainID = ref(constants.constants.swapCreationChainID)
const swapEndPoint = ref(constants.constants.swapEndPoint)

const getBalanceTopList = async (limit: number) => {
  const url = swapEndPoint.value + '/chains/' + swapChainID.value + '/applications/' + token0.value
  const appOptions = /* await */ getAppClientOptions(url)
  const appApolloClient = new ApolloClient(appOptions)
  const { /* result, refetch, fetchMore, */ onResult /*, onError */ } = provideApolloClient(appApolloClient)(() => useQuery(gql`
    query balanceTopList($limit: Int!) {
      balanceTopList(limit: $limit) {
        owner,
        balance
      }
    }
  `, {
    chainId: swapChainID.value,
    limit: limit,
  }, {
    fetchPolicy: 'network-only'
  }))

  onResult((res) => {
    if (res.loading) return
    const ownerBalances = graphqlResult.data(res, 'balanceTopList') as Array<OwnerBalance>
    topList.value = ownerBalances
  })
}

</script>
