import { defineStore } from 'pinia'

export const useHostStore = defineStore('hosts', {
  state: () => ({
    useDomainApi: process.env.NODE_ENV === 'production',
    apiSchema: process.env.NODE_ENV === 'production' ? 'https' : 'http',

    amsDomainApiHost: 'hk.testnet-archimedes.respeer.ai/api/ams',
    klineDomainApiHost: 'hk.testnet-archimedes.lineraswap.fun/api/kline',
    blobGatewayDomainApiHost: 'hk.testnet-archimedes.blobgateway.com/api/blobs',
    swapDomainApiHost: 'hk.testnet-archimedes.lineraswap.fun/api/swap',

    windowHost: 'localhost:8080',

    amsDebugApiHost: '/api/ams',
    klineDebugApiHost: '/api/kline',
    blobGatewayDebugApiHost: '/api/blobs',
    swapDebugApiHost: '/api/swap',

    erc20BytecodeId: '8f35d0a1a1ecec3406ce05f9b58809204c4b81a60512f42983950e077392eb03e09686709ef93e02de5689565008be6b57555877c8ea663fc039718d85f3ad2a',
    swapCreationChainId: '6e677651a05d090e15061875582a355b1d86b1156ab508e019060b925f979f3c',
    swapCreationOwner: '41ef4716e818fe9ec0acd638e4ba5c13fa78012e520da7bf11295d8e3600c6e2',
    swapApplicationId: '0a53c50a4012157bccba877b890b778f684b58da172605da49d357c732f1361eaabeee56a723e29481416b86a8ba18a4d7012dfda927d56339b96fdc3d25b89b6e677651a05d090e15061875582a355b1d86b1156ab508e019060b925f979f3c060000000000000000000000',
    wlineraApplicationId: '8f35d0a1a1ecec3406ce05f9b58809204c4b81a60512f42983950e077392eb03e09686709ef93e02de5689565008be6b57555877c8ea663fc039718d85f3ad2a525571956c2b3ded3e66e8dcbc42349af0e5639dcd949fe9f3be13b7e8601d01060000000000000000000000',
    amsCreationChainId: '051ede2da9b4f83554243d590656e4c1a9b032965a7b09b4b6af3043b29f802b',
    amsApplicationId: 'bf9b75f4ad24a9940b1f0dc4dec6cbb89e72ca8b391ef4e8dd6357a84d28e5566478527ea66be16313d7ec46334a8293b8254ffc87e1267f1bf6d2e899e220d1051ede2da9b4f83554243d590656e4c1a9b032965a7b09b4b6af3043b29f802b030000000000000000000000',
    blobGatewayCreationChainId: '1ed416a3a48a0bc76c53a29e35b55758e609a377da17e51e85e8dd09343fc1b8',
    blobGatewayApplicationId: '9e44878bc356390f539f3361720551e96c20c3f80566275d3a4e332c7425603b3b93ed75913ae0768755f68c161e57600cf4747b44e8301a1b8c037582571a191ed416a3a48a0bc76c53a29e35b55758e609a377da17e51e85e8dd09343fc1b80f0000000000000000000000'
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
