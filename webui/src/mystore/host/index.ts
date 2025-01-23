import { defineStore } from 'pinia'

export const useHostStore = defineStore('hosts', {
  state: () => ({
    useDomainApi: false,
    apiSchema: 'http',
    amsDomainApiHost: 'api.linerameme.fun',
    klineDomainApiHost: 'api.linerameme.fun',
    blobGatewayDomainApiHost: 'api.linerameme.fun',
    swapDomainApiHost: 'api.linerameme.fun',

    amsDebugApiHost: '172.16.31.42:30094',
    klineDebugApiHost: '172.16.31.42:30100',
    blobGatewayDebugApiHost: '172.16.31.42:9081',
    swapDebugApiHost: '172.16.31.42:30092',

    erc20BytecodeId: 'e17e44525efd606bc3a0cad0f878de9810a1749401d71684e7890232f2c1e2080a437cd9615a5239bc7d336cce13ad879a459f2e0a8d3fdc6bf83ac3cb4ebf2c',
    swapCreationChainId: 'be20093606a7296fbda537060becfecc62b5441fa784b3d26d6742152a80a1f9',
    swapCreationOwner: 'e349827852196d4f5a4fbea95cfe118d683ec47ced11adb3a0d27ff961f0e92a',
    swapApplicationId: '2866ea0ae6589f359be49ed6e91a07cece93163d146f455c18a15bbc6dcccf5a1690502a30feb0c7ba184272c183ab9a7dfcfbf400cb5f146b0a7b2ace2d8515be20093606a7296fbda537060becfecc62b5441fa784b3d26d6742152a80a1f9050000000000000000000000',
    wlineraApplicationId: 'e17e44525efd606bc3a0cad0f878de9810a1749401d71684e7890232f2c1e2080a437cd9615a5239bc7d336cce13ad879a459f2e0a8d3fdc6bf83ac3cb4ebf2c36137b1ddac30eb9d0d4821a1560e66795dbdad6dbf4b708a8d81ca3df866e1b050000000000000000000000',
    amsCreationChainId: 'a393137daba303e8b561cb3a5bff50efba1fb7f24950db28f1844b7ac2c1cf27',
    amsApplicationId: '19da788bdf56d68baf06719ade3262271ba4ce692609a81d75d44d87a1cb8d3440941f100db894ad191828ec8952baa68b9da2e4223707f798a32547aee2c32aa393137daba303e8b561cb3a5bff50efba1fb7f24950db28f1844b7ac2c1cf27010000000000000000000000',
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
        return this.apiSchema + '://' + (this.useDomainApi ? this.klineDomainApiHost : this.klineDebugApiHost) + '/' + path
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
    blobDataPath (): (storeType: string, blobHash: string) => string {
      return (storeType: string, blobHash: string) => {
        if (storeType === 'S3') {
          return `http://minio.respeer.ai/api/file/v1/images/${blobHash}`
        }
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
