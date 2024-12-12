package token

import (
	"context"
	"fmt"
	"time"

	tokenproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/token"
	tokencrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/token"
	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
)

func (h *Handler) DeleteToken(ctx context.Context) (*tokenproto.Token, error) {
	if h.ID == nil {
		return nil, fmt.Errorf("invalid id")
	}

	info, err := h.GetToken(ctx)
	if err != nil {
		return nil, err
	}

	err = db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		now := uint32(time.Now().Unix())
		updateOne, err := tokencrud.UpdateSet(tx.Token.UpdateOneID(info.ID), &tokencrud.Req{DeletedAt: &now})
		if err != nil {
			return err
		}
		_, err = updateOne.Save(ctx)
		return err
	})
	if err != nil {
		return nil, err
	}

	return info, nil
}
