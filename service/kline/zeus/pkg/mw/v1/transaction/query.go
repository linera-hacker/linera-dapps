package transaction

import (
	"context"
	"fmt"

	"entgo.io/ent/dialect/sql"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	transactionproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	transactionent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/transaction"

	transactioncrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/transaction"
)

type queryHandler struct {
	*Handler
	stm   *ent.TransactionSelect
	infos []*transactionproto.Transaction
}

func (h *queryHandler) selectTransaction(stm *ent.TransactionQuery) {
	h.stm = stm.Select(
		transactionent.FieldID,
		transactionent.FieldCreatedAt,
		transactionent.FieldUpdatedAt,
		transactionent.FieldPoolID,
		transactionent.FieldTransactionID,
		transactionent.FieldTransactionType,
		transactionent.FieldChainID,
		transactionent.FieldOwner,
		transactionent.FieldAmountZeroIn,
		transactionent.FieldAmountOneIn,
		transactionent.FieldAmountZeroOut,
		transactionent.FieldAmountOneOut,
		transactionent.FieldTimestamp,
	)
}

func (h *queryHandler) queryTransaction(cli *ent.Client) error {
	if h.ID == nil {
		return fmt.Errorf("invalid id")
	}
	stm := cli.Transaction.Query().Where(transactionent.DeletedAt(0))
	if h.ID != nil {
		stm.Where(transactionent.ID(*h.ID))
	}
	h.selectTransaction(stm)
	return nil
}

func (h *queryHandler) queryTransactions(ctx context.Context, cli *ent.Client) error {
	stm, err := transactioncrud.SetQueryConds(cli.Transaction.Query(), h.Conds)
	if err != nil {
		return err
	}

	h.selectTransaction(stm)
	return nil
}

func (h *queryHandler) scan(ctx context.Context) error {
	return h.stm.Scan(ctx, &h.infos)
}

func (h *Handler) GetTransaction(ctx context.Context) (*transactionproto.Transaction, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTransaction(cli); err != nil {
			return err
		}
		const singleRowLimit = 2
		handler.stm.Offset(0).Limit(singleRowLimit)
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}
	if len(handler.infos) == 0 {
		return nil, nil
	}
	if len(handler.infos) > 1 {
		return nil, fmt.Errorf("too many record")
	}

	return handler.infos[0], nil
}

//nolint:dupl
func (h *Handler) GetTransactions(ctx context.Context) ([]*transactionproto.Transaction, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTransactions(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(transactionent.FieldUpdatedAt))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}
	return handler.infos, nil
}

//nolint:dupl
func (h *Handler) GetEarlistTransactions(ctx context.Context) ([]*transactionproto.Transaction, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTransactions(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Asc(transactionent.FieldTransactionID))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}
	return handler.infos, nil
}

//nolint:dupl
func (h *Handler) GetLatestTransactions(ctx context.Context) ([]*transactionproto.Transaction, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTransactions(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(transactionent.FieldTransactionID))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}
	return handler.infos, nil
}

func (h *Handler) GetLastTransaction(ctx context.Context) (*transactionproto.Transaction, error) {
	var tx *transactionproto.Transaction
	var err error
	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		tx, err = getLastTransaction(ctx, cli)
		return err
	})
	if err != nil {
		return nil, err
	}
	return tx, nil
}

//nolint:lll
func getLastTransaction(ctx context.Context, cli *ent.Client) (*transactionproto.Transaction, error) {
	countVolumnSQL := "SELECT id,created_at,updated_at,pool_id,transaction_id,transaction_type,amount_zero_in,amount_one_in,amount_zero_out,amount_one_out,`timestamp` FROM  transactions WHERE transaction_id = (select max(transaction_id) from transactions);"
	txList := []*transactionproto.Transaction{}
	rows, err := cli.Transaction.QueryContext(ctx, countVolumnSQL)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	err = sql.ScanSlice(rows, &txList)
	if err != nil {
		return nil, err
	}

	if len(txList) == 0 {
		return nil, nil
	}

	return txList[0], nil
}

