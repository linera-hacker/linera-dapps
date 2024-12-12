package token

import (
	"context"
	"fmt"

	tokenproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/token"
	tokencrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/token"
	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	tokenent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/token"
)

func (h *Handler) UpdateToken(ctx context.Context) (*tokenproto.Token, error) {
	if h.ID == nil {
		return nil, fmt.Errorf("invalid id")
	}

	err := db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		info, err := tx.
			Token.
			Query().
			Where(
				tokenent.ID(*h.ID),
			).
			Only(_ctx)
		if err != nil {
			return err
		}

		stm, err := tokencrud.UpdateSet(
			info.Update(),
			&tokencrud.Req{
				Address: h.Address,
				Site:    h.Site,
				Icon:    h.Icon,
				Name:    h.Name,
				Symbol:  h.Symbol,
			},
		)
		if err != nil {
			return err
		}
		if _, err := stm.Save(_ctx); err != nil {
			return err
		}
		return nil
	})
	if err != nil {
		return nil, err
	}

	return h.GetToken(ctx)
}
