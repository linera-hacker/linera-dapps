import { defineStore } from 'pinia'

export const useHostStore = defineStore('hosts', {
  state: () => ({
    useDomainApi: process.env.NODE_ENV === 'production',
    apiSchema: process.env.NODE_ENV === 'production' ? 'https' : 'http',

    amsDomainApiHost: 'testnet-archimedes.respeer.ai/api/ams',
    klineDomainApiHost: 'testnet-archimedes.lineraswap.fun/api/kline',
    blobGatewayDomainApiHost: 'testnet-archimedes.blobgateway.com/api/blobs',
    swapDomainApiHost: 'testnet-archimedes.lineraswap.fun/api/swap',
    fileGatewayDomainApiHost: 'testnet-archimedes.blobgateway.com/api/file',

    windowHost: 'localhost:8080',

    amsDebugApiHost: '/api/ams',
    klineDebugApiHost: '/api/kline',
    blobGatewayDebugApiHost: '/api/blobs',
    swapDebugApiHost: '/api/swap',

    erc20BytecodeId: '8f35d0a1a1ecec3406ce05f9b58809204c4b81a60512f42983950e077392eb03e09686709ef93e02de5689565008be6b57555877c8ea663fc039718d85f3ad2a',
    swapCreationChainId: '7f430ede6a5fbf9a8aa59286fb94e3b3490c0451826583a6a98b20b8cba1a0ab',
    swapCreationOwner: '9ddb97dd256d6cd4efd58256c2b9afe926962321c733afcde34e7566ecc9459c',
    swapApplicationId: '0a53c50a4012157bccba877b890b778f684b58da172605da49d357c732f1361eaabeee56a723e29481416b86a8ba18a4d7012dfda927d56339b96fdc3d25b89b7f430ede6a5fbf9a8aa59286fb94e3b3490c0451826583a6a98b20b8cba1a0ab060000000000000000000000',
    wlineraApplicationId: '8f35d0a1a1ecec3406ce05f9b58809204c4b81a60512f42983950e077392eb03e09686709ef93e02de5689565008be6b57555877c8ea663fc039718d85f3ad2ad2040af57277cbc64d4d585535d0ec0a4e28fdbf58eba23c7d769673e541352c060000000000000000000000',
    amsCreationChainId: 'a7fcebee0c372daafeec2cdb76b34e32caa9d8be1d2d6e9d9d597d6a314d52a2',
    amsApplicationId: 'bf9b75f4ad24a9940b1f0dc4dec6cbb89e72ca8b391ef4e8dd6357a84d28e5566478527ea66be16313d7ec46334a8293b8254ffc87e1267f1bf6d2e899e220d1a7fcebee0c372daafeec2cdb76b34e32caa9d8be1d2d6e9d9d597d6a314d52a2030000000000000000000000',
    blobGatewayCreationChainId: 'e24f3d30875f3f7e6e9830536b98b48a9b0538ee0ffcf8bf0722f9b8a724b2f2',
    blobGatewayApplicationId: '1792c2f3699e48288081499b0455b494cb5883eb9423aa186c89714ed106c6043af42487fe70116e7375c88c4fb24743bd734d209f53b952937148e726fa7041e24f3d30875f3f7e6e9830536b98b48a9b0538ee0ffcf8bf0722f9b8a724b2f20f0000000000000000000000'
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
    formalizeFileGatewayPath (): (path: string) => string {
      return (path: string) => {
        if (path.startsWith('/')) path = path.substring(1)
        return this.apiSchema + '://' + (this.useDomainApi ? this.fileGatewayDomainApiHost : process.env.NODE_ENV === 'development' ? (this.windowHost + '/api/file') : this.fileGatewayDomainApiHost) + '/' + path
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
          return this.formalizeFileGatewayPath(`/v1/images/${blobHash}`)
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
