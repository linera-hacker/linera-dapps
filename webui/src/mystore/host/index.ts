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

    erc20BytecodeId: 'cf93b5a22a5566dc7de174d5c50dced2e62233b53ee0e819142ecfbc5ccdc85244aaf2cf0b1d21fca099e545abc8fc8ca0de62833580307161cce65cb4ee7d9f',
    swapCreationChainId: 'cf0e414d12efddc9094d6a476770923f3a9b043e3f51e5bb8f8773dbc6566cac',
    swapCreationOwner: '1bcab2248a727f9611987289c99bbd3643767cc66b3d308b36fdbd43848bbeec',
    swapApplicationId: '',
    wlineraApplicationId: 'cf93b5a22a5566dc7de174d5c50dced2e62233b53ee0e819142ecfbc5ccdc85244aaf2cf0b1d21fca099e545abc8fc8ca0de62833580307161cce65cb4ee7d9fb3e95393aaf6c146e3ed05f78273f3cf3c9ebc26533d2b0344a3f7cc52b2edd3060000000000000000000000',
    amsCreationChainId: '71f735a287b728c0a19b83c26f7be5b508090acc00e35b657a69afb86859630d',
    amsApplicationId: 'bf9b75f4ad24a9940b1f0dc4dec6cbb89e72ca8b391ef4e8dd6357a84d28e556431d6cc87355d1323f1b0d3670c6d76c61c8f6bd686c19da747f8609cca719d071f735a287b728c0a19b83c26f7be5b508090acc00e35b657a69afb86859630d030000000000000000000000',
    blobGatewayCreationChainId: '2ed9e7125f528245d8a9e9ac8ca5a0e1f9b26e954bbd2f0f5b6ec6a573753722',
    blobGatewayApplicationId: 'cf2f040970768c6822c14c9a7183a17556b30d1d98cf202eec9fd1b2b3ba65a249de46874274296b257fdd8d5cb3db4c7dbc9aeffde4100be5f407293f140d432ed9e7125f528245d8a9e9ac8ca5a0e1f9b26e954bbd2f0f5b6ec6a5737537220f0000000000000000000000',
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
