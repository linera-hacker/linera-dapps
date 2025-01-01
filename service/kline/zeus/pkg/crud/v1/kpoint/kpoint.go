package kpoint

import (
	"fmt"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	basetype "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/basetype/v1"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	kpointent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/kpoint"
)

type Req struct {
	ID          *uint32
	TokenPairID *uint32
	KPointType  *basetype.KPointType
	Open        *float64
	High        *float64
	Low         *float64
	Close       *float64
	StartTime   *uint32
	EndTime     *uint32
	DeletedAt   *uint32
}

func CreateSet(c *ent.KPointCreate, req *Req) *ent.KPointCreate {
	if req.TokenPairID != nil {
		c.SetTokenPairID(*req.TokenPairID)
	}
	if req.KPointType != nil {
		c.SetKPointType(req.KPointType.String())
	}
	if req.Open != nil {
		c.SetOpen(*req.Open)
	}
	if req.High != nil {
		c.SetHigh(*req.High)
	}
	if req.Low != nil {
		c.SetLow(*req.Low)
	}
	if req.Close != nil {
		c.SetClose(*req.Close)
	}
	if req.StartTime != nil {
		c.SetStartTime(*req.StartTime)
	}
	if req.EndTime != nil {
		c.SetEndTime(*req.EndTime)
	}
	return c
}

func UpdateSet(u *ent.KPointUpdateOne, req *Req) (*ent.KPointUpdateOne, error) {
	if req.KPointType != nil {
		u = u.SetKPointType(req.KPointType.String())
	}
	if req.Open != nil {
		u = u.SetOpen(*req.Open)
	}
	if req.High != nil {
		u = u.SetHigh(*req.High)
	}
	if req.Low != nil {
		u = u.SetLow(*req.Low)
	}
	if req.Close != nil {
		u = u.SetClose(*req.Close)
	}
	if req.StartTime != nil {
		u = u.SetStartTime(*req.StartTime)
	}
	if req.EndTime != nil {
		u = u.SetEndTime(*req.EndTime)
	}
	if req.DeletedAt != nil {
		u = u.SetDeletedAt(*req.DeletedAt)
	}
	return u, nil
}

type Conds struct {
	ID          *cruder.Cond
	TokenPairID *cruder.Cond
	KPointType  *cruder.Cond
	StartAt     *cruder.Cond
	EndAt       *cruder.Cond
	IDs         *cruder.Cond
}

func SetQueryConds(q *ent.KPointQuery, conds *Conds) (*ent.KPointQuery, error) { //nolint
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
			q.Where(kpointent.ID(id))
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
			q.Where(kpointent.TokenPairIDIn(ids...))
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
			q.Where(kpointent.TokenPairID(id))
		default:
			return nil, fmt.Errorf("invalid tokenzeroid field")
		}
	}

	if conds.KPointType != nil {
		kpointtype, ok := conds.KPointType.Val.(basetype.KPointType)
		if !ok {
			return nil, fmt.Errorf("invalid kpointtype")
		}
		switch conds.KPointType.Op {
		case cruder.EQ:
			q.Where(kpointent.KPointType(kpointtype.String()))
		default:
			return nil, fmt.Errorf("invalid kpointtype field")
		}
	}
	if conds.StartAt != nil {
		startAt, ok := conds.StartAt.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid startat")
		}

		startDateTimestamp := startAt / 24 / 60 / 60 * 24 * 60 * 60
		nextDateTimestamp := startDateTimestamp + 24 * 60 * 60

		switch conds.StartAt.Op {
		case cruder.LT:
			q.Where(kpointent.StartDateTimestampLT(startDateTimestamp))
			q.Where(kpointent.StartTimeLT(startAt))
			q.Where(kpointent.StartTimeGTE(startDateTimestamp))
		case cruder.LTE:
			q.Where(kpointent.StartDateTimestampLTE(startDateTimestamp))
			q.Where(kpointent.StartTimeLTE(startAt))
			q.Where(kpointent.StartTimeGTE(startDateTimestamp))
		case cruder.GT:
			q.Where(kpointent.StartDateTimestampGT(nextDateTimestamp))
			q.Where(kpointent.StartTimeGT(startAt))
			q.Where(kpointent.StartTimeLTE(nextDateTimestamp))
		case cruder.GTE:
			q.Where(kpointent.StartDateTimestampGT(nextDateTimestamp))
			q.Where(kpointent.StartTimeGTE(startAt))
			q.Where(kpointent.StartTimeLTE(nextDateTimestamp))
		case cruder.EQ:
			q.Where(kpointent.StartTimeGTE(startAt))
		default:
			return nil, fmt.Errorf("invalid startat field")
		}
	}
	if conds.EndAt != nil {
		endAt, ok := conds.EndAt.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid endat")
		}

		endDateTimestamp := endAt / 24 / 60 / 60 * 24 * 60 * 60
		nextDateTimestamp := endDateTimestamp + 24 * 60 * 60

		switch conds.EndAt.Op {
		case cruder.LT:
			q.Where(kpointent.EndDateTimestampLT(endDateTimestamp))
			q.Where(kpointent.EndTimeLT(endAt))
			q.Where(kpointent.EndTimeGTE(endDateTimestamp))
		case cruder.LTE:
			q.Where(kpointent.EndDateTimestampLTE(endDateTimestamp))
			q.Where(kpointent.EndTimeLTE(endAt))
			q.Where(kpointent.EndTimeGTE(endDateTimestamp))
		case cruder.GT:
			q.Where(kpointent.EndDateTimestampGT(nextDateTimestamp))
			q.Where(kpointent.EndTimeGT(endAt))
			q.Where(kpointent.EndTimeLTE(nextDateTimestamp))
		case cruder.GTE:
			q.Where(kpointent.EndDateTimestampGTE(nextDateTimestamp))
			q.Where(kpointent.EndTimeGTE(endAt))
			q.Where(kpointent.EndTimeLTE(nextDateTimestamp))
		case cruder.EQ:
			q.Where(kpointent.EndTimeGTE(endAt))
		default:
			return nil, fmt.Errorf("invalid endat field")
		}
	}
	return q, nil
}
