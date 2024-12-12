package kprice

import (
	"context"

	kpricecrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/kprice"
	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	kpriceent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/kprice"
)

func (h *Handler) ExistKPrice(ctx context.Context) (bool, error) {
	exist := false
	var err error

	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		exist, err = cli.
			KPrice.
			Query().
			Where(
				kpriceent.ID(*h.ID),
				kpriceent.DeletedAt(0),
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

func (h *Handler) ExistKPriceConds(ctx context.Context) (bool, error) {
	exist := false
	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		stm, err := kpricecrud.SetQueryConds(cli.KPrice.Query(), h.Conds)
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
