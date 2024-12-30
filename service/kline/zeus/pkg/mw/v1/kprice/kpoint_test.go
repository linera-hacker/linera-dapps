package kprice

import (
	"context"
	"fmt"
	"os"
	"strconv"
	"testing"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"

	"github.com/stretchr/testify/assert"

	"github.com/linera-hacker/linera-dapps/service/kline/proto/kline"
	kpriceproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kprice"
)

func init() {
	//nolint
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}
}

var tokenKPriceRet = &kpriceproto.KPrice{
	Price:     10086,
	Timestamp: 10086,
}

var tokenKPriceReq = &kpriceproto.KPriceReq{
	Price:     &tokenKPriceRet.Price,
	Timestamp: &tokenKPriceRet.Timestamp,
}

func createKP(t *testing.T) {
	tokenKPriceRet.TokenPairID = tokenPTRet.ID
	tokenKPriceReq.TokenPairID = &tokenPTRet.ID

	handler, err := NewHandler(
		context.Background(),
		WithTokenPairID(tokenKPriceReq.TokenPairID, true),
		WithPrice(tokenKPriceReq.Price, true),
		WithTime(tokenKPriceReq.Timestamp, true),
	)
	assert.Nil(t, err)

	err = handler.CreateKPrice(context.Background())
	assert.Nil(t, err)
}

func queryKP(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithConds(&kpriceproto.Conds{
			TokenPairID: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: *tokenKPriceReq.TokenPairID,
			},
			Timestamp: &kline.Uint32Val{
				Op:    cruder.GTE,
				Value: *tokenKPriceReq.Timestamp,
			},
		}),
		WithOffset(0),
		WithLimit(1),
	)
	assert.Nil(t, err)
	infos, total, err := handler.GetKPrices(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, total, uint32(1))

	tokenKPriceReq.ID = &infos[0].ID
	tokenKPriceRet.ID = infos[0].ID

	handler, err = NewHandler(
		context.Background(),
		WithID(tokenKPriceReq.ID, true),
	)
	assert.Nil(t, err)

	info, err := handler.GetKPrice(context.Background())
	assert.Nil(t, err)

	assert.Equal(t, infos[0], info)
}

func updateKP(t *testing.T) {
	price := 1008611.01
	tokenKPriceReq.Price = &price
	tokenKPriceRet.Price = price

	handler, err := NewHandler(
		context.Background(),
		WithID(tokenKPriceReq.ID, true),
		WithPrice(tokenKPriceReq.Price, false),
	)
	assert.Nil(t, err)
	err = handler.UpdateKPrice(context.Background())
	assert.Nil(t, err)

	info, err := handler.GetKPrice(context.Background())
	assert.Nil(t, err)
	tokenKPriceRet.UpdatedAt = info.UpdatedAt
	tokenKPriceRet.CreatedAt = info.CreatedAt
	assert.Equal(t, tokenKPriceRet, info)
}

func deleteKP(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithID(tokenKPriceReq.ID, true),
	)
	assert.Nil(t, err)

	err = handler.DeleteKPrice(context.Background())
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
