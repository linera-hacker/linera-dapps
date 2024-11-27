package kpoint

import (
	"context"
	"fmt"

	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
)

func (h *Handler) UpdateKPoint(ctx context.Context) error {
	info, err := h.GetKPoint(ctx)
	if err != nil {
		return err
	}

	if info == nil {
		return fmt.Errorf("invalid id or ent_id")
	}

	sqlH := h.newSQLHandler()
	kpType := info.KPointType.String()
	sqlH.BondKPointType = &kpType
	sqlH.BondEndTime = &info.EndTime
	sqlH.BondTokenPairID = &info.TokenPairID

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
			return fmt.Errorf("failed to update kpoint: %v", err)
		}
		return nil
	})
}
