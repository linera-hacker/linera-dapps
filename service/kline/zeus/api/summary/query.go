//nolint:nolintlint,dupl
package summary

import (
	"context"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	summaryproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/summary"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/summary"
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
	defaultNum := 8
	infos, err := summary.GetOneDayVolumnRank(ctx, defaultNum)

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
