//nolint:dupl
package kpoint

import (
	"context"
	"time"

	"github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"
	crud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/kpoint"

	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
)

type deleteHandler struct {
	*Handler
	info *kpoint.KPoint
}

func (h *deleteHandler) deleteKPointBase(ctx context.Context, tx *ent.Tx) error {
	now := uint32(time.Now().Unix())
	updateOne, err := crud.UpdateSet(tx.KPoint.UpdateOneID(h.info.ID), &crud.Req{DeletedAt: &now})
	if err != nil {
		return err
	}
	_, err = updateOne.Save(ctx)
	if err != nil {
		return err
	}

	return nil
}

func (h *Handler) DeleteKPoint(ctx context.Context) error {
	handler := deleteHandler{Handler: h}
	var err error

	handler.info, err = handler.GetKPoint(ctx)
	if err != nil {
		return err
	}

	if handler.info == nil {
		return nil
	}

	return db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		if err := handler.deleteKPointBase(ctx, tx); err != nil {
			return err
		}
		return nil
	})
}
