<template>
  <div class='row vertical-card-align bg-grey-2'>
    <div class='kline'>
      <div class='bg-white'>
        <KLine />
      </div>
      <div class='bg-white history'>
        <Trades />
      </div>
    </div>
    <div>
      <div class='swap bg-white swap-padding'>
        <Swap />
      </div>
      <div class='swap bg-white swap-padding'>
        <AddLiquidity />
      </div>
      <div class='swap bg-white vertical-card-align bulletin-padding'>
        <VolumeBulletin />
      </div>
      <div class='swap bg-white vertical-card-align bulletin-padding'>
        <HolderBulletin />
      </div>
    </div>
  </div>
</template>

<script setup lang='ts'>
import { defineAsyncComponent } from 'vue'

import VolumeBulletin from 'src/components/bulletin/Volume.vue'
import HolderBulletin from 'src/components/bulletin/Holder.vue'

const KLine = defineAsyncComponent(() => import('src/components/kline/KLine.vue'))
const Swap = defineAsyncComponent(() => import('src/components/swap/Swap.vue'))
const Trades = defineAsyncComponent(() => import('src/components/trades/Trades.vue'))
const AddLiquidity = defineAsyncComponent(() => import('src/components/liquidity/AddLiquidity.vue'))
</script>

<style scoped lang='sass'>
.swap
  width: 360px
  margin-left: 4px

.swap-padding
  padding: 0 16px

.bulletin-padding
  padding: 16px

.kline
  width: calc(100% - 360px - 4px)

.history
  margin-top: 4px
</style>
