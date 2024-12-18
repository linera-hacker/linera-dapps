package kpoint

import (
	"context"
	"fmt"
	"time"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/linera-hacker/linera-dapps/service/kline/common/kptype"
	basetype "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/basetype/v1"
	kpointproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kpoint"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	kpointent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/kpoint"

	kpointcrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/kpoint"
)

type queryHandler struct {
	*Handler
	stm   *ent.KPointSelect
	infos []*kpointproto.KPoint
	total uint32
}

func (h *queryHandler) selectKPoint(stm *ent.KPointQuery) {
	h.stm = stm.Select(
		kpointent.FieldID,
		kpointent.FieldCreatedAt,
		kpointent.FieldUpdatedAt,
		kpointent.FieldTokenPairID,
		kpointent.FieldKPointType,
		kpointent.FieldOpen,
		kpointent.FieldHigh,
		kpointent.FieldLow,
		kpointent.FieldClose,
		kpointent.FieldStartTime,
		kpointent.FieldEndTime,
	)
}

func (h *queryHandler) formalize() {
	for _, info := range h.infos {
		info.KPointType = basetype.KPointType(basetype.KPointType_value[info.KPointTypeStr])
	}
}

func (h *queryHandler) queryKPoint(cli *ent.Client) error {
	if h.ID == nil {
		return fmt.Errorf("invalid id")
	}
	stm := cli.KPoint.Query().Where(kpointent.DeletedAt(0))
	if h.ID != nil {
		stm.Where(kpointent.ID(*h.ID))
	}
	h.selectKPoint(stm)
	return nil
}

func (h *queryHandler) queryKPoints(ctx context.Context, cli *ent.Client) error {
	stm, err := kpointcrud.SetQueryConds(cli.KPoint.Query(), h.Conds)
	if err != nil {
		return err
	}

	stmCount, err := kpointcrud.SetQueryConds(cli.KPoint.Query(), h.Conds)
	if err != nil {
		return err
	}
	// stmCount.Modify(h.queryJoinToken)
	total, err := stmCount.Count(ctx)
	if err != nil {
		return err
	}
	h.total = uint32(total)

	h.selectKPoint(stm)
	return nil
}

func (h *queryHandler) scan(ctx context.Context) error {
	return h.stm.Scan(ctx, &h.infos)
}

func (h *Handler) GetKPoint(ctx context.Context) (*kpointproto.KPoint, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPoint(cli); err != nil {
			return err
		}
		// handler.queryJoin()
		const singleRowLimit = 2
		handler.stm.Offset(0).Limit(singleRowLimit)
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}
	if len(handler.infos) == 0 {
		return nil, nil
	}
	if len(handler.infos) > 1 {
		return nil, fmt.Errorf("too many record")
	}

	handler.formalize()
	return handler.infos[0], nil
}

func (h *Handler) GetKPoints(ctx context.Context) ([]*kpointproto.KPoint, uint32, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPoints(ctx, cli); err != nil {
			return err
		}
		// handler.queryJoin()
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(kpointent.FieldUpdatedAt))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	handler.formalize()
	return handler.infos, handler.total, nil
}

func (h *Handler) GetEarlistKPoints(ctx context.Context) ([]*kpointproto.KPoint, uint32, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPoints(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Asc(kpointent.FieldEndTime))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	return handler.infos, handler.total, nil
}

func (h *Handler) GetLatestKPoints(ctx context.Context) ([]*kpointproto.KPoint, uint32, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPoints(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(kpointent.FieldEndTime))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	return handler.infos, handler.total, nil
}

func convKPoint(item *kpointproto.KPoint) *kpointproto.KPointForLine {
	return &kpointproto.KPointForLine{
		Nums:        []float64{item.Open, item.Close, item.Low, item.High},
		Times:       []uint32{item.StartTime, item.EndTime},
		FormatTimes: []string{kptype.FormatU32Time(item.StartTime), kptype.FormatU32Time(item.EndTime)},
	}
}

func convKPoints(items []*kpointproto.KPoint) []*kpointproto.KPointForLine {
	ret := make([]*kpointproto.KPointForLine, len(items))
	for i, v := range items {
		ret[i] = convKPoint(v)
	}
	return ret
}

func (h *Handler) GetKPointsForLine(ctx context.Context) ([]*kpointproto.KPointForLine, uint32, error) {
	if h.Offset*h.Limit < 0 || h.Limit == 0 {
		return nil, 0, fmt.Errorf("invalid offset and limit")
	}

	if h.OriginalTime == nil || *h.OriginalTime == 0 {
		now := uint32(time.Now().Unix())
		h.OriginalTime = &now
	}

	h.Conds.EndAt = &cruder.Cond{
		Op:  cruder.GTE,
		Val: *h.OriginalTime,
	}

	forward := true
	if h.Limit < 0 {
		h.Limit = -h.Limit
		h.Offset = -h.Offset
		forward = false
		h.Conds.EndAt.Op = cruder.LT
	}

	var kpoints []*kpointproto.KPoint
	var total uint32
	var err error
	if forward {
		kpoints, total, err = h.GetEarlistKPoints(ctx)
	} else {
		kpoints, total, err = h.GetLatestKPoints(ctx)
		// reverse(kpoints)
	}

	if err != nil {
		return nil, 0, err
	}

	return convKPoints(kpoints), total, nil
}
