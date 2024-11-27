package kpoint

import (
	"context"

	kpointcrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/kpoint"
	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	kpointent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/kpoint"
)

func (h *Handler) ExistKPoint(ctx context.Context) (bool, error) {
	exist := false
	var err error

	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		exist, err = cli.
			KPoint.
			Query().
			Where(
				kpointent.ID(*h.ID),
				kpointent.DeletedAt(0),
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

func (h *Handler) ExistKPointConds(ctx context.Context) (bool, error) {
	exist := false
	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		stm, err := kpointcrud.SetQueryConds(cli.KPoint.Query(), h.Conds)
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
