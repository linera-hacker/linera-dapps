<template>
  <div class='row items-center'>
    <div class='cursor-pointer'>
      <q-img
        :src='selectedIcon'
        height='36px'
        width='480px'
        fit='contain'
        position='0 0'
      />
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
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { blobGatewayLogo, lineraMemeLogo, lineraSwapLogo } from 'src/assets'

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
const selectedIcon = ref(lineraMemeLogo)

const onCreateMemeTokenClick = () => {
  void router.push({ path: '/create/meme' })
}

onMounted(() => {
  if (window.location.host.endsWith('linerameme.fun')) {
    selectedIcon.value = lineraMemeLogo
  } else if (window.location.host.endsWith('lineraswap.fun')) {
    selectedIcon.value = lineraSwapLogo
  } else if (window.location.host.endsWith('blobgateway.com')) {
    selectedIcon.value = blobGatewayLogo
  }
})

</script>

<style scoped lang='sass'>
</style>
