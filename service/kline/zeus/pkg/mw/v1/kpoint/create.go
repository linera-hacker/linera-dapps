package kpoint

import (
	"context"
	"fmt"

	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent"
)

func (h *Handler) CreateKPointWithCli(ctx context.Context, cli *ent.Client) error {
	sqlH := h.newSQLHandler()
	sql, err := sqlH.genCreateSQL()
	if err != nil {
		return err
	}
	rc, err := cli.ExecContext(ctx, sql)
	if err != nil {
		return err
	}

	if n, err := rc.RowsAffected(); err != nil || n != 1 {
		return fmt.Errorf("fail create kpoint: %v", err)
	}
	return nil
}

func (h *Handler) CreateKPoint(ctx context.Context) error {
	return db.WithClient(ctx, func(ctx context.Context, cli *ent.Client) error {
		return h.CreateKPointWithCli(ctx, cli)
	})
}
