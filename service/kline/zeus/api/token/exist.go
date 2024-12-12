//nolint:nolintlint,dupl
package token

import (
	"context"

	tokenproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/token"
	token "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/token"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) ExistToken(ctx context.Context, in *tokenproto.ExistTokenRequest) (*tokenproto.ExistTokenResponse, error) {
	handler, err := token.NewHandler(
		ctx,
		token.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistToken",
			"In", in,
			"Error", err,
		)
		return &tokenproto.ExistTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistToken(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistToken",
			"In", in,
			"Error", err,
		)
		return &tokenproto.ExistTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenproto.ExistTokenResponse{
		Exist: exist,
	}, nil
}

func (s *Server) ExistTokenConds(ctx context.Context, in *tokenproto.ExistTokenCondsRequest) (*tokenproto.ExistTokenCondsResponse, error) {
	handler, err := token.NewHandler(
		ctx,
		token.WithConds(in.Conds),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTokenConds",
			"In", in,
			"Error", err,
		)
		return &tokenproto.ExistTokenCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistTokenConds(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTokenConds",
			"In", in,
			"Error", err,
		)
		return &tokenproto.ExistTokenCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenproto.ExistTokenCondsResponse{
		Exist: exist,
	}, nil
}
