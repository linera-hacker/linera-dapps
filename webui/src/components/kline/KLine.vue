<template>
  <div>
    <div style='padding: 5px;width: 100%;'>
      <div class='token-pair-tip'>
        <img :src='swapStore.SelectedTokenPair.TokenZeroIcon'>
        <img :src='swapStore.SelectedTokenPair.TokenOneIcon'>
        <div>
          {{ swapStore.SelectedTokenPair.TokenZeroSymbol }} / {{ swapStore.SelectedTokenPair.TokenOneSymbol }}
        </div>
      </div>
      <div class='radio-buttons-tip'>
        <div class='radio-buttons'>
          <div class='radio-button' v-for='(val,idx) in swapStore.KPointTypes' :key='idx' :value='val.KPointType'>
            <input
              class='radio-input'
              type='radio'
              :id='val.KPointType'
              :value='val.KPointType'
              v-model='selectedKPType'
              :checked='idx==0'
            >
            <label class='radio-lable' :for='val.KPointType'>{{ val.ShortName }}</label>
          </div>
        </div>
      </div>
    </div>
    <div id='chart-container' />
  </div>
</template>

<script setup lang='ts'>
import { ref, onMounted, watch } from 'vue'
import { useSwapStore } from 'src/mystore/swap'
import { initEchart, setKPointsToEchart, setStartAndEnd, calculateZoomStart } from './KLineOption'
import * as echarts from 'echarts/core'
import { useKLineStore } from 'src/mystore/kline'

const selectedKPType = ref('')
const swapStore = useSwapStore()
const klineStore = useKLineStore()
let myChart: echarts.ECharts

watch(() => swapStore.KPointTypes, (selected) => {
  if (selected === null || selected.length === 0) {
    return
  }
  selectedKPType.value = selected[0].KPointType
})

watch(selectedKPType, (selected) => {
  klineStore.SelectedKPType = selected
  initKPointsStore()
})

watch(() => swapStore.SelectedTokenPair, (selected) => {
  klineStore.SelectedTokenPairID = selected.ID
  initKPointsStore()
})

watch(klineStore.EchartPoinsData, () => {
  setKPointsToEchart(myChart, klineStore.EchartPoinsData)
  if (klineStore.ResetKLineViewLock + 60000 < new Date().getTime()) {
    setStartAndEnd(myChart, calculateZoomStart(klineStore.EchartPoinsData.CategoryItems.length), 100)
  }
}, { deep: true })

const initKPointsStore = () => {
  klineStore.OriginalTime = 0
  klineStore.EchartPoinsData.CategoryItems = []
  klineStore.EchartPoinsData.Nums = []
  klineStore.ResetKLineViewLock = 0
  klineStore.refreshHistoryKPoints()
}

interface eventParams {
  type: string
  start?: number
  end?: number
  batch?: {
    start: number
    end: number
  }[]
}

onMounted(() => {
  myChart = initEchart('chart-container')
  myChart.on('datazoom', (params) => {
    const _params = params as eventParams
    const start: number | undefined = _params.start || _params.batch?.[0].start
    const end: number | undefined = _params.end || _params.batch?.[0].end
    if (start === undefined || start < 1) {
      klineStore.refreshHistoryKPoints()
      setStartAndEnd(myChart, 1, end || 0)
    }
    klineStore.ResetKLineViewLock = new Date().getTime()
  })

  if (!klineStore.NeedInitKLine) {
    setKPointsToEchart(myChart, klineStore.EchartPoinsData)
    setStartAndEnd(myChart, calculateZoomStart(klineStore.EchartPoinsData.CategoryItems.length), 100)
    return
  }
  klineStore.NeedInitKLine = false

  swapStore.getKPointTypes()
  // update kline
  setInterval(() => {
    klineStore.refreshNewKPoints()
    if (klineStore.ResetKLineViewLock + 60000 < new Date().getTime()) {
      klineStore.ResetKLineViewLock = new Date().getTime()
    }
  }, 6000)
})
</script>

<style scoped lang="sass">
*
  margin: 0
  padding: 0

#chart-container
  position: relative
  height: 50vh
  min-height: 700px
  display: block
  overflow: auto

.token-pair-tip
  width: 63%
  display: inline-block
  vertical-align: middle

.token-pair-tip img
  width: 1.5rem
  border: 2px solid #dadada
  border-radius: 0.7rem
  display: inline-block
  vertical-align: middle

.token-pair-tip div
  margin-left: 5px
  font-size: 0.9rem
  font-weight: bold
  color: #555
  display: inline-block
  vertical-align: middle

.radio-buttons-tip
  width: 37%
  display: inline-block
  vertical-align: middle
  text-align: right

.radio-buttons
  display: inline-block
  padding: 2px
  background-color: #dadada
  border-radius: 5px

.radio-buttons:hover *
  cursor: pointer

.radio-button
  display: inline-block

.radio-input
  display: none

.radio-lable
  width: 2rem
  margin: 1px
  font-size: 0.8rem
  border-radius: 3px
  background-color: #e5e5e5
  text-align: center
  display: inline-block
  color: gray

.radio-input:checked+label
  display: inline-block
  color: black
  background-color: #eee
  font-weight: bold
</style>
