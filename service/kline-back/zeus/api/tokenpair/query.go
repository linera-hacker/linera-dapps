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

func (s *Server) GetTokenPair(ctx context.Context, in *tokenpairproto.GetTokenPairRequest) (*tokenpairproto.GetTokenPairResponse, error) {
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.GetTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	info, err := handler.GetTokenPair(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.GetTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.GetTokenPairResponse{
		Info: info,
	}, nil
}

func (s *Server) GetTokenPairs(ctx context.Context, in *tokenpairproto.GetTokenPairsRequest) (*tokenpairproto.GetTokenPairsResponse, error) {
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithConds(in.Conds),
		tokenpair.WithOffset(in.GetOffset()),
		tokenpair.WithLimit(in.GetLimit()),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTokenPairs",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.GetTokenPairsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	infos, total, err := handler.GetTokenPairs(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTokenPairs",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.GetTokenPairsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.GetTokenPairsResponse{
		Infos: infos,
		Total: total,
	}, nil
}
