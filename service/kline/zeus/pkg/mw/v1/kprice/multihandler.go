package kprice

import (
	"context"
	"fmt"

	"github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kprice"
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

func NewMultiCreateHandler(ctx context.Context, reqs []*kprice.KPriceReq, must bool) (*MultiCreateHandler, error) {
	mh := &MultiHandler{}
	if len(reqs) == 0 && must {
		return nil, fmt.Errorf("invalid reqs")
	}

	for _, req := range reqs {
		handler, err := NewHandler(
			ctx,
			WithTokenPairID(req.TokenPairID, true),
			WithPrice(req.Price, true),
			WithTime(req.Timestamp, true),
		)
		if err != nil {
			return nil, err
		}
		mh.AppendHandler(handler)
	}
	return &MultiCreateHandler{mh}, nil
}

func (h *MultiCreateHandler) CreateKPricesWithCli(ctx context.Context, cli *ent.Client) error {
	for _, handler := range h.Handlers {
		if err := handler.CreateKPriceWithCli(ctx, cli); err != nil {
			return err
		}
	}
	return nil
}

func (h *MultiCreateHandler) CreateKPrices(ctx context.Context) error {
	return db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		return h.CreateKPricesWithCli(_ctx, cli)
	})
}
