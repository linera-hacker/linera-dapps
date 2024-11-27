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

func (s *Server) CreateKPoint(ctx context.Context, in *kpointproto.CreateKPointRequest) (*kpointproto.CreateKPointResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"CreateKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.CreateKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithTokenPairID(req.TokenPairID, true),
		kpoint.WithKPointType(req.KPointType, true),
		kpoint.WithOpen(req.Open, true),
		kpoint.WithHigh(req.High, true),
		kpoint.WithLow(req.Low, true),
		kpoint.WithClose(req.Close, true),
		kpoint.WithStartTime(req.StartTime, true),
		kpoint.WithEndTime(req.EndTime, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.CreateKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.CreateKPoint(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.CreateKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.CreateKPointResponse{}, nil
}
