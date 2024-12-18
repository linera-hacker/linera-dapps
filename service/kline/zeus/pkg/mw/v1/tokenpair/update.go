package tokenpair

import (
	"context"
	"fmt"

	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent"
)

func (h *Handler) UpdateTokenPair(ctx context.Context) error {
	info, err := h.GetTokenPair(ctx)
	if err != nil {
		return err
	}

	if info == nil {
		return fmt.Errorf("invalid id or ent_id")
	}

	sqlH := h.newSQLHandler()
	sqlH.BondTokenZeroID = &info.TokenZeroID
	sqlH.BondTokenOneID = &info.TokenOneID

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
			return fmt.Errorf("failed to update tokenpair: %v", err)
		}
		return nil
	})
}
