package tokenpair

import (
	"context"
	"fmt"

	tokenpairproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/tokenpair"
	constant "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/const"
	tokenpaircrud "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/crud/v1/tokenpair"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/mw/v1/token"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Handler struct {
	tokenpaircrud.Req
	Reqs   []*tokenpaircrud.Req
	Conds  *tokenpaircrud.Conds
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

func WithPoolID(u *uint64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if u == nil {
			if must {
				return fmt.Errorf("invalid poolid")
			}
			return nil
		}
		h.PoolID = u
		return nil
	}
}

func WithTokenZeroID(id *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if id == nil {
			if must {
				return fmt.Errorf("invalid tokenzeroid1")
			}
			return nil
		}
		tokenH, err := token.NewHandler(ctx, token.WithID(id, true))
		if err != nil {
			return err
		}

		exist, err := tokenH.ExistToken(ctx)
		if err != nil {
			return err
		}

		if !exist {
			return fmt.Errorf("invalid tokenzeroid2")
		}

		h.TokenZeroID = id
		return nil
	}
}

func WithTokenOneID(id *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if id == nil {
			if must {
				return fmt.Errorf("invalid tokenoneid")
			}
			return nil
		}
		tokenH, err := token.NewHandler(ctx, token.WithID(id, true))
		if err != nil {
			return err
		}

		exist, err := tokenH.ExistToken(ctx)
		if err != nil {
			return err
		}

		if !exist {
			return fmt.Errorf("invalid tokenoneid")
		}

		h.TokenOneID = id
		return nil
	}
}

func WithRemark(remark *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if remark == nil {
			if must {
				return fmt.Errorf("invalid remark")
			}
			return nil
		}
		h.Remark = remark
		return nil
	}
}

//nolint:gocognit
func WithConds(conds *tokenpairproto.Conds) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		h.Conds = &tokenpaircrud.Conds{}
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
		if conds.PoolIDs != nil {
			h.Conds.PoolIDs = &cruder.Cond{
				Op:  conds.GetPoolIDs().GetOp(),
				Val: conds.GetPoolIDs().GetValue(),
			}
		}
		if conds.PoolID != nil {
			h.Conds.PoolID = &cruder.Cond{
				Op:  conds.GetPoolID().GetOp(),
				Val: conds.GetPoolID().GetValue(),
			}
		}
		if conds.TokenZeroID != nil {
			h.Conds.TokenZeroID = &cruder.Cond{
				Op:  conds.GetTokenZeroID().GetOp(),
				Val: conds.GetTokenZeroID().GetValue(),
			}
		}
		if conds.TokenOneID != nil {
			h.Conds.TokenOneID = &cruder.Cond{
				Op:  conds.GetTokenOneID().GetOp(),
				Val: conds.GetTokenOneID().GetValue(),
			}
		}
		if conds.Remark != nil {
			h.Conds.Remark = &cruder.Cond{
				Op:  conds.GetRemark().GetOp(),
				Val: conds.GetRemark().GetValue(),
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
