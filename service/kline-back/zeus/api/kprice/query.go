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

func (s *Server) GetKPrice(ctx context.Context, in *kpriceproto.GetKPriceRequest) (*kpriceproto.GetKPriceResponse, error) {
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.GetKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	info, err := handler.GetKPrice(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPrice",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.GetKPriceResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.GetKPriceResponse{
		Info: info,
	}, nil
}

func (s *Server) GetKPrices(ctx context.Context, in *kpriceproto.GetKPricesRequest) (*kpriceproto.GetKPricesResponse, error) {
	handler, err := kprice.NewHandler(
		ctx,
		kprice.WithConds(in.Conds),
		kprice.WithOffset(in.GetOffset()),
		kprice.WithLimit(in.GetLimit()),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPrices",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.GetKPricesResponse{}, status.Error(codes.Internal, "internal server err")
	}

	infos, total, err := handler.GetKPrices(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPrices",
			"In", in,
			"Error", err,
		)
		return &kpriceproto.GetKPricesResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpriceproto.GetKPricesResponse{
		Infos: infos,
		Total: total,
	}, nil
}
