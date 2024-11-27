package summary

import (
	"context"
	"fmt"
	"time"

	"github.com/Geapefurit/kline-back/common/kptype"
	"github.com/Geapefurit/kline-back/common/utils"
	"github.com/Geapefurit/kline-back/proto/kline"
	basetype "github.com/Geapefurit/kline-back/proto/kline/basetype/v1"
	kpriceproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	summaryproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/summary"
	tokenpairproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	transactionproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/transaction"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kprice"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/transaction"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

func GetTokenLastCond(ctx context.Context, poolID uint64, t0Addr, t1Addr string) (*summaryproto.TokenLastCond, error) {
	fmt.Println(poolID, t0Addr, t1Addr)
	tokenPair, err := GetTokenPair(ctx, poolID, t0Addr, t1Addr)
	if err != nil {
		return nil, err
	}
	lastTx, err := GetLastTransaction(ctx, poolID)
	if err != nil {
		return nil, err
	}
	oneDayPrices, err := GetOneDayKPrice(ctx, tokenPair.ID)
	if err != nil {
		return nil, err
	}
	fmt.Println(utils.PrettyStruct(oneDayPrices))
	txVolumn, err := GetOneDayVolumn(ctx, poolID)
	if err != nil {
		return nil, err
	}

	return &summaryproto.TokenLastCond{
		PoolID:                 poolID,
		TokenZeroAddress:       t0Addr,
		TokenOneAddress:        t1Addr,
		LastTxAt:               lastTx.Timestamp,
		LastTxZeroAmount:       lastTx.AmountZeroIn,
		LastTxOneAmount:        lastTx.AmountOneIn,
		OneDayZeroAmountVolumn: txVolumn.AmountZeroVolumn,
		OneDayOneAmountVolumn:  txVolumn.AmountOneVolumn,
		NowPrice:               oneDayPrices[1].Price,
		OneDayIncresePercent:   (oneDayPrices[1].Price - oneDayPrices[0].Price) / oneDayPrices[0].Price * 100,
	}, nil
}

func GetTokenPair(ctx context.Context, poolID uint64, t0Addr, t1Addr string) (*tokenpairproto.TokenPair, error) {
	conds := tokenpairproto.Conds{
		PoolID: &kline.Uint64Val{Op: cruder.EQ, Value: poolID},
	}

	handler, err := tokenpair.NewHandler(ctx,
		tokenpair.WithConds(&conds),
		tokenpair.WithOffset(0),
		tokenpair.WithLimit(2),
	)

	if err != nil {
		return nil, err
	}

	infos, _, err := handler.GetTokenPairs(ctx)
	if err != nil {
		return nil, err
	}

	if len(infos) == 0 {
		return nil, fmt.Errorf("cannot get token pair")
	}

	if infos[0].TokenZeroAddress == t0Addr && infos[0].TokenOneAddress == t1Addr {
		return infos[0], nil
	}

	if len(infos) >= 2 && infos[1].TokenZeroAddress == t0Addr && infos[1].TokenOneAddress == t1Addr {
		return infos[1], nil
	}

	return nil, fmt.Errorf("cannot get token pair")
}

func GetLastTransaction(ctx context.Context, poolID uint64) (*transactionproto.Transaction, error) {
	conds := transactionproto.Conds{
		PoolID:    &kline.Uint64Val{Op: cruder.EQ, Value: poolID},
		Timestamp: &kline.Uint32Val{Op: cruder.LTE, Value: uint32(time.Now().Unix())},
	}

	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithConds(&conds),
		transaction.WithOffset(0),
		transaction.WithLimit(1),
	)
	if err != nil {
		return nil, err
	}

	infos, _, err := handler.GetLatestTransactions(ctx)
	if err != nil {
		return nil, err
	}

	if len(infos) == 0 {
		return &transactionproto.Transaction{}, nil
	}

	return infos[0], nil
}

func GetOneDayKPrice(ctx context.Context, tpID uint32) (ret [2]*kpriceproto.KPrice, err error) {
	nowTimestap := uint32(time.Now().Unix())
	yesterdayTimestap := nowTimestap - kptype.KPointTypeInfos[basetype.KPointType_OneDay].GetSeconds()

	info, err := getEarlistKPrice(ctx, tpID, yesterdayTimestap)
	if err != nil {
		return ret, err
	}
	ret[0] = info

	info, err = getLatestKPrice(ctx, tpID, nowTimestap)
	if err != nil {
		return ret, err
	}
	ret[1] = info

	return ret, nil
}

func getLatestKPrice(ctx context.Context, tpID uint32, timestap uint32) (*kpriceproto.KPrice, error) {
	conds := kpriceproto.Conds{
		TokenPairID: &kline.Uint32Val{Op: cruder.EQ, Value: tpID},
		Timestamp:   &kline.Uint32Val{Op: cruder.LTE, Value: timestap},
	}

	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithConds(&conds),
		kprice.WithOffset(0),
		kprice.WithLimit(1),
	)

	if err != nil {
		return nil, err
	}

	infos, _, err := handler.GetLatestKPrices(ctx)
	if err != nil {
		return nil, err
	}

	if len(infos) == 0 {
		return nil, nil
	}

	return infos[0], nil
}

func getEarlistKPrice(ctx context.Context, tpID uint32, timestap uint32) (*kpriceproto.KPrice, error) {
	conds := kpriceproto.Conds{
		TokenPairID: &kline.Uint32Val{Op: cruder.EQ, Value: tpID},
		Timestamp:   &kline.Uint32Val{Op: cruder.GTE, Value: timestap},
	}

	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithConds(&conds),
		kprice.WithOffset(0),
		kprice.WithLimit(1),
	)

	if err != nil {
		return nil, err
	}

	infos, _, err := handler.GetEarlistKPrices(ctx)
	if err != nil {
		return nil, err
	}

	if len(infos) == 0 {
		return nil, fmt.Errorf("cannot get earlist price")
	}

	return infos[0], nil
}

func GetOneDayVolumn(ctx context.Context, poolID uint64) (*transaction.TransactionVolumn, error) {
	nowTimestap := uint32(time.Now().Unix())
	yesterdayTimestap := nowTimestap - kptype.KPointTypeInfos[basetype.KPointType_OneDay].GetSeconds()

	txH, err := transaction.NewHandler(ctx)
	if err != nil {
		return nil, err
	}
	return txH.GetVolumnFromTransactionByPoolID(ctx, yesterdayTimestap, nowTimestap, poolID)
}
