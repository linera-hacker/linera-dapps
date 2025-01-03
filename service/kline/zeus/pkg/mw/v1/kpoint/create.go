package kpoint

import (
	"context"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
)

func (h *Handler) CreateKPointWithCli(ctx context.Context, cli *ent.Client) error {
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

func (h *Handler) CreateKPoint(ctx context.Context) error {
	return db.WithClient(ctx, func(ctx context.Context, cli *ent.Client) error {
		return h.CreateKPointWithCli(ctx, cli)
	})
}
