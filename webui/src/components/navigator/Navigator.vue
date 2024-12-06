<template>
  <div class='row items-center'>
    <div class='cursor-pointer'>
      <q-img :src='selectedIcon' width='36px' height='36px' />
    </div>
    <q-space />
    <q-tabs
      v-model='tab'
      class='text-black'
      narrow-indicator
      dense indicator-color='red-6'
    >
      <q-tab name='meme' label='meme' />
      <q-tab name='swap' label='swap' />
      <q-tab name='blob' label='blob' />
    </q-tabs>
    <q-btn
      flat label='create meme token' class='text-red-6 border-red-4' rounded
      @click='onCreateMemeTokenClick'
    />
    <ConnectWallet />
  </div>
</template>

<script setup lang='ts'>
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import swap from 'src/assets/Swap.svg'
import meme from 'src/assets/Meme.svg'

import ConnectWallet from './ConnectWallet.vue'

const route = useRoute()
const router = useRouter()
const path = computed(() => route.path)

const tab = computed({
  get: () => path.value.includes('meme') ? 'meme' : path.value.includes('swap') ? 'swap' : 'blob',
  set: (v) => {
    void router.push({ path: '/' + v })
  }
})
const selectedIcon = computed(() => tab.value === 'meme' ? meme : swap)

const onCreateMemeTokenClick = () => {
  void router.push({ path: '/create/meme' })
}

</script>

<style scoped lang='sass'>
</style>
