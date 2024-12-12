package kpoint

import (
	"context"
	"fmt"
	"os"
	"strconv"
	"testing"

	"github.com/Geapefurit/kline-back/zeus/pkg/db"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"

	"github.com/stretchr/testify/assert"

	"github.com/Geapefurit/kline-back/proto/kline"
	v1 "github.com/Geapefurit/kline-back/proto/kline/basetype/v1"
	kpointproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"
)

func init() {
	//nolint
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}
}

var tokenKPRet = &kpointproto.KPoint{
	KPointType: v1.KPointType_FiveSecond,
	Open:       10086,
	High:       10086,
	Low:        10086,
	Close:      10081,
	StartTime:  10086,
	EndTime:    10086,
}

var tokenKPReq = &kpointproto.KPointReq{
	KPointType: &tokenKPRet.KPointType,
	Open:       &tokenKPRet.Open,
	High:       &tokenKPRet.High,
	Low:        &tokenKPRet.Low,
	Close:      &tokenKPRet.Close,
	StartTime:  &tokenKPRet.StartTime,
	EndTime:    &tokenKPRet.EndTime,
}

func createKP(t *testing.T) {
	tokenKPRet.TokenPairID = tokenPTRet.ID
	tokenKPReq.TokenPairID = &tokenPTRet.ID

	handler, err := NewHandler(
		context.Background(),
		WithTokenPairID(tokenKPReq.TokenPairID, true),
		WithKPointType(tokenKPReq.KPointType, true),
		WithOpen(tokenKPReq.Open, true),
		WithHigh(tokenKPReq.High, true),
		WithLow(tokenKPReq.Low, true),
		WithClose(tokenKPReq.Close, true),
		WithStartTime(tokenKPReq.StartTime, true),
		WithEndTime(tokenKPReq.EndTime, true),
	)
	assert.Nil(t, err)

	err = handler.CreateKPoint(context.Background())
	assert.Nil(t, err)
}

func queryKP(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithConds(&kpointproto.Conds{
			TokenPairID: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: *tokenKPReq.TokenPairID,
			},
			KPointType: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: uint32(*tokenKPReq.KPointType),
			},
			EndAt: &kline.Uint32Val{
				Op:    cruder.GTE,
				Value: *tokenKPReq.EndTime,
			},
		}),
		WithOffset(0),
		WithLimit(1),
	)
	assert.Nil(t, err)
	infos, total, err := handler.GetKPoints(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, total, uint32(1))

	tokenKPReq.ID = &infos[0].ID
	tokenKPRet.ID = infos[0].ID

	handler, err = NewHandler(
		context.Background(),
		WithID(tokenKPReq.ID, true),
	)
	assert.Nil(t, err)

	info, err := handler.GetKPoint(context.Background())
	assert.Nil(t, err)

	assert.Equal(t, infos[0], info)
}

func updateKP(t *testing.T) {
	open := 1008611.0
	tokenKPReq.Open = &open
	tokenKPRet.Open = open

	handler, err := NewHandler(
		context.Background(),
		WithID(tokenKPReq.ID, true),
		WithOpen(tokenKPReq.Open, false),
		WithClose(tokenKPReq.Close, false),
	)
	assert.Nil(t, err)
	err = handler.UpdateKPoint(context.Background())
	assert.Nil(t, err)

	info, err := handler.GetKPoint(context.Background())
	assert.Nil(t, err)
	tokenKPRet.KPointTypeStr = info.KPointTypeStr
	tokenKPRet.UpdatedAt = info.UpdatedAt
	tokenKPRet.CreatedAt = info.CreatedAt
	assert.Equal(t, tokenKPRet, info)
}

func deleteKP(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithID(tokenKPReq.ID, true),
	)
	assert.Nil(t, err)

	err = handler.DeleteKPoint(context.Background())
	assert.Nil(t, err)
}

func TestKP(t *testing.T) {
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}

	err := db.Init()
	if err != nil {
		fmt.Printf("cannot init database: %v \n", err)
		os.Exit(0)
	}
	t.Run("createToken1", createToken1)
	t.Run("createToken2", createToken2)
	t.Run("createTP", createTP)
	t.Run("createKP", createKP)
	t.Run("queryKP", queryKP)
	t.Run("updateKP", updateKP)
	t.Run("deleteKP", deleteKP)
	t.Run("deleteTP", deleteTP)
	t.Run("deleteToken1", deleteToken1)
	t.Run("deleteToken2", deleteToken2)
}
