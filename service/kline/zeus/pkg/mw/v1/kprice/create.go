package kprice

import (
	"context"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
)

func (h *Handler) CreateKPrice(ctx context.Context) error {
	return db.WithClient(ctx, func(ctx context.Context, cli *ent.Client) error {
		return h.CreateKPriceWithCli(ctx, cli)
	})
}

func (h *Handler) CreateKPriceWithCli(ctx context.Context, cli *ent.Client) error {
	sqlH := h.newSQLHandler()
	sql, err := sqlH.genCreateSQL()
	if err != nil {
		return err
	}
	_, err = cli.ExecContext(ctx, sql)
	if err != nil {
		return err
	}

	return nil
}
