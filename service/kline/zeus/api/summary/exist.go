//nolint:nolintlint,dupl
package summary

import (
	"context"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/danced25519/linera-dapps/service/kline/proto/kline"
	summaryproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/summary"
	tokenproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/token"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/mw/v1/token"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) ExistToken(ctx context.Context, in *summaryproto.ExistTokenRequest) (*summaryproto.ExistTokenResponse, error) {
	conds := &tokenproto.Conds{
		Symbol: &kline.StringVal{
			Op:    cruder.EQ,
			Value: in.Symbol,
		},
	}
	handler, err := token.NewHandler(
		ctx,
		token.WithConds(conds),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistToken",
			"In", in,
			"Error", err,
		)
		return &summaryproto.ExistTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistTokenConds(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistToken",
			"In", in,
			"Error", err,
		)
		return &summaryproto.ExistTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &summaryproto.ExistTokenResponse{
		Exist: exist,
	}, nil
}
