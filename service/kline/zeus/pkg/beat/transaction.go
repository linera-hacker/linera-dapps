package beat

import (
	"context"
	"time"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/linera-hacker/linera-dapps/service/kline/config"
	transactionproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/applications"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/transaction"
)

type SamplingTransactionTask struct {
	closeChan chan struct{}
}

func GetSamplingTransactionTask(ctx context.Context) (*SamplingTransactionTask, error) {
	task := &SamplingTransactionTask{}

	return task, nil
}

func (st *SamplingTransactionTask) StartSampling(ctx context.Context, secounds uint32) {
	st.closeChan = make(chan struct{})
	for {
		select {
		case <-time.NewTimer(time.Second * time.Duration(secounds)).C:
			err := st.samplingAndStore(ctx)
			if err != nil {
				logger.Sugar().Error(err)
			}
		case <-ctx.Done():
			return
		case <-st.closeChan:
			return
		}
	}
}

func (st *SamplingTransactionTask) Close(ctx context.Context, interval time.Duration) {
	close(st.closeChan)
}

func createTransactions(ctx context.Context, transactionReqs []*transactionproto.TransactionReq) error {
	if len(transactionReqs) == 0 {
		return nil
	}
	nmcH, err := transaction.NewMultiCreateHandler(ctx, transactionReqs, true)
	if err != nil {
		return err
	}
	return nmcH.CreateTransactions(ctx)
}

func (st *SamplingTransactionTask) samplingAndStore(ctx context.Context) error {
	h, err := transaction.NewHandler(ctx)
	if err != nil {
		return err
	}
	tx, err := h.GetLastTransaction(ctx)
	if err != nil {
		return err
	}
	txID := uint64(0)
	if tx != nil {
		txID = tx.TransactionID + 1
	}

	txList := GetTransactions(txID)

	transactionReqs := []*transactionproto.TransactionReq{}
	for _, tx := range txList {
		transactionReqs = append(transactionReqs, &transactionproto.TransactionReq{
			PoolID:          &tx.PoolID,
			TransactionID:   &tx.TransactionID,
			TransactionType: &tx.TransactionType,
			ChainID:         &tx.ChainID,
			Owner:           &tx.Owner,
			AmountZeroIn:    &tx.AmountZeroIn,
			AmountOneIn:     &tx.AmountOneIn,
			AmountZeroOut:   &tx.AmountZeroOut,
			AmountOneOut:    &tx.AmountOneOut,
			Timestamp:       &tx.Timestamp,
		})
	}

	if err := createTransactions(ctx, transactionReqs); err != nil {
		return err
	}

	return nil
}

func RunSamplingTransaction(ctx context.Context) {
	samplingTask, err := GetSamplingTransactionTask(ctx)
	if err != nil {
		panic(err)
	}
	var intervalSecounds uint32 = 5
	samplingTask.StartSampling(ctx, intervalSecounds)
}

func GetTransactions(startTxID uint64) []*applications.Transaction {
	swapApp := applications.NewSwapApp(
		config.GetConfig().SwapApp.ServerAddr,
		config.GetConfig().SwapApp.ChainID,
		config.GetConfig().SwapApp.AppID,
	)
	resp, err := swapApp.GetTransactions(startTxID)
	if err != nil {
		logger.Sugar().Error(err)
		return nil
	}

	return resp
}
