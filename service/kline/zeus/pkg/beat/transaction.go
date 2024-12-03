package beat

import (
	"context"
	"time"

	"github.com/Geapefurit/kline-back/config"
	transactionproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/transaction"
	"github.com/Geapefurit/kline-back/zeus/pkg/applications"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/transaction"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
)

type SamplingTransactionTask struct {
	closeChan chan struct{}
}

func GetSamplingTransactionTask(ctx context.Context) (*SamplingTransactionTask, error) {
	task := &SamplingTransactionTask{}

	return task, nil
}

func (st *SamplingTransactionTask) StartSampling(ctx context.Context, interval time.Duration) {
	st.closeChan = make(chan struct{})
	for {
		select {
		// try to start with whole seconds and offset 10 milliseconds
		case <-time.NewTicker(interval*time.Second + time.Millisecond*10 - time.Duration(time.Now().Nanosecond())%time.Second).C:
			go func() {
				err := st.samplingAndStore(ctx)
				if err != nil {
					logger.Sugar().Error(err)
				}
			}()
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
	for _, req := range transactionReqs {
		//TODO: will create bulk
		createH, err := transaction.NewHandler(ctx,
			transaction.WithPoolID(req.PoolID, true),
			transaction.WithTransactionID(req.TransactionID, true),
			transaction.WithTransactionType(req.TransactionType, true),
			transaction.WithAmountZeroIn(req.AmountZeroIn, false),
			transaction.WithAmountOneIn(req.AmountOneIn, false),
			transaction.WithAmountZeroOut(req.AmountZeroOut, false),
			transaction.WithAmountOneOut(req.AmountOneOut, false),
			transaction.WithTimestamp(req.Timestamp, true),
		)
		if err != nil {
			return err
		}
		if err := createH.CreateTransaction(ctx); err != nil {
			return err
		}
	}
	return nil
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
		txID = tx.TransactionID
	}

	txList := GetTransactions(txID)
	// txList := MockGetTransactions(txID)

	transactionReqs := []*transactionproto.TransactionReq{}
	for _, tx := range txList {
		transactionReqs = append(transactionReqs, &transactionproto.TransactionReq{
			PoolID:          &tx.PoolID,
			TransactionID:   &tx.TransactionID,
			TransactionType: &tx.TransactionType,
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
	samplingTask.StartSampling(ctx, 1)
}

func MockGetTransactions(startTxID *uint64) []*applications.Transaction {
	id := uint64(1)
	if startTxID == nil {
		startTxID = &id
	} else {
		id = *startTxID
	}

	ret := []*applications.Transaction{}
	for i := uint64(1); i < 5; i++ {
		id++
		ret = append(ret, &applications.Transaction{
			PoolID:          i % 5,
			TransactionID:   id,
			TransactionType: "Swap",
			AmountZeroIn:    float64(id),
			AmountOneIn:     float64(i),
			AmountZeroOut:   float64(i),
			AmountOneOut:    float64(id),
			Timestamp:       uint32(time.Now().Unix()),
		})
	}

	return ret
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