type TransactionVolumn struct {
	PoolID           uint64  `sql:"pool_id"`
	Volumn           uint32  `sql:"num_volumn"`
	AmountZeroVolumn float64 `sql:"amount_zero_volumn"`
	AmountOneVolumn  float64 `sql:"amount_one_volumn"`
}

func (h *Handler) GetVolumnFromTransaction(ctx context.Context, startTime, endTime uint32) ([]*TransactionVolumn, error) {
	ret := []*TransactionVolumn{}
	var err error
	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		ret, err = getVolumnFromTransaction(ctx, cli, startTime, endTime)
		return err
	})
	if err != nil {
		return nil, err
	}
	return ret, nil
}

func (h *Handler) GetVolumnFromTransactionByPoolID(ctx context.Context, startTime, endTime uint32, poolID uint64) (*TransactionVolumn, error) {
	ret := &TransactionVolumn{}
	var err error
	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		ret, err = getVolumnFromTransactionByPoolID(ctx, cli, startTime, endTime, poolID)
		return err
	})
	if err != nil {
		return nil, err
	}
	return ret, nil
}

//nolint:lll
func getVolumnFromTransactionByPoolID(ctx context.Context, cli *ent.Client, startTime, endTime uint32, poolID uint64) (*TransactionVolumn, error) {
	countVolumnSQL := fmt.Sprintf(
		"SELECT pool_id,count(*) as num_volumn,sum(amount_zero_in) as amount_zero_volumn,sum(amount_one_in) as amount_one_volumn FROM  transactions WHERE `timestamp`>=%v && `timestamp`<=%v  && pool_id='%v';",
		startTime,
		endTime,
		poolID,
	)
	_txVolumn := []*TransactionVolumn{}
	rows, err := cli.Transaction.QueryContext(ctx, countVolumnSQL)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	err = sql.ScanSlice(rows, &_txVolumn)
	if err != nil {
		return nil, err
	}

	if len(_txVolumn) == 0 {
		return nil, nil
	}

	return _txVolumn[0], nil
}

//nolint:lll
func getVolumnFromTransaction(ctx context.Context, cli *ent.Client, startTime, endTime uint32) ([]*TransactionVolumn, error) {
	countVolumnSQL := fmt.Sprintf(
		"SELECT pool_id,count(*) as num_volumn,sum(amount_zero_in) as amount_zero_volumn,sum(amount_one_in) as amount_one_volumn FROM  transactions WHERE `timestamp`>=%v && `timestamp`<=%v  GROUP BY pool_id;",
		startTime,
		endTime,
	)

	_txVolumn := []*TransactionVolumn{}
	rows, err := cli.Transaction.QueryContext(ctx, countVolumnSQL)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	err = sql.ScanSlice(rows, &_txVolumn)
	if err != nil {
		return nil, err
	}

	return _txVolumn, nil
}

func (h *Handler) GetTransactionsForLine(ctx context.Context) ([]*transactionproto.Transaction, error) {
	if h.Offset*h.Limit < 0 || h.Limit == 0 {
		return nil, fmt.Errorf("invalid offset and limit")
	}

	tx, err := h.GetLastTransaction(ctx)
	if err != nil {
		return nil, err
	}
	if tx == nil {
		return nil, nil
	}

	if h.OriginalTxID == nil || *h.OriginalTxID == 0 {
		h.OriginalTxID = &tx.TransactionID
	}

	h.Conds.TransactionID = &cruder.Cond{
		Op:  cruder.GTE,
		Val: *h.OriginalTxID,
	}

	forward := true
	if h.Limit < 0 {
		h.Limit = -h.Limit
		h.Offset = -h.Offset
		forward = false
		h.Conds.TransactionID.Op = cruder.LT
	}

	var transactions []*transactionproto.Transaction
	if forward {
		transactions, err = h.GetEarlistTransactions(ctx)
	} else {
		transactions, err = h.GetLatestTransactions(ctx)
	}

	if err != nil {
		return nil, err
	}

	return transactions, nil
}
