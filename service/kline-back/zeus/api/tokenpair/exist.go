//nolint:nolintlint,dupl
package tokenpair

import (
	"context"

	tokenpairproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	tokenpair "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) ExistTokenPair(ctx context.Context, in *tokenpairproto.ExistTokenPairRequest) (*tokenpairproto.ExistTokenPairResponse, error) {
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.ExistTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistTokenPair(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.ExistTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.ExistTokenPairResponse{
		Exist: exist,
	}, nil
}

func (s *Server) ExistTokenPairConds(ctx context.Context, in *tokenpairproto.ExistTokenPairCondsRequest) (*tokenpairproto.ExistTokenPairCondsResponse, error) {
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithConds(in.Conds),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTokenPairConds",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.ExistTokenPairCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistTokenPairConds(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTokenPairConds",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.ExistTokenPairCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.ExistTokenPairCondsResponse{
		Exist: exist,
	}, nil
}
