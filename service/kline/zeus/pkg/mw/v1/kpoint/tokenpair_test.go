package kpoint

import (
	"context"
	"fmt"
	"os"
	"strconv"
	"testing"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/linera-hacker/linera-dapps/service/kline/proto/kline"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/tokenpair"

	"github.com/stretchr/testify/assert"

	tokenpairproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/tokenpair"
)

func init() {
	//nolint
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}
}

var tokenPTRet = &tokenpairproto.TokenPair{
	Remark: "test token pair",
}

var tokenPTReq = &tokenpairproto.TokenPairReq{
	Remark: &tokenPTRet.Remark,
}

func createTP(t *testing.T) {
	tokenPTRet.TokenZeroID = token1Ret.ID
	tokenPTRet.TokenOneID = token2Ret.ID
	tokenPTReq.TokenZeroID = &token1Ret.ID
	tokenPTReq.TokenOneID = &token2Ret.ID

	handler, err := tokenpair.NewHandler(
		context.Background(),
		tokenpair.WithTokenZeroID(tokenPTReq.TokenZeroID, true),
		tokenpair.WithTokenOneID(tokenPTReq.TokenOneID, true),
		tokenpair.WithRemark(tokenPTReq.Remark, true),
	)
	assert.Nil(t, err)

	err = handler.CreateTokenPair(context.Background())
	assert.Nil(t, err)

	handler, err = tokenpair.NewHandler(
		context.Background(),
		tokenpair.WithConds(&tokenpairproto.Conds{
			TokenZeroID: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: *tokenPTReq.TokenZeroID,
			},
			TokenOneID: &kline.Uint32Val{
				Op:    cruder.EQ,
				Value: *tokenPTReq.TokenOneID,
			},
			Remark: &kline.StringVal{
				Op:    cruder.EQ,
				Value: *tokenPTReq.Remark,
			},
		}),
		tokenpair.WithOffset(0),
		tokenpair.WithLimit(1),
	)
	assert.Nil(t, err)
	infos, total, err := handler.GetTokenPairs(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, total, uint32(1))

	tokenPTReq.ID = &infos[0].ID
	tokenPTRet.ID = infos[0].ID
}

func deleteTP(t *testing.T) {
	handler, err := tokenpair.NewHandler(
		context.Background(),
		tokenpair.WithID(tokenPTReq.ID, true),
	)
	assert.Nil(t, err)

	err = handler.DeleteTokenPair(context.Background())
	assert.Nil(t, err)
}

func TestTx(t *testing.T) {
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
	t.Run("create", createTP)
	t.Run("delete", deleteTP)
	t.Run("deleteToken1", deleteToken1)
	t.Run("deleteToken2", deleteToken2)
}
