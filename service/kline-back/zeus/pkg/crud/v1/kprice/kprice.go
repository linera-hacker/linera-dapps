package kprice

import (
	"fmt"

	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	kpriceent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/kprice"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Req struct {
	ID          *uint32
	TokenPairID *uint32
	Price       *float64
	Timestamp   *uint32
	DeletedAt   *uint32
}

func CreateSet(c *ent.KPriceCreate, req *Req) *ent.KPriceCreate {
	if req.TokenPairID != nil {
		c.SetTokenPairID(*req.TokenPairID)
	}
	if req.Price != nil {
		c.SetPrice(*req.Price)
	}
	if req.Timestamp != nil {
		c.SetTimestamp(*req.Timestamp)
	}
	return c
}

func UpdateSet(u *ent.KPriceUpdateOne, req *Req) (*ent.KPriceUpdateOne, error) {
	if req.Price != nil {
		u = u.SetPrice(*req.Price)
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
	ID          *cruder.Cond
	TokenPairID *cruder.Cond
	Price       *cruder.Cond
	Timestamp   *cruder.Cond
	IDs         *cruder.Cond
}

func SetQueryConds(q *ent.KPriceQuery, conds *Conds) (*ent.KPriceQuery, error) { //nolint
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
			q.Where(kpriceent.ID(id))
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
			q.Where(kpriceent.TokenPairIDIn(ids...))
		default:
			return nil, fmt.Errorf("invalid ids field")
		}
	}
	if conds.TokenPairID != nil {
		id, ok := conds.TokenPairID.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid tokenzeroid")
		}
		switch conds.TokenPairID.Op {
		case cruder.EQ:
			q.Where(kpriceent.TokenPairID(id))
		default:
			return nil, fmt.Errorf("invalid tokenzeroid field")
		}
	}
	if conds.Timestamp != nil {
		endat, ok := conds.Timestamp.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid endat")
		}
		switch conds.Timestamp.Op {
		case cruder.LT:
			q.Where(kpriceent.TimestampLT(endat))
		case cruder.LTE:
			q.Where(kpriceent.TimestampLTE(endat))
		case cruder.GT:
			q.Where(kpriceent.TimestampGT(endat))
		case cruder.GTE:
			q.Where(kpriceent.TimestampGTE(endat))
		case cruder.EQ:
			q.Where(kpriceent.TimestampEQ(endat))
		default:
			return nil, fmt.Errorf("invalid endat field")
		}
	}
	return q, nil
}
