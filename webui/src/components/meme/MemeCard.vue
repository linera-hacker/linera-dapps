<template>
  <q-card flat :class='newTx ? "meme-card cursor-pointer shake" : "meme-card cursor-pointer"' @click='onSwap(memeInfo.appID)'>
    <q-item>
      <div class='horizontal-inner-x-margin-right vertical-card-align' avatar>
        <q-img :src='processBase64Img(memeInfo.logo)' width='128px' />
      </div>

      <div>
        <q-item-label class='text-h6'>
          <q-badge color='green-6'>{{ memeInfo.ticker }}</q-badge> {{ memeInfo.appName }}
        </q-item-label>
        <q-item-label>
          <div class='vertical-inner-y-margin'>
            {{ memeInfo.description }}
          </div>
        </q-item-label>
        <q-item-label>
          <div class='vertical-inner-y-margin'>
            <div class='row meme-info'>
              <span class='label text-grey-8'>Last Transaction</span> {{ timeAgo(lastTransaction.LastTxAt) }}, {{ lastTransaction.LastTxOneAmount }} WTLINERA
            </div>
            <div class='row meme-info'>
              <span class='label text-grey-8'>Last 24H Volume</span> {{ lastTransaction.OneDayOneAmountVolumn }} WTLINERA
            </div>
            <div class='row meme-info'>
              <span class='label text-grey-8'>{{ memeInfo.ticker }}/WTLINERA</span> {{ lastTransaction.NowPrice }} WLINERA <span :class='Number(lastTransaction.OneDayIncresePercent) < 0 ? "change text-red" : "change text-green"'>{{ Number(lastTransaction.OneDayIncresePercent) < 0 ? "" : "+" }}{{ lastTransaction.OneDayIncresePercent }}%</span>
            </div>
            <div class='row meme-info'>
              <span class='label text-grey-8'>Market Capacity</span> {{ Number(memeInfo.initialSupply) * Number(lastTransaction.NowPrice) }} WTLINERA
            </div>
          </div>
        </q-item-label>
        <q-item-label caption>
          <div class='row vertical-section-y-margin'>
            <q-img
              v-if='memeInfo.github' :src='githubLogo' width='20px' height='20px'
              class='cursor-pointer horizontal-inner-x-margin-right'
              @click='goLink(memeInfo.github, $event)'
            />
            <q-img
              v-if='memeInfo.discord?.length > 0' :src='discordLogo' width='20px' height='20px'
              class='cursor-pointer horizontal-inner-x-margin-right'
              @click='goLink(memeInfo.discord, $event)'
            />
            <q-img
              v-if='memeInfo.twitter?.length > 0' :src='twitterLogo' width='20px' height='20px'
              class='cursor-pointer horizontal-inner-x-margin-right'
              @click='goLink(memeInfo.twitter, $event)'
            />
            <q-img
              v-if='memeInfo.telegram?.length > 0' :src='telegramLogo' width='20px' height='20px'
              class='cursor-pointer horizontal-inner-x-margin-right'
              @click='goLink(memeInfo.telegram, $event)'
            />
            <q-img
              v-if='memeInfo.website?.length > 0' :src='internetLogo' width='20px' height='20px'
              class='cursor-pointer'
              @click='goLink(memeInfo.website, $event)'
            />
          </div>
        </q-item-label>
      </div>
    </q-item>
  </q-card>
</template>

<script setup lang='ts'>
import { toRef, ref, onBeforeUnmount, onMounted } from 'vue'
import { MemeAppInfoDisplay } from 'src/stores/memeInfo'
import { discordLogo, githubLogo, internetLogo, telegramLogo, twitterLogo } from 'src/assets'
import { useRouter } from 'vue-router'
import { wlineraAppID } from 'src/const/const'
import { LastTranscation, useUserStore } from 'src/mystore/user'

const userStore = useUserStore()

interface Props {
  memeInfo: MemeAppInfoDisplay
}
const props = defineProps<Props>()
const memeInfo = toRef(props, 'memeInfo')
const newTx = ref(true)

const router = useRouter()

const onSwap = (token0: string) => {
  void router.push({
    path: 'swap',
    query: {
      token0,
      token1: wlineraAppID
    }
  })
}

const goLink = (url: string, event: MouseEvent) => {
  event.stopPropagation()
  window.open(url, '_blank')
}

const processBase64Img = (input: string): string => {
  return input.replace(/ /g, '+')
}

// get last transaction
const lastTransaction = ref({} as LastTranscation)

const timeAgo = (timestamp: number): string => {
  if (timestamp === 0) {
    return 'No recent transactions'
  }
  const now = Date.now()
  const seconds = Math.floor((now / 1000) - timestamp)
  const minutes = Math.floor(seconds / 60)
  const hours = Math.floor(minutes / 60)
  const days = Math.floor(hours / 24)

  if (seconds < 60) {
      return seconds === 1 ? '1 second ago' : `${seconds} seconds ago`;
  } else if (minutes < 60) {
      return minutes === 1 ? '1 minute ago' : `${minutes} minutes ago`;
  } else if (hours < 24) {
      return hours === 1 ? '1 hour ago' : `${hours} hours ago`;
  } else {
      return days === 1 ? '1 day ago' : `${days} days ago`;
  }
}

const getNewTxInfo = () => {
  newTx.value = false
  if (memeInfo.value.appID === wlineraAppID) {
    lastTransaction.value = {
      PoolID: 1,
      TokenZeroAddress: '0',
      TokenOneAddress: '0',
      LastTxAt: 0,
      LastTxZeroAmount: '0',
      LastTxOneAmount: '0',
      OneDayZeroAmountVolumn: '0',
      OneDayOneAmountVolumn: '0',
      NowPrice: '0',
      OneDayIncresePercent: '0'
    } as LastTranscation
    return
  }
  userStore.getLastTranscation({
    PoolID: Number(memeInfo.value.poolID),
    TokenZeroAddress: memeInfo.value.appID,
    TokenOneAddress: wlineraAppID,
  }, (error: boolean, row: LastTranscation) => {
    if (error) {
      console.log('error: ', error)
      return
    }
    if (row) {
      if (lastTransaction.value.LastTxAt == undefined || row.LastTxAt !== lastTransaction.value.LastTxAt) {
        lastTransaction.value = row
        newTx.value = true
      }
    }
  })
}

const interval = setInterval(getNewTxInfo, 15 * 1000)

onMounted(() => {
  getNewTxInfo()
})

onBeforeUnmount(() => {
  clearInterval(interval)
})


</script>

<style lang='sass' scoped>
.meme-card
  border: 1px solid transparent
  border-bottom: 1px solid $red-1
  padding: 16px
  border-radius: 16px
  .q-badge
    font-size: 16px

.meme-card:hover
  border: 1px solid $red-4
  transition: 1s

.meme-info
  border-bottom: 1px dashed $grey-4
  padding: 4px 0 0 0
  .label
    width: 180px
  .change
    font-size: 12px
    margin-left: 6px
</style>
