package applications

import "fmt"

// ################### Swap App ####################

var GetReservesReq = `{"query":"query {\n  getPools {\n    id\n    token0\n    token1\n    reserve0\n    reserve1\n  }\n}"}`

type GetReservesResponse struct {
	Data Reserves `json:"data,omitempty"`
}

type Reserves struct {
	TokenPairReserves []TokenPairReserves `json:"getPools,omitempty"`
}

type TokenPairReserves struct {
	PoolID   uint64 `json:"id,omitempty"`
	Token0   string `json:"token0,omitempty"`
	Token1   string `json:"token1,omitempty"`
	Reserve0 string `json:"reserve0,omitempty"`
	Reserve1 string `json:"reserve1,omitempty"`
}

//nolint:lll
var GetTransactionsReq = func(startTxID uint64) string {
	return fmt.Sprintf(
		`{"query":"query{\n  getTransactions(startId: %v){\n  \ttransactionId\n    transactionType\n    poolId\n    owner\n    amount0In\n    amount1In\n    amount0Out\n    amount1Out\n    timestamp\n  }\n}"}`,
		startTxID,
	)
}

type ChainAccountOwner struct {
	ChainID string `json:"chain_id,omitempty"`
	Owner   string `json:"owner,omitempty"`
}

type Transaction struct {
	PoolID          uint64  `json:"poolId,omitempty"`
	TransactionID   uint64  `json:"transactionId,omitempty"`
	TransactionType string  `json:"transactionType,omitempty"`
	ChainID         string  `json:"chain_id,omitempty"`
	Owner           string  `json:"owner,omitempty"`
	AmountZeroIn    float64 `json:"amount0In,omitempty"`
	AmountOneIn     float64 `json:"amount1In,omitempty"`
	AmountZeroOut   float64 `json:"amount0Out,omitempty"`
	AmountOneOut    float64 `json:"amount1Out,omitempty"`
	Timestamp       uint32  `json:"timestamp,omitempty"`
}

type _Transaction struct {
	PoolID          uint64            `json:"poolId,omitempty"`
	TransactionID   uint64            `json:"transactionId,omitempty"`
	TransactionType string            `json:"transactionType,omitempty"`
	Owner           ChainAccountOwner `json:"owner,omitempty"`
	AmountZeroIn    string            `json:"amount0In,omitempty"`
	AmountOneIn     string            `json:"amount1In,omitempty"`
	AmountZeroOut   string            `json:"amount0Out,omitempty"`
	AmountOneOut    string            `json:"amount1Out,omitempty"`
	Timestamp       uint64            `json:"timestamp,omitempty"`
}

type GetTransactions struct {
	Transactions []*_Transaction `json:"getTransactions,omitempty"`
}

type GetTransactionsResponse struct {
	Data GetTransactions `json:"data,omitempty"`
}

// ################### ERC20 Token App ####################

var GetErc20InfoReq = `{"query":"query {\n  name\n  symbol\n  totalSupply\n  decimals\n  tokenMetadata\n}"}`

type GetErc20InfoResponse struct {
	Data Erc20Info `json:"data,omitempty"`
}

type Erc20Info struct {
	Name          string                 `json:"name,omitempty"`
	Symbol        string                 `json:"symbol,omitempty"`
	TotalSupply   string                 `json:"totalSupply,omitempty"`
	Decimals      int                    `json:"decimals,omitempty"`
	TokenMetadata map[string]interface{} `json:"tokenMetadata,omitempty"`
}
