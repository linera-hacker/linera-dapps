//nolint:nolintlint,dupl
package tokenpair

import (
	"context"
	"fmt"

	tokenpairproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"
	tokenpair "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/tokenpair"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) UpdateTokenPair(ctx context.Context, in *tokenpairproto.UpdateTokenPairRequest) (*tokenpairproto.UpdateTokenPairResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"UpdateTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.UpdateTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithID(req.ID, true),
		tokenpair.WithRemark(req.Remark, false),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.UpdateTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.UpdateTokenPair(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.UpdateTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.UpdateTokenPairResponse{}, nil
}
