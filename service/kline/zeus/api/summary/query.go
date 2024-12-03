//nolint:nolintlint,dupl
package summary

import (
	"context"

	summaryproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/summary"
	"github.com/Geapefurit/kline-back/zeus/pkg/summary"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) GetTokenLastCond(ctx context.Context, in *summaryproto.GetTokenLastCondRequest) (*summaryproto.GetTokenLastCondResponse, error) {
	info, err := summary.GetTokenLastCond(ctx, in.PoolID, in.TokenZeroAddress, in.TokenOneAddress)

	if err != nil {
		logger.Sugar().Errorw(
			"GetTokenLastCond",
			"In", in,
			"Error", err,
		)
		return &summaryproto.GetTokenLastCondResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &summaryproto.GetTokenLastCondResponse{
		Info: info,
	}, nil
}

func (s *Server) GetTokenLastConds(ctx context.Context, in *summaryproto.GetTokenLastCondsRequest) (*summaryproto.GetTokenLastCondsResponse, error) {
	infos, err := summary.GetTokenLastConds(ctx, in.PoolTokenConds)

	if err != nil {
		logger.Sugar().Errorw(
			"GetTokenLastConds",
			"In", in,
			"Error", err,
		)
		return &summaryproto.GetTokenLastCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &summaryproto.GetTokenLastCondsResponse{
		Infos: infos,
	}, nil
}

func (s *Server) GetOneDayVolumn(ctx context.Context, in *summaryproto.GetOneDayVolumnRequest) (*summaryproto.GetOneDayVolumnResponse, error) {
	infos, err := summary.GetOneDayVolumnRank(ctx, 8)

	if err != nil {
		logger.Sugar().Errorw(
			"GetOneDayVolumn",
			"In", in,
			"Error", err,
		)
		return &summaryproto.GetOneDayVolumnResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &summaryproto.GetOneDayVolumnResponse{
		Infos: infos,
	}, nil
}
