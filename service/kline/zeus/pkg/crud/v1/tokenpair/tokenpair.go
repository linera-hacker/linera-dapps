package tokenpair

import (
	"fmt"

	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent"
	tokenpairent "github.com/Geapefurit/kline-back/zeus/pkg/db/ent/tokenpair"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

type Req struct {
	ID          *uint32
	PoolID      *uint64
	TokenZeroID *uint32
	TokenOneID  *uint32
	Remark      *string
	DeletedAt   *uint32
}

func CreateSet(c *ent.TokenPairCreate, req *Req) *ent.TokenPairCreate {
	if req.PoolID != nil {
		c.SetPoolID(*req.PoolID)
	}
	if req.TokenZeroID != nil {
		c.SetTokenZeroID(*req.TokenZeroID)
	}
	if req.TokenOneID != nil {
		c.SetTokenOneID(*req.TokenOneID)
	}
	if req.Remark != nil {
		c.SetRemark(*req.Remark)
	}
	return c
}

func UpdateSet(u *ent.TokenPairUpdateOne, req *Req) (*ent.TokenPairUpdateOne, error) {
	if req.Remark != nil {
		u = u.SetRemark(*req.Remark)
	}
	if req.DeletedAt != nil {
		u = u.SetDeletedAt(*req.DeletedAt)
	}
	return u, nil
}

type Conds struct {
	ID          *cruder.Cond
	PoolID      *cruder.Cond
	TokenZeroID *cruder.Cond
	TokenOneID  *cruder.Cond
	Remark      *cruder.Cond
	IDs         *cruder.Cond
	PoolIDs     *cruder.Cond
}

func SetQueryConds(q *ent.TokenPairQuery, conds *Conds) (*ent.TokenPairQuery, error) { //nolint
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
			q.Where(tokenpairent.ID(id))
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
			q.Where(tokenpairent.IDIn(ids...))
		default:
			return nil, fmt.Errorf("invalid ids field")
		}
	}
	if conds.PoolIDs != nil {
		ids, ok := conds.PoolIDs.Val.([]uint64)
		if !ok {
			return nil, fmt.Errorf("invalid poolidin")
		}
		switch conds.PoolIDs.Op {
		case cruder.IN:
			q.Where(tokenpairent.PoolIDIn(ids...))
		default:
			return nil, fmt.Errorf("invalid poolidin field")
		}
	}
	if conds.PoolID != nil {
		id, ok := conds.PoolID.Val.(uint64)
		if !ok {
			return nil, fmt.Errorf("invalid poolid")
		}
		switch conds.PoolID.Op {
		case cruder.EQ:
			q.Where(tokenpairent.PoolID(id))
		default:
			return nil, fmt.Errorf("invalid poolid field")
		}
	}
	if conds.TokenZeroID != nil {
		id, ok := conds.TokenZeroID.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid tokenzeroid")
		}
		switch conds.TokenZeroID.Op {
		case cruder.EQ:
			q.Where(tokenpairent.TokenZeroID(id))
		default:
			return nil, fmt.Errorf("invalid tokenzeroid field")
		}
	}

	if conds.TokenOneID != nil {
		id, ok := conds.TokenOneID.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid tokenoneid")
		}
		switch conds.TokenOneID.Op {
		case cruder.EQ:
			q.Where(tokenpairent.TokenOneID(id))
		default:
			return nil, fmt.Errorf("invalid tokenoneid field")
		}
	}
	if conds.Remark != nil {
		remark, ok := conds.Remark.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid remark")
		}
		switch conds.Remark.Op {
		case cruder.EQ:
			q.Where(tokenpairent.Remark(remark))
		default:
			return nil, fmt.Errorf("invalid remark field")
		}
	}
	return q, nil
}
