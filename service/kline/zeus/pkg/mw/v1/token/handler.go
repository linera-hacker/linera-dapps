package token

import (
	"context"
	"fmt"

	tokenproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/token"
	constant "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/const"
	tokencrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/token"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Handler struct {
	ID      *uint32
	Address *string
	Site    *string
	Icon    *string
	Name    *string
	Symbol  *string
	Reqs    []*tokencrud.Req
	Conds   *tokencrud.Conds
	Offset  int32
	Limit   int32
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

func WithAddress(address *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if address == nil {
			if must {
				return fmt.Errorf("invalid address")
			}
			return nil
		}

		h.Address = address
		return nil
	}
}
func WithSite(u *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if u == nil {
			if must {
				return fmt.Errorf("invalid site")
			}
			return nil
		}

		h.Site = u
		return nil
	}
}
func WithIcon(u *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if u == nil {
			if must {
				return fmt.Errorf("invalid icon")
			}
			return nil
		}
		h.Icon = u
		return nil
	}
}
func WithName(u *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if u == nil {
			if must {
				return fmt.Errorf("invalid name")
			}
			return nil
		}
		h.Name = u
		return nil
	}
}

func WithSymbol(u *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if u == nil {
			if must {
				return fmt.Errorf("invalid symbol")
			}
			return nil
		}
		h.Symbol = u
		return nil
	}
}

func WithReqs(reqs []*tokenproto.TokenReq, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		_reqs := []*tokencrud.Req{}
		for _, req := range reqs {
			_req := &tokencrud.Req{}
			if req.Address != nil {
				_req.Address = req.Address
			}
			if req.Site != nil {
				_req.Site = req.Site
			}
			if req.Icon != nil {
				_req.Icon = req.Icon
			}
			if req.Name != nil {
				_req.Name = req.Name
			}
			if req.Symbol != nil {
				_req.Symbol = req.Symbol
			}
			_reqs = append(_reqs, _req)
		}
		h.Reqs = _reqs
		return nil
	}
}

func WithConds(conds *tokenproto.Conds) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		h.Conds = &tokencrud.Conds{}
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
		if conds.Address != nil {
			h.Conds.Address = &cruder.Cond{
				Op:  conds.GetAddress().GetOp(),
				Val: conds.GetAddress().GetValue(),
			}
		}

		if conds.Site != nil {
			h.Conds.Site = &cruder.Cond{
				Op:  conds.GetSite().GetOp(),
				Val: conds.GetSite().GetValue(),
			}
		}
		if conds.Icon != nil {
			h.Conds.Icon = &cruder.Cond{
				Op:  conds.GetIcon().GetOp(),
				Val: conds.GetIcon().GetValue(),
			}
		}
		if conds.Name != nil {
			h.Conds.Name = &cruder.Cond{
				Op:  conds.GetName().GetOp(),
				Val: conds.GetName().GetValue(),
			}
		}
		if conds.Symbol != nil {
			h.Conds.Symbol = &cruder.Cond{
				Op:  conds.GetSymbol().GetOp(),
				Val: conds.GetSymbol().GetValue(),
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
