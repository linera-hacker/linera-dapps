package token

import (
	"context"
	"fmt"
	"os"
	"strconv"
	"testing"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/google/uuid"
	"github.com/linera-hacker/linera-dapps/service/kline/proto/kline"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"

	"github.com/stretchr/testify/assert"

	tokenproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/token"
)

func init() {
	//nolint
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}
}

var tokenRet = &tokenproto.Token{
	Address: "Address",
	Site:    "basetype.Site_Ethereum",
	Icon:    "test_token",
	Name:    "10010",
	Symbol:  "10010",
}

var tokenReq = &tokenproto.TokenReq{
	Address: &tokenRet.Address,
	Site:    &tokenRet.Site,
	Icon:    &tokenRet.Icon,
	Name:    &tokenRet.Name,
	Symbol:  &tokenRet.Symbol,
}

func create(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithAddress(tokenReq.Address, true),
		WithSite(tokenReq.Site, true),
		WithIcon(tokenReq.Icon, true),
		WithName(tokenReq.Name, true),
		WithSymbol(tokenReq.Symbol, true),
	)
	assert.Nil(t, err)

	info, err := handler.CreateToken(context.Background())
	fmt.Println(err)
	if assert.Nil(t, err) {
		tokenReq.ID = &info.ID
		tokenReq.Address = &info.Address
	}
}

func update(t *testing.T) {
	siteStr := uuid.NewString()
	tokenReq.Site = &siteStr

	handler, err := NewHandler(
		context.Background(),
		WithID(tokenReq.ID, true),
		WithSite(tokenReq.Site, false),
		WithIcon(tokenReq.Icon, false),
	)
	assert.Nil(t, err)

	info, err := handler.UpdateToken(context.Background())
	if assert.Nil(t, err) {
		assert.Equal(t, info.Site, *tokenReq.Site)
	}

	handler, err = NewHandler(
		context.Background(),
		WithID(tokenReq.ID, true),
		WithIcon(tokenReq.Icon, false),
	)
	assert.Nil(t, err)

	_, err = handler.UpdateToken(context.Background())
	assert.Nil(t, err)
}

func query(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithID(tokenReq.ID, true),
	)
	assert.Nil(t, err)

	info, err := handler.GetToken(context.Background())
	assert.Nil(t, err)

	handler, err = NewHandler(
		context.Background(),
		WithConds(&tokenproto.Conds{
			Site: &kline.StringVal{
				Op:    cruder.EQ,
				Value: *tokenReq.Site,
			},
			Icon: &kline.StringVal{
				Op:    cruder.EQ,
				Value: *tokenReq.Icon,
			},
			Name: &kline.StringVal{
				Op:    cruder.EQ,
				Value: *tokenReq.Name,
			},
		}),
		WithOffset(0),
		WithLimit(1),
	)
	assert.Nil(t, err)

	infos, total, err := handler.GetTokens(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, total, uint32(1))
	assert.Equal(t, infos[0], info)
}

func deleteToken(t *testing.T) {
	handler, err := NewHandler(
		context.Background(),
		WithID(tokenReq.ID, true),
	)
	assert.Nil(t, err)

	_info1, err := handler.DeleteToken(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, *tokenReq.ID, _info1.ID)
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

	t.Run("create", create)
	t.Run("update", update)
	t.Run("query", query)
	t.Run("query", deleteToken)
}
