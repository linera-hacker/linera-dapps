package kpoint

import (
	"context"

	kpointproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"
	kpoint "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kpoint"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) DeleteKPoint(ctx context.Context, in *kpointproto.DeleteKPointRequest) (*kpointproto.DeleteKPointResponse, error) {
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.DeleteKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.DeleteKPoint(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.DeleteKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.DeleteKPointResponse{}, nil
}
