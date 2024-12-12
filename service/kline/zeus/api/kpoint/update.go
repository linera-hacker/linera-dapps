//nolint:nolintlint,dupl
package kpoint

import (
	"context"
	"fmt"

	kpointproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"
	kpoint "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kpoint"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) UpdateKPoint(ctx context.Context, in *kpointproto.UpdateKPointRequest) (*kpointproto.UpdateKPointResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"UpdateKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.UpdateKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithID(req.ID, true),
		kpoint.WithOpen(req.Open, false),
		kpoint.WithHigh(req.High, false),
		kpoint.WithLow(req.Low, false),
		kpoint.WithClose(req.Close, false),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.UpdateKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.UpdateKPoint(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.UpdateKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.UpdateKPointResponse{}, nil
}
