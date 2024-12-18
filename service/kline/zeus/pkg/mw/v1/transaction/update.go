package transaction

import (
	"context"
	"fmt"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
)

func (h *Handler) UpdateTransaction(ctx context.Context) error {
	info, err := h.GetTransaction(ctx)
	if err != nil {
		return err
	}

	if info == nil {
		return fmt.Errorf("invalid id or id")
	}

	sqlH := h.newSQLHandler()
	sqlH.BondTransactionID = &info.TransactionID

	return db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		sql, err := sqlH.genUpdateSQL()
		if err != nil {
			return err
		}

		rc, err := tx.ExecContext(ctx, sql)
		if err != nil {
			return err
		}

		if n, err := rc.RowsAffected(); err != nil || n != 1 {
			return fmt.Errorf("failed to update transaction: %v", err)
		}
		return nil
	})
}
