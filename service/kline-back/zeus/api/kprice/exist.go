//nolint:nolintlint,dupl
package kprice

import (
	"context"

	kpriceproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	kprice "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kprice"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) ExistKPrice(ctx context.Context, in *kpriceproto.ExistKPriceRequest) (*kpriceproto.ExistKPriceResponse, error) {
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.ExistKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistKPrice(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.ExistKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.ExistKPriceResponse{
		Exist: exist,
	}, nil
}

func (s *Server) ExistKPriceConds(ctx context.Context, in *kpriceproto.ExistKPriceCondsRequest) (*kpriceproto.ExistKPriceCondsResponse, error) {
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithConds(in.Conds),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPriceConds",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.ExistKPriceCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistKPriceConds(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistKPriceConds",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.ExistKPriceCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.ExistKPriceCondsResponse{
		Exist: exist,
	}, nil
}
