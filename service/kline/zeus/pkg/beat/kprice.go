package beat

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/Geapefurit/kline-back/config"
	"github.com/Geapefurit/kline-back/proto/kline"
	kpriceproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	tokenproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/token"
	tokenpairproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	"github.com/Geapefurit/kline-back/zeus/pkg/applications"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kprice"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/token"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/shopspring/decimal"
)

type SamplingKPriceTask struct {
	// Token0 address -> Token1 address -> TokenPair ID
	tPairMap   map[string]map[string]uint32
	changeLock sync.Mutex
	closeChan  chan struct{}
}

func GetSamplingKPriceTask(ctx context.Context) (*SamplingKPriceTask, error) {
	task := &SamplingKPriceTask{
		tPairMap: map[string]map[string]uint32{},
	}
	if err := task.loadTPairMap(ctx); err != nil {
		return nil, err
	}
	return task, nil
}

func (st *SamplingKPriceTask) loadTPairMap(ctx context.Context) error {
	//TODO: please read records by paging
	tpH, err := tokenpair.NewHandler(ctx,
		tokenpair.WithConds(
			&tokenpairproto.Conds{}),
		tokenpair.WithLimit(0),
		tokenpair.WithOffset(0))
	if err != nil {
		return err
	}

	tpInfos, _, err := tpH.GetTokenPairs(ctx)
	if err != nil {
		return err
	}

	for _, tpInfo := range tpInfos {
		if _, ok := st.tPairMap[tpInfo.TokenZeroAddress]; !ok {
			st.tPairMap[tpInfo.TokenZeroAddress] = make(map[string]uint32)
		}
		st.tPairMap[tpInfo.TokenZeroAddress][tpInfo.TokenOneAddress] = tpInfo.ID
	}

	return nil
}

func checkAndCreateToken(ctx context.Context, address string) (*tokenproto.Token, error) {
	tokenH, err := token.NewHandler(ctx,
		token.WithConds(&tokenproto.Conds{
			Address: &kline.StringVal{
				Op:    cruder.EQ,
				Value: address,
			},
		}),
		token.WithOffset(0),
		token.WithLimit(1),
	)

	if err != nil {
		return nil, err
	}

	tokenInfos, _, err := tokenH.GetTokens(ctx)
	if err != nil {
		return nil, err
	}

	if len(tokenInfos) > 0 {
		return tokenInfos[0], nil
	}

	tokenReq, err := GetTokenInfos(address)

	// tokenReq, err := MockGetTokenInfos(address)
	if tokenReq == nil {
		return nil, err
	}
	tokenH, err = token.NewHandler(ctx,
		token.WithAddress(&address, true),
		token.WithSite(tokenReq.Site, true),
		token.WithIcon(tokenReq.Icon, true),
		token.WithName(tokenReq.Name, true),
		token.WithSymbol(tokenReq.Symbol, true),
	)
	if err != nil {
		return nil, err
	}

	tokenInfo, err := tokenH.CreateToken(ctx)
	if err != nil {
		return nil, err
	}

	return tokenInfo, nil
}

func checkTokenPair(ctx context.Context, poolID uint64, tokenZeroID, tokenOneID uint32) (*tokenpairproto.TokenPair, error) {
	queryH, err := tokenpair.NewHandler(ctx,
		tokenpair.WithConds(&tokenpairproto.Conds{
			TokenZeroID: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: tokenZeroID,
			},
			TokenOneID: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: tokenOneID,
			},
		}),
		tokenpair.WithLimit(1),
		tokenpair.WithOffset(0),
	)
	if err != nil {
		return nil, err
	}

	tpInfos, _, err := queryH.GetTokenPairs(ctx)
	if err != nil {
		return nil, err
	}

	if len(tpInfos) > 0 {
		return tpInfos[0], nil
	}

	createH, err := tokenpair.NewHandler(ctx,
		tokenpair.WithPoolID(&poolID, true),
		tokenpair.WithTokenZeroID(&tokenZeroID, true),
		tokenpair.WithTokenOneID(&tokenOneID, true),
	)
	if err != nil {
		return nil, err
	}
	if err := createH.CreateTokenPair(ctx); err != nil {
		return nil, err
	}

	tpInfos, _, err = queryH.GetTokenPairs(ctx)
	if err != nil {
		return nil, err
	}

	if len(tpInfos) > 0 {
		return tpInfos[0], nil
	}

	return nil, fmt.Errorf("failed to create tokenpair")
}

