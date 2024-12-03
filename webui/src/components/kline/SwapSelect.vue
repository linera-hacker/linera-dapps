<template>
  <div class='bg-white vertical-card-padding'>
    <q-select dense v-model='swapStore.SelectedToken' :options='swapStore.Tokens' dropdown-icon='bi-chevron-down' class='swap-token-option'>
      <template #option='scope'>
        <q-item dense v-bind='scope.itemProps'>
          <q-img :src='scope.opt.Icon' width='24px' height='24px' />
          <div class='swap-token-list'>
            <div class='row'>
              <div class='swap-token-name text-bold'>{{ scope.opt.Symbol }}</div>
              <q-space />
            </div>
            <div>{{ shortid.shortId(scope.opt.Name, 10) }}</div>
          </div>
        </q-item>
      </template>
      <template #selected>
        <div class='row'>
          <q-img :src='swapStore.SelectedToken?.Icon' width='24px' height='24px' />
          <div class='swap-token-name text-bold swap-token-label'>{{ swapStore.SelectedToken?.Symbol }}</div>
        </div>
      </template>
    </q-select>
    <div class='separator'>
      /
    </div>
    <q-select dense v-model='swapStore.SelectedTokenPair' :options='swapStore.TokenPairs' dropdown-icon='bi-chevron-down' class='swap-token-option'>
      <template #option='scope'>
        <q-item dense v-bind='scope.itemProps'>
          <q-img :src='scope.opt.TokenOneIcon' width='24px' height='24px' />
          <div class='swap-token-list'>
            <div class='row'>
              <div class='swap-token-name text-bold'>{{ scope.opt.TokenOneSymbol }}</div>
              <q-space />
            </div>
            <div>{{ shortid.shortId(scope.opt.TokenOneName, 10) }}</div>
          </div>
        </q-item>
      </template>
      <template #selected>
        <div class='row'>
          <q-img :src='swapStore.SelectedToken?.Icon' width='24px' height='24px' />
          <div class='swap-token-name text-bold swap-token-label'>{{ swapStore.SelectedTokenPair?.TokenOneSymbol }}</div>
        </div>
      </template>
    </q-select>
  </div>
</template>

<script setup lang='ts'>
import { useSwapStore } from 'src/mystore/swap'
import { shortid } from 'src/utils'
import { onMounted, watch } from 'vue'
import { useRouter } from 'vue-router'

const swapStore = useSwapStore()

const router = useRouter()

const setSpecifyTokenPair = () => {
  const t0Addr = router.currentRoute.value.query.token0
  const t1Addr = router.currentRoute.value.query.token1
  if (t0Addr === undefined || t1Addr === undefined) {
    return
  }

  swapStore.SelectedToken = null
  for (const item of swapStore.Tokens) {
    if (item.Address === t0Addr) {
      swapStore.SelectedToken = item
      break
    }
  }

  swapStore.getTokenPairsByTokenZeroID()
  swapStore.SelectedTokenPair = null
  for (const item of swapStore.TokenPairs) {
    if (item.TokenOneAddress === t1Addr) {
      swapStore.SelectedTokenPair = { ...item }
    }
  }
}

watch(() => swapStore.SelectedToken, (selected) => {
  if (selected === null) {
    return
  }
  swapStore.getTokenPairsByTokenZeroID()
})

onMounted(() => {
  if (swapStore.IsInitilazed) {
    return
  }
  swapStore.IsInitilazed = true
  swapStore.getTokens((error) => {
    if (!error) {
      setSpecifyTokenPair()
    }
  })
})

</script>

<style scoped lang='sass'>
.swap-token-name
  line-height: 24px

:deep(.swap-token)
  .q-select
    .q-icon
      font-size: 16px

.swap-token-option
  display: inline-block
  border-radius: 4px

.swap-token-label
  margin-left: 3px
  overflow: hidden

.separator
  display: inline-block
  font-size: 24px
  margin-left: 15px
  margin-right: 15px
  font-weight: bolder
  color: #aaa
</style>
