package transaction

import (
	"context"
	"fmt"

	"github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
)

type MultiHandler struct {
	Handlers []*Handler
}

type MultiCreateHandler struct {
	*MultiHandler
}

func (h *MultiHandler) AppendHandler(handler *Handler) {
	h.Handlers = append(h.Handlers, handler)
}

func (h *MultiHandler) GetHandlers() []*Handler {
	return h.Handlers
}

func NewMultiCreateHandler(ctx context.Context, reqs []*transaction.TransactionReq, must bool) (*MultiCreateHandler, error) {
	mh := &MultiHandler{}
	if len(reqs) == 0 && must {
		return nil, fmt.Errorf("invalid reqs")
	}

	for _, req := range reqs {
		handler, err := NewHandler(
			ctx,
			WithPoolID(req.PoolID, true),
			WithTransactionID(req.TransactionID, true),
			WithTransactionType(req.TransactionType, true),
			WithChainID(req.ChainID, true),
			WithOwner(req.Owner, true),
			WithAmountZeroIn(req.AmountZeroIn, false),
			WithAmountOneIn(req.AmountOneIn, false),
			WithAmountZeroOut(req.AmountZeroOut, false),
			WithAmountOneOut(req.AmountOneOut, false),
			WithTimestamp(req.Timestamp, true),
		)
		if err != nil {
			return nil, err
		}
		mh.AppendHandler(handler)
	}
	return &MultiCreateHandler{mh}, nil
}

func (h *MultiCreateHandler) CreateTransactionsWithCli(ctx context.Context, cli *ent.Client) error {
	for _, handler := range h.Handlers {
		if err := handler.CreateTransactionWithCli(ctx, cli); err != nil {
			return err
		}
	}
	return nil
}

func (h *MultiCreateHandler) CreateTransactions(ctx context.Context) error {
	return db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		return h.CreateTransactionsWithCli(_ctx, cli)
	})
}
