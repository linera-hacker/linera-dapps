import { defineStore } from 'pinia'

export const useHostStore = defineStore('hosts', {
  state: () => ({
    useDomainApi: false,
    apiSchema: 'http',
    amsDomainApiHost: 'api.linerameme.fun',
    klineDomainApiHost: 'api.linerameme.fun',
    blobGatewayDomainApiHost: 'api.linerameme.fun',
    swapDomainApiHost: 'api.linerameme.fun',

    windowHost: 'localhost:8080',

    amsDebugApiHost: '172.16.31.42:30094',
    klineDebugApiHost: 'api.development.npool.top/api/kline',
    blobGatewayDebugApiHost: '172.16.31.42:9081',
    swapDebugApiHost: '172.16.31.42:30092',

    erc20BytecodeId: 'e17e44525efd606bc3a0cad0f878de9810a1749401d71684e7890232f2c1e2080a437cd9615a5239bc7d336cce13ad879a459f2e0a8d3fdc6bf83ac3cb4ebf2c',
    swapCreationChainId: '5c930675fb2c9374653a868de5f8e98e5a7be27703d01e40479fd1d00c0bc16d',
    swapCreationOwner: '387ab6a4fd17001853842fbf0ae6613941c28b1abf87b43a88d3edd51eeb6821',
    swapApplicationId: '2866ea0ae6589f359be49ed6e91a07cece93163d146f455c18a15bbc6dcccf5a1690502a30feb0c7ba184272c183ab9a7dfcfbf400cb5f146b0a7b2ace2d85155c930675fb2c9374653a868de5f8e98e5a7be27703d01e40479fd1d00c0bc16d050000000000000000000000',
    wlineraApplicationId: 'e17e44525efd606bc3a0cad0f878de9810a1749401d71684e7890232f2c1e2080a437cd9615a5239bc7d336cce13ad879a459f2e0a8d3fdc6bf83ac3cb4ebf2c90895b6b9ac022abec03fa2940547a551c6265bae5d9c7a95592e502e0a947d4050000000000000000000000',
    amsCreationChainId: '0c35b2a0f8fa4a4acc7766df9d5900bcdc6f45bb0439812d5d3b4993d56ee4b9',
    amsApplicationId: '19da788bdf56d68baf06719ade3262271ba4ce692609a81d75d44d87a1cb8d3440941f100db894ad191828ec8952baa68b9da2e4223707f798a32547aee2c32a0c35b2a0f8fa4a4acc7766df9d5900bcdc6f45bb0439812d5d3b4993d56ee4b9010000000000000000000000',
    blobGatewayCreationChainId: '1db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d03',
    blobGatewayApplicationId: 'b348f8039b02f685b8f90c147a650ea4ff590f74513c8080205836debcd7df6c69f4f4c55c769e3465e3f80c89c4d557a5ed748c1c41c1d2c19bf8e26389fbb31db1936dad0717597a7743a8353c9c0191c14c3a129b258e9743aec2b4f05d030d0000000000000000000000'
  }),
  getters: {
    formalizeAmsPath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.amsDomainApiHost : this.amsDebugApiHost) + '/' + path
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
        return this.apiSchema + '://' + (this.useDomainApi ? this.blobGatewayDomainApiHost : this.blobGatewayDebugApiHost) + '/' + path
      }
    },
    formalizeSwapPath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.swapDomainApiHost : this.swapDebugApiHost) + '/' + path
      }
    },
    _amsEndpoint (): () => string {
      return () => {
        return this.apiSchema + '://' + (this.useDomainApi ? this.amsDomainApiHost : this.amsDebugApiHost)
      }
    },
    _swapEndpoint (): () => string {
      return () => {
        return this.apiSchema + '://' + (this.useDomainApi ? this.swapDomainApiHost : this.swapDebugApiHost)
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