func (st *SamplingKPriceTask) getTokenPairID(ctx context.Context, poolID uint64, tokenZeroAddress, tokenOneAddress string) (uint32, error) {
	st.changeLock.Lock()
	defer st.changeLock.Unlock()

	if _, ok := st.tPairMap[tokenZeroAddress]; ok {
		if _, ok := st.tPairMap[tokenZeroAddress][tokenOneAddress]; ok {
			return st.tPairMap[tokenZeroAddress][tokenOneAddress], nil
		}
	} else {
		st.tPairMap[tokenZeroAddress] = make(map[string]uint32)
	}

	tokenZero, err := checkAndCreateToken(ctx, tokenZeroAddress)
	if err != nil {
		return 0, err
	}
	if tokenZero == nil {
		return 0, fmt.Errorf("failed to create token: %v", tokenZeroAddress)
	}

	tokenOne, err := checkAndCreateToken(ctx, tokenOneAddress)
	if err != nil {
		return 0, err
	}
	if tokenOne == nil {
		return 0, fmt.Errorf("failed to create token: %v", tokenOneAddress)
	}

	tpInfo, err := checkTokenPair(ctx, poolID, tokenZero.ID, tokenOne.ID)
	if err != nil {
		return 0, err
	}
	if tpInfo == nil {
		return 0, fmt.Errorf("failed to create tokenpair")
	}

	st.tPairMap[tokenZeroAddress][tokenOneAddress] = tpInfo.ID

	return tpInfo.ID, nil
}

func (st *SamplingKPriceTask) StartSampling(ctx context.Context, seconds uint32) {
	st.closeChan = make(chan struct{})
	for {
		select {
		// try to start with whole seconds and offset 10 milliseconds
		case <-time.NewTicker(time.Second*time.Duration(seconds) + time.Millisecond*10 - time.Duration(time.Now().Nanosecond())%time.Second).C:
			go func() {
				err := st.samplingAndStore(ctx)
				if err != nil {
					logger.Sugar().Error(err)
				}
			}()
		case <-ctx.Done():
			return
		case <-st.closeChan:
			return
		}
	}
}

func (st *SamplingKPriceTask) Close(ctx context.Context, interval time.Duration) {
	close(st.closeChan)
}

type KPriceData struct {
	PoolID           uint64
	TokenZeroAddress string
	TokenOneAddress  string
	Price            float64
	Timestamp        uint32
}

func createKPrices(ctx context.Context, kpriceReqs []*kpriceproto.KPriceReq) error {
	for _, req := range kpriceReqs {
		createH, err := kprice.NewHandler(ctx,
			kprice.WithTokenPairID(req.TokenPairID, true),
			kprice.WithPrice(req.Price, true),
			kprice.WithTime(req.Timestamp, true),
		)
		if err != nil {
			return err
		}
		if err := createH.CreateKPrice(ctx); err != nil {
			return err
		}
	}
	return nil
}

func (st *SamplingKPriceTask) samplingAndStore(ctx context.Context) error {
	kpDataList := GetKPriceDatas()
	// kpDataList := MockGetKPriceDatas()
	kpriceReqs := []*kpriceproto.KPriceReq{}
	for _, kpData := range kpDataList {
		tpID, err := st.getTokenPairID(ctx, kpData.PoolID, kpData.TokenZeroAddress, kpData.TokenOneAddress)
		if err != nil {
			return err
		}
		kpriceReqs = append(kpriceReqs, &kpriceproto.KPriceReq{
			TokenPairID: &tpID,
			Price:       &kpData.Price,
			Timestamp:   &kpData.Timestamp,
		})
	}

	if err := createKPrices(ctx, kpriceReqs); err != nil {
		return err
	}

	return nil
}

