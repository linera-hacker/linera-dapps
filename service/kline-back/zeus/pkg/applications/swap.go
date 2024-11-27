package applications

import (
	"strconv"
	"time"
)

type SwapApp struct {
	*baseApp
}

func NewSwapApp(serverAddr, chainID, appID string) *SwapApp {
	return &SwapApp{baseApp: newBaseApp(serverAddr, chainID, appID)}
}

func (app *SwapApp) GetReverves() (resp *GetReservesResponse, err error) {
	resp = &GetReservesResponse{}
	err = app.post(GetReservesReq, resp)
	return
}

func (app *SwapApp) GetTransactions(txID uint64) ([]*Transaction, error) {
	resp := &GetTransactionsResponse{}
	err := app.post(GetTransactionsReq(txID), resp)
	if err != nil {
		return nil, err
	}
	txList := []*Transaction{}
	for _, v := range resp.Data.Transactions {
		tx := Transaction{
			PoolID:          v.PoolID,
			TransactionID:   v.TransactionID,
			TransactionType: v.TransactionType,
		}
		a0in, err := strconv.ParseFloat(v.AmountZeroIn, 64)
		if err != nil {
			return nil, err
		}
		a1in, err := strconv.ParseFloat(v.AmountOneIn, 64)
		if err != nil {
			return nil, err
		}
		a0out, err := strconv.ParseFloat(v.AmountZeroOut, 64)
		if err != nil {
			return nil, err
		}
		a1out, err := strconv.ParseFloat(v.AmountOneOut, 64)
		if err != nil {
			return nil, err
		}

		tx.AmountZeroIn = a0in
		tx.AmountOneIn = a1in
		tx.AmountZeroOut = a0out
		tx.AmountOneOut = a1out
		tx.Timestamp = uint32(time.UnixMicro(int64(v.Timestamp)).Unix())
		txList = append(txList, &tx)
	}

	return txList, nil
}
