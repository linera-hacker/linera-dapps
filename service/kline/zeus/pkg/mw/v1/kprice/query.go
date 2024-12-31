package kprice

import (
	"context"
	"fmt"

	"entgo.io/ent/dialect/sql"
	basetype "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/basetype/v1"
	kpointproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kpoint"
	kpriceproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kprice"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	kpriceent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/kprice"

	kpricecrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/kprice"
)

type queryHandler struct {
	*Handler
	stm   *ent.KPriceSelect
	infos []*kpriceproto.KPrice
	total uint32
}

func (h *queryHandler) selectKPrice(stm *ent.KPriceQuery) {
	h.stm = stm.Select(
		kpriceent.FieldID,
		kpriceent.FieldCreatedAt,
		kpriceent.FieldUpdatedAt,
		kpriceent.FieldTokenPairID,
		kpriceent.FieldPrice,
		kpriceent.FieldTimestamp,
	)
}

func (h *queryHandler) queryKPrice(cli *ent.Client) error {
	if h.ID == nil {
		return fmt.Errorf("invalid id")
	}
	stm := cli.KPrice.Query().Where(kpriceent.DeletedAt(0))
	if h.ID != nil {
		stm.Where(kpriceent.ID(*h.ID))
	}
	h.selectKPrice(stm)
	return nil
}

func (h *queryHandler) queryKPrices(ctx context.Context, cli *ent.Client) error {
	stm, err := kpricecrud.SetQueryConds(cli.KPrice.Query(), h.Conds)
	if err != nil {
		return err
	}

	stmCount, err := kpricecrud.SetQueryConds(cli.KPrice.Query(), h.Conds)
	if err != nil {
		return err
	}
	total, err := stmCount.Count(ctx)
	if err != nil {
		return err
	}
	h.total = uint32(total)

	h.selectKPrice(stm)
	return nil
}

func (h *queryHandler) scan(ctx context.Context) error {
	return h.stm.Scan(ctx, &h.infos)
}

func (h *Handler) GetKPrice(ctx context.Context) (*kpriceproto.KPrice, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPrice(cli); err != nil {
			return err
		}
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

	return handler.infos[0], nil
}

func (h *Handler) GetKPrices(ctx context.Context) ([]*kpriceproto.KPrice, uint32, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPrices(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(kpriceent.FieldUpdatedAt))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	return handler.infos, handler.total, nil
}

func (h *Handler) GetEarlistKPrices(ctx context.Context) ([]*kpriceproto.KPrice, uint32, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryKPrices(ctx, cli.Debug()); err != nil {
			return err
		}
		handler.stm.
			Limit(int(h.Limit)).
			Order(ent.Asc(kpriceent.FieldTimestamp)).
			Offset(int(h.Offset))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	return handler.infos, handler.total, nil
}

func (h *Handler) GetLatestKPrices(ctx context.Context) ([]*kpriceproto.KPrice, uint32, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		cli.Debug()
		if err := handler.queryKPrices(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(kpriceent.FieldTimestamp))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	return handler.infos, handler.total, nil
}

type kpMinMax struct {
	TokenPairID uint32  `sql:"token_pair_id"`
	Low         float64 `sql:"low"`
	High        float64 `sql:"high"`
}

type kpOpen struct {
	TokenPairID uint32  `sql:"token_pair_id"`
	Open        float64 `sql:"open"`
}

type kpClose struct {
	TokenPairID uint32  `sql:"token_pair_id"`
	Close       float64 `sql:"close"`
}

func (h *Handler) GetKPointFromKPrice(ctx context.Context, startTime, endTime uint32, kpType basetype.KPointType) ([]*kpointproto.KPointReq, error) {
	ret := []*kpointproto.KPointReq{}
	var err error
	err = db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		ret, err = getKPointFromKPrice(ctx, cli, startTime, endTime, kpType)
		return err
	})
	if err != nil {
		return nil, err
	}
	return ret, nil
}

func getKPointFromKPrice(ctx context.Context, cli *ent.Client, startTime, endTime uint32, kpType basetype.KPointType) ([]*kpointproto.KPointReq, error) {
	selectMinMaxSql := fmt.Sprintf(
		"SELECT token_pair_id,MIN(price) as low,MAX(price) as high FROM  kprices WHERE `timestamp`>=%v AND `timestamp`<=%v  GROUP BY token_pair_id;",
		startTime,
		endTime,
	)
	selectOpenSql := fmt.Sprintf(
		"SELECT t1.token_pair_id,t1.price as open FROM kprices t1 INNER JOIN (SELECT MIN(`timestamp`) as `timestamp` ,token_pair_id FROM kprices WHERE `timestamp`>=%v AND `timestamp`<=%v GROUP BY token_pair_id ) t2 ON t2.token_pair_id = t1.token_pair_id AND t2.`timestamp` = t1.`timestamp`;",
		startTime,
		endTime,
	)
	selectCloseSql := fmt.Sprintf(
		"SELECT t1.token_pair_id,t1.price as close FROM kprices t1 INNER JOIN (SELECT MAX(`timestamp`) as `timestamp` ,token_pair_id FROM kprices WHERE `timestamp`>=%v AND `timestamp`<=%v GROUP BY token_pair_id ) t2 ON t2.token_pair_id = t1.token_pair_id AND t2.`timestamp` = t1.`timestamp`;",
		startTime,
		endTime,
	)

	queryFunc := func(ctx context.Context, cli *ent.Client, sqlStr string, v interface{}) error {
		rows, err := cli.KPoint.QueryContext(ctx, sqlStr)
		if err != nil {
			return err
		}
		defer rows.Close()

		err = sql.ScanSlice(rows, v)
		if err != nil {
			return err
		}
		return nil
	}

	var _kpMaxMin []*kpMinMax
	if err := queryFunc(ctx, cli, selectMinMaxSql, &_kpMaxMin); err != nil {
		return nil, err
	}

	var _kpOpen []*kpOpen
	if err := queryFunc(ctx, cli, selectOpenSql, &_kpOpen); err != nil {
		return nil, err
	}

	var _kpClose []*kpClose
	if err := queryFunc(ctx, cli, selectCloseSql, &_kpClose); err != nil {
		return nil, err
	}

	kpReqList := []*kpointproto.KPointReq{}
	for i := range _kpMaxMin {
		kpReqList = append(kpReqList, &kpointproto.KPointReq{
			KPointType:  &kpType,
			TokenPairID: &_kpMaxMin[i].TokenPairID,
			Open:        &_kpOpen[i].Open,
			Close:       &_kpClose[i].Close,
			High:        &_kpMaxMin[i].High,
			Low:         &_kpMaxMin[i].Low,
			StartTime:   &startTime,
			EndTime:     &endTime,
		})
	}

	return kpReqList, nil
}
