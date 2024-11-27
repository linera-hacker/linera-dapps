//nolint:nolintlint,dupl
package kprice

import (
	"context"
	"fmt"

	kpriceproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	kprice "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kprice"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) UpdateKPrice(ctx context.Context, in *kpriceproto.UpdateKPriceRequest) (*kpriceproto.UpdateKPriceResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"UpdateKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.UpdateKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithID(req.ID, true),
		kprice.WithTokenPairID(req.TokenPairID, false),
		kprice.WithPrice(req.Price, false),
		kprice.WithTime(req.Timestamp, false),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.UpdateKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.UpdateKPrice(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.UpdateKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.UpdateKPriceResponse{}, nil
}
