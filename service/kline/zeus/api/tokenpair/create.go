//nolint:nolintlint,dupl
package tokenpair

import (
	"context"
	"fmt"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	tokenpairproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/tokenpair"
	tokenpair "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/tokenpair"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) CreateTokenPair(ctx context.Context, in *tokenpairproto.CreateTokenPairRequest) (*tokenpairproto.CreateTokenPairResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"CreateTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.CreateTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := tokenpair.NewHandler(
		ctx,
		tokenpair.WithPoolID(req.PoolID, true),
		tokenpair.WithTokenZeroID(req.TokenZeroID, true),
		tokenpair.WithTokenOneID(req.TokenOneID, true),
		tokenpair.WithRemark(req.Remark, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.CreateTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.CreateTokenPair(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateTokenPair",
			"In", in,
			"Error", err,
		)
		return &tokenpairproto.CreateTokenPairResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenpairproto.CreateTokenPairResponse{}, nil
}
