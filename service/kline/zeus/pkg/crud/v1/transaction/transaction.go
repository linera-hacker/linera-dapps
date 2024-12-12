package transaction

import (
	"fmt"

	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	transactionent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/transaction"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Req struct {
	ID              *uint32
	PoolID          *uint64
	TransactionID   *uint64
	TransactionType *string
	ChainID         *string
	Owner           *string
	AmountZeroIn    *float64
	AmountOneIn     *float64
	AmountZeroOut   *float64
	AmountOneOut    *float64
	Timestamp       *uint32
	DeletedAt       *uint32
}

func CreateSet(c *ent.TransactionCreate, req *Req) *ent.TransactionCreate {
	if req.PoolID != nil {
		c.SetPoolID(*req.PoolID)
	}
	if req.TransactionID != nil {
		c.SetTransactionID(*req.TransactionID)
	}
	if req.TransactionType != nil {
		c.SetTransactionType(*req.TransactionType)
	}
	if req.ChainID != nil {
		c.SetChainID(*req.ChainID)
	}
	if req.Owner != nil {
		c.SetOwner(*req.Owner)
	}
	if req.AmountZeroIn != nil {
		c.SetAmountZeroIn(*req.AmountZeroIn)
	}
	if req.AmountOneIn != nil {
		c.SetAmountOneIn(*req.AmountOneIn)
	}
	if req.AmountZeroOut != nil {
		c.SetAmountZeroOut(*req.AmountZeroOut)
	}
	if req.AmountOneOut != nil {
		c.SetAmountOneOut(*req.AmountOneOut)
	}
	if req.Timestamp != nil {
		c.SetTimestamp(*req.Timestamp)
	}
	return c
}

func UpdateSet(u *ent.TransactionUpdateOne, req *Req) (*ent.TransactionUpdateOne, error) {
	if req.PoolID != nil {
		u = u.SetPoolID(*req.PoolID)
	}
	if req.TransactionID != nil {
		u = u.SetTransactionID(*req.TransactionID)
	}
	if req.TransactionType != nil {
		u = u.SetTransactionType(*req.TransactionType)
	}
	if req.ChainID != nil {
		u = u.SetChainID(*req.ChainID)
	}
	if req.Owner != nil {
		u = u.SetOwner(*req.Owner)
	}
	if req.AmountZeroIn != nil {
		u = u.SetAmountZeroIn(*req.AmountZeroIn)
	}
	if req.AmountOneIn != nil {
		u = u.SetAmountOneIn(*req.AmountOneIn)
	}
	if req.AmountZeroOut != nil {
		u = u.SetAmountZeroOut(*req.AmountZeroOut)
	}
	if req.AmountOneOut != nil {
		u = u.SetAmountOneOut(*req.AmountOneOut)
	}
	if req.Timestamp != nil {
		u = u.SetTimestamp(*req.Timestamp)
	}
	if req.DeletedAt != nil {
		u = u.SetDeletedAt(*req.DeletedAt)
	}
	return u, nil
}

type Conds struct {
	ID              *cruder.Cond
	PoolID          *cruder.Cond
	TransactionID   *cruder.Cond
	TransactionType *cruder.Cond
	Timestamp       *cruder.Cond
	IDs             *cruder.Cond
}

func SetQueryConds(q *ent.TransactionQuery, conds *Conds) (*ent.TransactionQuery, error) { //nolint
	if conds == nil {
		return nil, fmt.Errorf("have no any conds")
	}
	if conds.ID != nil {
		id, ok := conds.ID.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid id")
		}
		switch conds.ID.Op {
		case cruder.EQ:
			q.Where(transactionent.ID(id))
		default:
			return nil, fmt.Errorf("invalid id field")
		}
	}
	if conds.IDs != nil {
		ids, ok := conds.IDs.Val.([]uint32)
		if !ok {
			return nil, fmt.Errorf("invalid ids")
		}
		switch conds.IDs.Op {
		case cruder.IN:
			q.Where(transactionent.IDIn(ids...))
		default:
			return nil, fmt.Errorf("invalid ids field")
		}
	}
	if conds.PoolID != nil {
		id, ok := conds.PoolID.Val.(uint64)
		if !ok {
			return nil, fmt.Errorf("invalid poolid")
		}
		switch conds.PoolID.Op {
		case cruder.EQ:
			q.Where(transactionent.PoolID(id))
		default:
			return nil, fmt.Errorf("invalid poolid field")
		}
	}
	if conds.TransactionID != nil {
		id, ok := conds.TransactionID.Val.(uint64)
		if !ok {
			return nil, fmt.Errorf("invalid transactionid")
		}
		switch conds.TransactionID.Op {
		case cruder.LT:
			q.Where(transactionent.TransactionIDLT(id))
		case cruder.LTE:
			q.Where(transactionent.TransactionIDLTE(id))
		case cruder.GT:
			q.Where(transactionent.TransactionIDGT(id))
		case cruder.GTE:
			q.Where(transactionent.TransactionIDGTE(id))
		case cruder.EQ:
			q.Where(transactionent.TransactionIDEQ(id))
		default:
			return nil, fmt.Errorf("invalid transactionid field")
		}
	}
	if conds.TransactionType != nil {
		txType, ok := conds.TransactionType.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid transactiontype")
		}
		switch conds.TransactionType.Op {
		case cruder.EQ:
			q.Where(transactionent.TransactionType(txType))
		default:
			return nil, fmt.Errorf("invalid transactiontype field")
		}
	}
	if conds.Timestamp != nil {
		timestamp, ok := conds.Timestamp.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid timestamp")
		}
		switch conds.Timestamp.Op {
		case cruder.LT:
			q.Where(transactionent.TimestampLT(timestamp))
		case cruder.LTE:
			q.Where(transactionent.TimestampLTE(timestamp))
		case cruder.GT:
			q.Where(transactionent.TimestampGT(timestamp))
		case cruder.GTE:
			q.Where(transactionent.TimestampGTE(timestamp))
		case cruder.EQ:
			q.Where(transactionent.TimestampEQ(timestamp))
		default:
			return nil, fmt.Errorf("invalid timestamp field")
		}
	}
	return q, nil
}
