package transaction

import (
	"context"

	transactioncrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/transaction"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	transactionent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/transaction"
)

func (h *Handler) ExistTransaction(ctx context.Context) (bool, error) {
	exist := false
	var err error

	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		exist, err = cli.
			Transaction.
			Query().
			Where(
				transactionent.ID(*h.ID),
				transactionent.DeletedAt(0),
			).
			Exist(_ctx)
		if err != nil {
			return err
		}
		return nil
	})
	if err != nil {
		return false, err
	}
	return exist, nil
}

func (h *Handler) ExistTransactionConds(ctx context.Context) (bool, error) {
	exist := false
	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		stm, err := transactioncrud.SetQueryConds(cli.Transaction.Query(), h.Conds)
		if err != nil {
			return err
		}
		exist, err = stm.Exist(_ctx)
		if err != nil {
			return err
		}
		return nil
	})
	if err != nil {
		return false, err
	}
	return exist, nil
}
