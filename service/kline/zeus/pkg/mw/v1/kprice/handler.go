package kprice

import (
	"context"
	"fmt"

	kpriceproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/kprice"
	constant "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/const"
	kpricecrud "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/crud/v1/kprice"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/mw/v1/tokenpair"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Handler struct {
	kpricecrud.Req
	Reqs   []*kpricecrud.Req
	Conds  *kpricecrud.Conds
	Offset int32
	Limit  int32
}

func NewHandler(ctx context.Context, options ...func(context.Context, *Handler) error) (*Handler, error) {
	handler := &Handler{}
	for _, opt := range options {
		if err := opt(ctx, handler); err != nil {
			return nil, err
		}
	}
	return handler, nil
}

func WithID(u *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if u == nil {
			if must {
				return fmt.Errorf("invalid id")
			}
			return nil
		}
		h.ID = u
		return nil
	}
}

func WithTokenPairID(id *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if id == nil {
			if must {
				return fmt.Errorf("invalid tokenpairid")
			}
			return nil
		}
		tokenH, err := tokenpair.NewHandler(ctx, tokenpair.WithID(id, true))
		if err != nil {
			return err
		}

		exist, err := tokenH.ExistTokenPair(ctx)
		if err != nil {
			return err
		}

		if !exist {
			return fmt.Errorf("invalid tokenpairid")
		}

		h.TokenPairID = id
		return nil
	}
}

func WithPrice(price *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if price == nil {
			if must {
				return fmt.Errorf("invalid price")
			}
			return nil
		}

		h.Price = price
		return nil
	}
}

func WithTime(endtime *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if endtime == nil {
			if must {
				return fmt.Errorf("invalid endtime")
			}
			return nil
		}

		h.Timestamp = endtime
		return nil
	}
}

//nolint:gocognit
func WithConds(conds *kpriceproto.Conds) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		h.Conds = &kpricecrud.Conds{}
		if conds == nil {
			return nil
		}
		if conds.ID != nil {
			h.Conds.ID = &cruder.Cond{
				Op:  conds.GetID().GetOp(),
				Val: conds.GetID().GetValue(),
			}
		}
		if conds.IDs != nil {
			h.Conds.IDs = &cruder.Cond{
				Op:  conds.GetIDs().GetOp(),
				Val: conds.GetIDs().GetValue(),
			}
		}
		if conds.TokenPairID != nil {
			h.Conds.TokenPairID = &cruder.Cond{
				Op:  conds.GetTokenPairID().GetOp(),
				Val: conds.GetTokenPairID().GetValue(),
			}
		}
		if conds.Timestamp != nil {
			h.Conds.Timestamp = &cruder.Cond{
				Op:  conds.GetTimestamp().GetOp(),
				Val: conds.GetTimestamp().GetValue(),
			}
		}
		return nil
	}
}

func WithOffset(offset int32) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		h.Offset = offset
		return nil
	}
}

func WithLimit(limit int32) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if limit == 0 {
			limit = constant.DefaultRowLimit
		}
		h.Limit = limit
		return nil
	}
}
