package kpoint

import (
	"context"
	"fmt"

	basetype "github.com/Geapefurit/kline-back/proto/kline/basetype/v1"
	kpointproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"
	constant "github.com/Geapefurit/kline-back/zeus/pkg/const"
	kpointcrud "github.com/Geapefurit/kline-back/zeus/pkg/crud/v1/kpoint"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Handler struct {
	kpointcrud.Req
	Reqs         []*kpointcrud.Req
	Conds        *kpointcrud.Conds
	OriginalTime *uint32
	Offset       int32
	Limit        int32
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

func WithKPointType(kpointtype *basetype.KPointType, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if kpointtype == nil {
			if must {
				return fmt.Errorf("invalid kpointtype")
			}
			return nil
		}
		if *kpointtype == basetype.KPointType_KPointTypeUnknown {
			return fmt.Errorf("invalid kpointtype")
		}
		h.KPointType = kpointtype
		return nil
	}
}

func WithOpen(open *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if open == nil {
			if must {
				return fmt.Errorf("invalid open")
			}
			return nil
		}

		h.Open = open
		return nil
	}
}

func WithHigh(high *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if high == nil {
			if must {
				return fmt.Errorf("invalid high")
			}
			return nil
		}

		h.High = high
		return nil
	}
}

func WithLow(low *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if low == nil {
			if must {
				return fmt.Errorf("invalid low")
			}
			return nil
		}

		h.Low = low
		return nil
	}
}

func WithClose(close *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if close == nil {
			if must {
				return fmt.Errorf("invalid close")
			}
			return nil
		}

		h.Close = close
		return nil
	}
}

func WithStartTime(starttime *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if starttime == nil {
			if must {
				return fmt.Errorf("invalid starttime")
			}
			return nil
		}

		h.StartTime = starttime
		return nil
	}
}

func WithEndTime(endtime *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if endtime == nil {
			if must {
				return fmt.Errorf("invalid endtime")
			}
			return nil
		}

		h.EndTime = endtime
		return nil
	}
}

func WithOriginalTime(originaltime *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if originaltime == nil {
			if must {
				return fmt.Errorf("invalid originaltime")
			}
			return nil
		}

		h.OriginalTime = originaltime
		return nil
	}
}

//nolint:gocognit
func WithConds(conds *kpointproto.Conds) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		h.Conds = &kpointcrud.Conds{}
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
		if conds.KPointType != nil {
			h.Conds.KPointType = &cruder.Cond{
				Op:  conds.GetKPointType().GetOp(),
				Val: basetype.KPointType(conds.GetKPointType().GetValue()),
			}
		}
		if conds.StartAt != nil {
			h.Conds.StartAt = &cruder.Cond{
				Op:  conds.GetStartAt().GetOp(),
				Val: conds.GetStartAt().GetValue(),
			}
		}
		if conds.EndAt != nil {
			h.Conds.EndAt = &cruder.Cond{
				Op:  conds.GetEndAt().GetOp(),
				Val: conds.GetEndAt().GetValue(),
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
