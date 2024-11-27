package kprice

import (
	"context"

	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
)

func (h *Handler) CreateKPrice(ctx context.Context) error {
	sqlH := h.newSQLHandler()

	return db.WithClient(ctx, func(ctx context.Context, cli *ent.Client) error {
		sql, err := sqlH.genCreateSQL()
		if err != nil {
			return err
		}
		_, err = cli.ExecContext(ctx, sql)
		if err != nil {
			return err
		}

		if err != nil {
			return err
		}

		return nil
	})
}
