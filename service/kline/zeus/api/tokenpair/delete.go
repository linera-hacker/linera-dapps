package tokenpair

import (
	"context"

	tokenpairproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	tokenpair "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) DeleteTokenPair(ctx context.Context, in *tokenpairproto.DeleteTokenPairRequest) (*tokenpairproto.DeleteTokenPairResponse, error) {
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.DeleteTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.DeleteTokenPair(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.DeleteTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.DeleteTokenPairResponse{}, nil
}
