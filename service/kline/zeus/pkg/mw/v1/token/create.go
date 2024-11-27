package token

import (
	"context"

	tokenproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/token"
	tokencrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/token"
	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

func (h *Handler) CreateToken(ctx context.Context) (*tokenproto.Token, error) {
	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		info, err := tokencrud.CreateSet(
			cli.Token.Create(),
			&tokencrud.Req{
				Address: h.Address,
				Site:    h.Site,
				Icon:    h.Icon,
				Name:    h.Name,
				Symbol:  h.Symbol,
			},
		).Save(ctx)
		if err != nil {
			return err
		}
		h.ID = &info.ID
		return nil
	})
	if err != nil {
		return nil, err
	}

	return h.GetToken(ctx)
}

func (h *Handler) CreateTokens(ctx context.Context) ([]*tokenproto.Token, error) {
	ids := []uint32{}

	err := db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		for _, req := range h.Reqs {
			info, err := tokencrud.CreateSet(tx.Token.Create(), req).Save(_ctx)
			if err != nil {
				return err
			}
			ids = append(ids, info.ID)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}

	h.Conds = &tokencrud.Conds{
		IDs: &cruder.Cond{Op: cruder.IN, Val: ids},
	}
	h.Offset = 0
	h.Limit = int32(len(ids))

	infos, _, err := h.GetTokens(ctx)
	return infos, err
}
