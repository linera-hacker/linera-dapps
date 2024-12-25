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
    swapCreationChainId: '23011460ebc72efac0b7f73ccd5bcb5d12eda134c75054d55e90c8abc0b99dda',
    swapCreationOwner: 'd4128b4fd73b95d4e149e849dcda57f3ef090741b65a33ae9257a636452a0e08',
    swapApplicationId: 'ca30b7409bfa4a373c2597ca63568ce698679c53ddeef9cc033aba955d2823b5f332095b5e2bb1757c03fd11fa58e930c7f6e28cc5cea0309f21db1dca210a2223011460ebc72efac0b7f73ccd5bcb5d12eda134c75054d55e90c8abc0b99dda060000000000000000000000',
    wlineraApplicationId: '716c598da8db64bd276d6efbae5c67842c9caf16eade6587d51b0ff001e4a7b407b5e65376146b60763dfc1776a4353e15e3333f7ba53ccc88756f2b0d6308d2d92e5f94e7a4caf67c3ab283d3958fd0c870144a685c6fd51e897c299e6da621060000000000000000000000',
    amsCreationChainId: '860264780e386ac1c6b5a869f8df5a4e2b21f2bf6813f87001e873c0ff407d03',
    amsApplicationId: '61afe169cf65c3798ebe9f968f0317bfb37a1087a131efa180308ba3d82e8ba53f73938f84203fb4f8cbc8f0a0eab6101872be430b536c756d5bfa7855d2a868860264780e386ac1c6b5a869f8df5a4e2b21f2bf6813f87001e873c0ff407d03030000000000000000000000',
    blobGatewayCreationChainId: '8970f7d8f2eadd58ee22c33298266200bda9f6f4df8e4201fc395673d58c5761',
    blobGatewayApplicationId: '1d04907279078ae32532119efcf05ceaf3715c5ea0f7eed2c22046580c8e64efdcd98a9d4c4aba9730db87fc95e9b0a5d2b44edac6fa2a7f5c488e87ed9148af8970f7d8f2eadd58ee22c33298266200bda9f6f4df8e4201fc395673d58c57610f0000000000000000000000'
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
