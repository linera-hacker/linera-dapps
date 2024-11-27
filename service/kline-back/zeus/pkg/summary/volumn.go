package summary

import (
	"context"
	"fmt"
	"sort"
	"time"

	"github.com/Geapefurit/kline-back/common/kptype"
	"github.com/Geapefurit/kline-back/common/utils"
	"github.com/Geapefurit/kline-back/proto/kline"
	basetype "github.com/Geapefurit/kline-back/proto/kline/basetype/v1"
	summaryproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/summary"
	tokenpairproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/transaction"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

var oneDayTokenVolumn []*summaryproto.TokenVolumn
var oneDayUpdateAt *time.Time

func GetOneDayVolumnRank(ctx context.Context, topN int) ([]*summaryproto.TokenVolumn, error) {
	now := time.Now()
	nowUnix := uint32(now.Unix())
	yesterdayUnix := nowUnix - kptype.KPointTypeInfos[basetype.KPointType_OneDay].Seconds
	if oneDayTokenVolumn != nil && !now.After(oneDayUpdateAt.Add(time.Minute)) {
		return oneDayTokenVolumn, nil
	}

	ret, err := getVolumnRank(ctx, yesterdayUnix, nowUnix, topN)
	if err != nil {
		return nil, err
	}
	oneDayTokenVolumn = ret
	oneDayUpdateAt = &now
	return oneDayTokenVolumn, nil
}

func getVolumnRank(ctx context.Context, start, end uint32, topN int) ([]*summaryproto.TokenVolumn, error) {
	txH, err := transaction.NewHandler(ctx)
	if err != nil {
		return nil, err
	}
	txList, err := txH.GetVolumnFromTransaction(ctx, start, end)
	if err != nil {
		return nil, err
	}

	sort.Slice(txList, func(i, j int) bool {
		return txList[i].AmountZeroVolumn > txList[j].AmountZeroVolumn
	})

	if len(txList) < topN {
		topN = len(txList)
	}

	txList = txList[0:topN]
	poolIDs := []uint64{}
	for _, v := range txList {
		poolIDs = append(poolIDs, v.PoolID)
	}

	conds := &tokenpairproto.Conds{PoolIDs: &kline.Uint64SliceVal{Op: cruder.IN, Value: poolIDs}}
	tpH, err := tokenpair.NewHandler(ctx, tokenpair.WithConds(conds), tokenpair.WithLimit(int32(topN*2)), tokenpair.WithOffset(0))
	if err != nil {
		return nil, err
	}

	_tokenPairs, _, err := tpH.GetTokenPairs(ctx)
	if err != nil {
		return nil, err
	}
	fmt.Println(utils.PrettyStruct(tpH.Conds.PoolIDs))
	fmt.Println(utils.PrettyStruct(poolIDs))
	fmt.Println(utils.PrettyStruct(_tokenPairs))

	tokenPairs := make(map[uint64]*tokenpairproto.TokenPair)
	for _, v := range _tokenPairs {
		if v.TokenZeroSymbol != "WTLINERA" {
			tokenPairs[v.PoolID] = v
		}
	}

	tvList := []*summaryproto.TokenVolumn{}
	for _, v := range txList {
		if tpInfo, ok := tokenPairs[v.PoolID]; ok {
			tvList = append(tvList, &summaryproto.TokenVolumn{
				PoolID:  tpInfo.PoolID,
				Address: tpInfo.TokenZeroAddress,
				Name:    tpInfo.TokenZeroName,
				Icon:    tpInfo.TokenZeroIcon,
				Symbol:  tpInfo.TokenZeroSymbol,
				Amount:  v.AmountZeroVolumn,
			})
		}
	}

	return tvList, nil
}
