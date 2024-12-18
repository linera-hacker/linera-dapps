//nolint:dupl
package transaction

import (
	"context"
	"time"

	"github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	crud "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/crud/v1/transaction"

	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent"
)

type deleteHandler struct {
	*Handler
	info *transaction.Transaction
}

func (h *deleteHandler) deleteTransactionBase(ctx context.Context, tx *ent.Tx) error {
	now := uint32(time.Now().Unix())
	updateOne, err := crud.UpdateSet(tx.Transaction.UpdateOneID(h.info.ID), &crud.Req{DeletedAt: &now})
	if err != nil {
		return err
	}
	_, err = updateOne.Save(ctx)
	if err != nil {
		return err
	}

	return nil
}

func (h *Handler) DeleteTransaction(ctx context.Context) error {
	handler := deleteHandler{Handler: h}
	var err error

	handler.info, err = handler.GetTransaction(ctx)
	if err != nil {
		return err
	}

	if handler.info == nil {
		return nil
	}

	return db.WithTx(ctx, func(_ctx context.Context, tx *ent.Tx) error {
		if err := handler.deleteTransactionBase(ctx, tx); err != nil {
			return err
		}
		return nil
	})
}
