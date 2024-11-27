export interface TokenMetadata {
  logo: string
  discord: string
  telegram: string
  twitter: string
  website: string
  description: string
  github: string
  mintable: boolean
}

export interface MemeInfo {
  appID: string
  link: string
  totalSupply: string
  name: string
  symbol: string
  decimals: string
  tokenMetadata: TokenMetadata
  balanceOf: string
  allowance: string
  poolId: string
}

export interface NewMemeInfo {
  totalSupply: string
  name: string
  symbol: string
  decimals: string
  logo: string
  discord: string
  telegram: string
  twitter: string
  website: string
  github: string
  description: string
  mintable: boolean
  initialSupply: string
  initialCurrency: string
  fixedCurrency: boolean
  feePercent: string
}

export interface InitPoolLiquidity {
  amount0Initial: string
  amount1Initial: string
  amount0Virtual: string
  amount1Virtual: string
}

export interface ChainApp {
  id: string
  link: string
}
