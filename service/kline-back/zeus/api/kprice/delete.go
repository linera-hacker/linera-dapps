package kprice

import (
	"context"

	kpriceproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	kprice "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kprice"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) DeleteKPrice(ctx context.Context, in *kpriceproto.DeleteKPriceRequest) (*kpriceproto.DeleteKPriceResponse, error) {
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.DeleteKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.DeleteKPrice(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.DeleteKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.DeleteKPriceResponse{}, nil
}
