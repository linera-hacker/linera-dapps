//nolint:dupl
package tokenpair

import (
	"context"
	"time"

	"github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	crud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/tokenpair"

	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
)

type deleteHandler struct {
	*Handler
	info *tokenpair.TokenPair
}

func (h *deleteHandler) deleteTokenPairBase(ctx context.Context, tx *ent.Tx) error {
	now := uint32(time.Now().Unix())
	updateOne, err := crud.UpdateSet(tx.TokenPair.UpdateOneID(h.info.ID), &crud.Req{DeletedAt: &now})
	if err != nil {
		return err
	}
	_, err = updateOne.Save(ctx)
	if err != nil {
		return err
	}

	return nil
}

func (h *Handler) DeleteTokenPair(ctx context.Context) error {
	handler := deleteHandler{Handler: h}
	var err error

	handler.info, err = handler.GetTokenPair(ctx)
	if err != nil {
		return err
	}

	if handler.info == nil {
		return nil
	}

	return db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		if err := handler.deleteTokenPairBase(ctx, tx); err != nil {
			return err
		}
		return nil
	})
}
