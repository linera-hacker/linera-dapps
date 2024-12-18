//nolint:nolintlint,dupl
package kpoint

import (
	"context"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	kpointproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/kpoint"
	kpoint "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/mw/v1/kpoint"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) ExistKPoint(ctx context.Context, in *kpointproto.ExistKPointRequest) (*kpointproto.ExistKPointResponse, error) {
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.ExistKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistKPoint(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.ExistKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.ExistKPointResponse{
		Exist: exist,
	}, nil
}

func (s *Server) ExistKPointConds(ctx context.Context, in *kpointproto.ExistKPointCondsRequest) (*kpointproto.ExistKPointCondsResponse, error) {
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithConds(in.Conds),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPointConds",
			"In", in,
			"Error", err,
		)
		return &kpointproto.ExistKPointCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistKPointConds(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPointConds",
			"In", in,
			"Error", err,
		)
		return &kpointproto.ExistKPointCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.ExistKPointCondsResponse{
		Exist: exist,
	}, nil
}
