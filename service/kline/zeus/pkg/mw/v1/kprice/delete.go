//nolint:dupl
package kprice

import (
	"context"
	"time"

	"github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	crud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/kprice"

	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
)

type deleteHandler struct {
	*Handler
	info *kprice.KPrice
}

func (h *deleteHandler) deleteKPriceBase(ctx context.Context, tx *ent.Tx) error {
	now := uint32(time.Now().Unix())
	updateOne, err := crud.UpdateSet(tx.KPrice.UpdateOneID(h.info.ID), &crud.Req{DeletedAt: &now})
	if err != nil {
		return err
	}
	_, err = updateOne.Save(ctx)
	if err != nil {
		return err
	}

	return nil
}

func (h *Handler) DeleteKPrice(ctx context.Context) error {
	handler := deleteHandler{Handler: h}
	var err error

	handler.info, err = handler.GetKPrice(ctx)
	if err != nil {
		return err
	}

	if handler.info == nil {
		return nil
	}

	return db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		if err := handler.deleteKPriceBase(ctx, tx); err != nil {
			return err
		}
		return nil
	})
}
