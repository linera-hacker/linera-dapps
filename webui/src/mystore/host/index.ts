import { defineStore } from 'pinia'

export const useHostStore = defineStore('hosts', {
  state: () => ({
    useDomainApi: process.env.NODE_ENV === 'production',
    apiSchema: process.env.NODE_ENV === 'production' ? 'https' : 'http',

    amsDomainApiHost: 'testnet-archimedes.respeer.ai/api/ams',
    klineDomainApiHost: 'testnet-archimedes.lineraswap.fun/api/kline',
    blobGatewayDomainApiHost: 'testnet-archimedes.blobgateway.com/api/blobs',
    swapDomainApiHost: 'testnet-archimedes.lineraswap.fun/api/swap',

    windowHost: 'localhost:8080',

    amsDebugApiHost: '/api/ams',
    klineDebugApiHost: '/api/kline',
    blobGatewayDebugApiHost: '/api/blobs',
    swapDebugApiHost: '/api/swap',

    erc20BytecodeId: '716c598da8db64bd276d6efbae5c67842c9caf16eade6587d51b0ff001e4a7b407b5e65376146b60763dfc1776a4353e15e3333f7ba53ccc88756f2b0d6308d2',
    swapCreationChainId: '2acc5e06468eb9d1de8f9234815e42e548e94f80c094b737427160ccd8c5facd',
    swapCreationOwner: '88909469f61e4e3459de1b8d4524bb5d3ff148d6a0c71a50285752c409697389',
    swapApplicationId: 'ca30b7409bfa4a373c2597ca63568ce698679c53ddeef9cc033aba955d2823b5f332095b5e2bb1757c03fd11fa58e930c7f6e28cc5cea0309f21db1dca210a222acc5e06468eb9d1de8f9234815e42e548e94f80c094b737427160ccd8c5facd060000000000000000000000',
    wlineraApplicationId: '716c598da8db64bd276d6efbae5c67842c9caf16eade6587d51b0ff001e4a7b407b5e65376146b60763dfc1776a4353e15e3333f7ba53ccc88756f2b0d6308d28058da81e9c23e69faec667c6711db3871ca445ca9e3ffd4cdc5a160a67b5d92060000000000000000000000',
    amsCreationChainId: 'dd595b39eab6263d40abd9ec883ec547b8a2962eb8fb6848f2a6be479fa7e918',
    amsApplicationId: '61afe169cf65c3798ebe9f968f0317bfb37a1087a131efa180308ba3d82e8ba53f73938f84203fb4f8cbc8f0a0eab6101872be430b536c756d5bfa7855d2a868dd595b39eab6263d40abd9ec883ec547b8a2962eb8fb6848f2a6be479fa7e918030000000000000000000000',
    blobGatewayCreationChainId: '088f922c7178abb39627756ac4001e5c7a99b45eed1db3a1741a73798f04d797',
    blobGatewayApplicationId: '1bb9526a4624c2c29623e5cb699cf52315f77337055de65349b8116e212828db7103993ccfcf753b78737131616f7aac2d001b26de2da03374ddbba0aeca0548088f922c7178abb39627756ac4001e5c7a99b45eed1db3a1741a73798f04d7970f0000000000000000000000'
  }),
  getters: {
    formalizeAmsPath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.amsDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/ams') : this.amsDebugApiHost) + '/' + path
      }
    },
    formalizeKlinePath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.klineDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/kline') : this.klineDebugApiHost) + '/' + path
      }
    },
    formalizeBlobGatewayPath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.blobGatewayDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/blobs') : this.blobGatewayDebugApiHost) + '/' + path
      }
    },
    formalizeSwapPath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.swapDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/swap') : this.swapDebugApiHost) + '/' + path
      }
    },
    _amsEndpoint (): () => string {
      return () => {
        return this.apiSchema + '://' + (this.useDomainApi ? this.amsDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/ams') : this.amsDebugApiHost)
      }
    },
    _swapEndpoint (): () => string {
      return () => {
        return this.apiSchema + '://' + (this.useDomainApi ? this.swapDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/swap') : this.swapDebugApiHost)
      }
    },
    blobGatewayApplicationPath (): () => string {
      return () => {
        return this.formalizeBlobGatewayPath(`/chains/${this.blobGatewayCreationChainId}/applications/${this.blobGatewayApplicationId}`)
      }
    },
    blobDataPath (): (blobHash: string) => string {
      return (blobHash: string) => {
        return this.blobGatewayApplicationPath() + `/images/${blobHash}`
      }
    },
    swapApplicationTokenPath (): (tokenApplicationId: string) => string {
      return (tokenApplicationId: string) => {
        return this.formalizeSwapPath(`/chains/${this.swapCreationChainId}/applications/${tokenApplicationId}`)
      }
    },
    swapApplicationPath (): () => string {
      return () => {
        return this.formalizeSwapPath(`/chains/${this.swapCreationChainId}/applications/${this.swapApplicationId}`)
      }
    }
  },
  actions: {}
})
