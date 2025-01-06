package kprice

import (
	"context"
	"os"
	"strconv"
	"testing"

	"github.com/stretchr/testify/assert"

	tokenproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/token"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/token"
)

func init() {
	//nolint
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}
}

var token1Ret = &tokenproto.Token{
	Address: "AddressOne",
	Site:    "basetype.Site_Ethereum",
	Icon:    "test_token",
	Name:    "10010",
	Symbol:  "10010",
}

var token2Ret = &tokenproto.Token{
	Address: "AddressTwo",
	Site:    "basetype.Site_Ethereum",
	Icon:    "test_token",
	Name:    "10010",
	Symbol:  "10010",
}

var token1Req = &tokenproto.TokenReq{
	Address: &token1Ret.Address,
	Site:    &token1Ret.Site,
	Icon:    &token1Ret.Icon,
	Name:    &token1Ret.Name,
	Symbol:  &token1Ret.Symbol,
}

var token2Req = &tokenproto.TokenReq{
	Address: &token1Ret.Address,
	Site:    &token1Ret.Site,
	Icon:    &token1Ret.Icon,
	Name:    &token1Ret.Name,
	Symbol:  &token1Ret.Symbol,
}

//nolint:dupl
func createToken1(t *testing.T) {
	handler, err := token.NewHandler(
		context.Background(),
		token.WithAddress(token1Req.Address, true),
		token.WithSite(token1Req.Site, true),
		token.WithIcon(token1Req.Icon, true),
		token.WithName(token1Req.Name, true),
		token.WithSymbol(token1Req.Symbol, true),
	)
	assert.Nil(t, err)

	info, err := handler.CreateToken(context.Background())
	if assert.Nil(t, err) {
		token1Req.ID = &info.ID
		token1Ret.ID = info.ID
		token1Req.Address = &info.Address
	}
}

//nolint:dupl
func createToken2(t *testing.T) {
	handler, err := token.NewHandler(
		context.Background(),
		token.WithAddress(token2Req.Address, true),
		token.WithSite(token2Req.Site, true),
		token.WithIcon(token2Req.Icon, true),
		token.WithName(token2Req.Name, true),
		token.WithSymbol(token2Req.Symbol, true),
	)
	assert.Nil(t, err)

	info, err := handler.CreateToken(context.Background())
	if assert.Nil(t, err) {
		token2Req.ID = &info.ID
		token2Ret.ID = info.ID
		token2Req.Address = &info.Address
	}
}

func deleteToken2(t *testing.T) {
	handler, err := token.NewHandler(
		context.Background(),
		token.WithID(token2Req.ID, true),
	)
	assert.Nil(t, err)

	_info1, err := handler.DeleteToken(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, *token2Req.ID, _info1.ID)
}

func deleteToken1(t *testing.T) {
	handler, err := token.NewHandler(
		context.Background(),
		token.WithID(token1Req.ID, true),
	)
	assert.Nil(t, err)

	_info1, err := handler.DeleteToken(context.Background())
	assert.Nil(t, err)
	assert.Equal(t, *token1Req.ID, _info1.ID)
}
