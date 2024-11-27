package tokenpair

import (
	"context"

	tokenpaircrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/tokenpair"
	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	tokenpairent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/tokenpair"
)

func (h *Handler) ExistTokenPair(ctx context.Context) (bool, error) {
	exist := false
	var err error

	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		exist, err = cli.
			TokenPair.
			Query().
			Where(
				tokenpairent.ID(*h.ID),
				tokenpairent.DeletedAt(0),
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

func (h *Handler) ExistTokenPairConds(ctx context.Context) (bool, error) {
	exist := false
	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		stm, err := tokenpaircrud.SetQueryConds(cli.TokenPair.Query(), h.Conds)
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
