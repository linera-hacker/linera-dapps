//nolint:nolintlint,dupl
package kprice

import (
	"context"
	"fmt"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	kpriceproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kprice"
	kprice "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/kprice"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) CreateKPrice(ctx context.Context, in *kpriceproto.CreateKPriceRequest) (*kpriceproto.CreateKPriceResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"CreateKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.CreateKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithTokenPairID(req.TokenPairID, true),
		kprice.WithPrice(req.Price, true),
		kprice.WithTime(req.Timestamp, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.CreateKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.CreateKPrice(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.CreateKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.CreateKPriceResponse{}, nil
}
