package transaction

import (
	"context"
	"fmt"

	transactionproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	constant "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/const"
	transactioncrud "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/crud/v1/transaction"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Handler struct {
	transactioncrud.Req
	OriginalTxID *uint64
	Reqs         []*transactioncrud.Req
	Conds        *transactioncrud.Conds
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

func WithPoolID(id *uint64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if id == nil {
			if must {
				return fmt.Errorf("invalid poolid")
			}
			return nil
		}

		h.PoolID = id
		return nil
	}
}

func WithTransactionID(id *uint64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if id == nil {
			if must {
				return fmt.Errorf("invalid transactionid")
			}
			return nil
		}

		h.TransactionID = id
		return nil
	}
}

func WithOriginalTxID(id *uint64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if id == nil {
			if must {
				return fmt.Errorf("invalid originaltxid")
			}
			return nil
		}

		h.OriginalTxID = id
		return nil
	}
}

func WithTransactionType(txType *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if txType == nil {
			if must {
				return fmt.Errorf("invalid transactiontype")
			}
			return nil
		}

		h.TransactionType = txType
		return nil
	}
}

func WithChainID(chainId *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if chainId == nil {
			if must {
				return fmt.Errorf("invalid chainid")
			}
			return nil
		}

		h.ChainID = chainId
		return nil
	}
}

func WithOwner(owner *string, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if owner == nil {
			if must {
				return fmt.Errorf("invalid owner")
			}
			return nil
		}

		h.Owner = owner
		return nil
	}
}

func WithAmountZeroIn(amount *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if amount == nil {
			if must {
				return fmt.Errorf("invalid amountzeroin")
			}
			return nil
		}

		h.AmountZeroIn = amount
		return nil
	}
}

func WithAmountOneIn(amount *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if amount == nil {
			if must {
				return fmt.Errorf("invalid amountonein")
			}
			return nil
		}

		h.AmountOneIn = amount
		return nil
	}
}

func WithAmountZeroOut(amount *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if amount == nil {
			if must {
				return fmt.Errorf("invalid amountzeroout")
			}
			return nil
		}

		h.AmountZeroOut = amount
		return nil
	}
}

func WithAmountOneOut(amount *float64, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if amount == nil {
			if must {
				return fmt.Errorf("invalid amountoneout")
			}
			return nil
		}

		h.AmountOneOut = amount
		return nil
	}
}

func WithTimestamp(timestamp *uint32, must bool) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		if timestamp == nil {
			if must {
				return fmt.Errorf("invalid timestamp")
			}
			return nil
		}

		h.Timestamp = timestamp
		return nil
	}
}

//nolint:gocognit
func WithConds(conds *transactionproto.Conds) func(context.Context, *Handler) error {
	return func(ctx context.Context, h *Handler) error {
		h.Conds = &transactioncrud.Conds{}
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
		if conds.PoolID != nil {
			h.Conds.PoolID = &cruder.Cond{
				Op:  conds.GetPoolID().GetOp(),
				Val: conds.GetPoolID().GetValue(),
			}
		}
		if conds.TransactionID != nil {
			h.Conds.TransactionID = &cruder.Cond{
				Op:  conds.GetTransactionID().GetOp(),
				Val: conds.GetTransactionID().GetValue(),
			}
		}
		if conds.TransactionType != nil {
			h.Conds.TransactionType = &cruder.Cond{
				Op:  conds.GetTransactionType().GetOp(),
				Val: conds.GetTransactionType().GetValue(),
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