func RunSamplingKPrice(ctx context.Context) {
	samplingTask, err := GetSamplingKPriceTask(ctx)
	if err != nil {
		panic(err)
	}
	samplingTask.StartSampling(ctx, 1)
}

func GetKPriceDatas() []*KPriceData {
	now := uint32(time.Now().Unix())
	swapApp := applications.NewSwapApp(
		config.GetConfig().SwapApp.ServerAddr,
		config.GetConfig().SwapApp.ChainID,
		config.GetConfig().SwapApp.AppID,
	)
	resp, err := swapApp.GetReverves()
	if err != nil {
		logger.Sugar().Error(err)
		return nil
	}

	kpDataList := []*KPriceData{}

	for _, tpReverves := range resp.Data.TokenPairReserves {
		price1, err := calPrice(tpReverves.Reserve0, tpReverves.Reserve1)
		if err != nil {
			logger.Sugar().Error(err)
			continue
		}
		price0, err := calPrice(tpReverves.Reserve1, tpReverves.Reserve0)
		if err != nil {
			logger.Sugar().Error(err)
			continue
		}

		kpDataList = append(kpDataList,
			&KPriceData{
				PoolID:           tpReverves.PoolID,
				TokenZeroAddress: tpReverves.Token0,
				TokenOneAddress:  tpReverves.Token1,
				Price:            price0,
				Timestamp:        now,
			},
			&KPriceData{
				PoolID:           tpReverves.PoolID,
				TokenZeroAddress: tpReverves.Token1,
				TokenOneAddress:  tpReverves.Token0,
				Price:            price1,
				Timestamp:        now,
			})
	}

	return kpDataList
}

func MockGetKPriceDatas() []*KPriceData {
	now := uint32(time.Now().Unix())

	return []*KPriceData{
		{
			PoolID:           1,
			TokenZeroAddress: "A",
			TokenOneAddress:  "B",
			Price:            float64(now - 1732502402),
			Timestamp:        now,
		},
		{
			PoolID:           2,
			TokenZeroAddress: "C",
			TokenOneAddress:  "D",
			Price:            float64(now - 1732502402),
			Timestamp:        now,
		},
		{
			PoolID:           3,
			TokenZeroAddress: "E",
			TokenOneAddress:  "F",
			Price:            float64(now - 1732502402),
			Timestamp:        now,
		},
		{
			PoolID:           4,
			TokenZeroAddress: "G",
			TokenOneAddress:  "H",
			Price:            float64(now - 1732502402),
			Timestamp:        now,
		},
	}
}

func calPrice(r0, r1 string) (float64, error) {
	d0, err := decimal.NewFromString(r0)
	if err != nil {
		return 0, err
	}
	d1, err := decimal.NewFromString(r1)
	if err != nil {
		return 0, err
	}

	if d0.IsZero() || d1.IsZero() {
		return 0, nil
	}

	p, _ := d0.Div(d1).Float64()
	return p, nil
}

func MockGetTokenInfos(address string) (*tokenproto.TokenReq, error) {
	return &tokenproto.TokenReq{
		Address: &address,
		Site:    &address,
		Icon:    &address,
		Name:    &address,
		Symbol:  &address,
	}, nil
}

func GetTokenInfos(address string) (*tokenproto.TokenReq, error) {
	app := applications.NewErc20App(
		config.GetConfig().SwapApp.ServerAddr,
		config.GetConfig().SwapApp.ChainID,
		address,
	)
	resp, err := app.GetTokenInfo()
	if err != nil {
		return nil, err
	}

	if resp.Data.Name == "" {
		return nil, nil
	}

	site := ""
	icon := ""

	if _site, ok := resp.Data.TokenMetadata["website"].(string); ok {
		site = _site
	}
	if _icon, ok := resp.Data.TokenMetadata["logo"].(string); ok {
		icon = _icon
	}

	return &tokenproto.TokenReq{
		Symbol:  &resp.Data.Symbol,
		Address: &address,
		Site:    &site,
		Icon:    &icon,
		Name:    &resp.Data.Name,
	}, nil
}
