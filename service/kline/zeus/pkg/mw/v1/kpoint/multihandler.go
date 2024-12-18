package kpoint

import (
	"context"
	"fmt"

	"github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kpoint"
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

func NewMultiCreateHandler(ctx context.Context, reqs []*kpoint.KPointReq, must bool) (*MultiCreateHandler, error) {
	mh := &MultiHandler{}
	if len(reqs) == 0 && must {
		return nil, fmt.Errorf("invalid reqs")
	}

	for _, req := range reqs {
		handler, err := NewHandler(
			ctx,
			WithTokenPairID(req.TokenPairID, false),
			WithKPointType(req.KPointType, true),
			WithOpen(req.Open, true),
			WithHigh(req.High, false),
			WithLow(req.Low, false),
			WithClose(req.Close, true),
			WithStartTime(req.StartTime, false),
			WithEndTime(req.EndTime, false),
		)
		if err != nil {
			return nil, err
		}
		mh.AppendHandler(handler)
	}
	return &MultiCreateHandler{mh}, nil
}

func (h *MultiCreateHandler) CreateKPointsWithCli(ctx context.Context, cli *ent.Client) error {
	for _, handler := range h.Handlers {
		if err := handler.CreateKPointWithCli(ctx, cli); err != nil {
			return err
		}
	}
	return nil
}

func (h *MultiCreateHandler) CreateKPoints(ctx context.Context) error {
	return db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		return h.CreateKPointsWithCli(_ctx, cli)
	})
}
