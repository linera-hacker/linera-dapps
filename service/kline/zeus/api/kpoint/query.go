//nolint:nolintlint,dupl
package kpoint

import (
	"context"
	"sort"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/danced25519/linera-dapps/service/kline/common/kptype"
	"github.com/danced25519/linera-dapps/service/kline/proto/kline"
	kpointproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/kpoint"
	kpoint "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/mw/v1/kpoint"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) GetKPoint(ctx context.Context, in *kpointproto.GetKPointRequest) (*kpointproto.GetKPointResponse, error) {
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.GetKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	info, err := handler.GetKPoint(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPoint",
			"In", in,
			"Error", err,
		)
		return &kpointproto.GetKPointResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.GetKPointResponse{
		Info: info,
	}, nil
}

func (s *Server) GetKPoints(ctx context.Context, in *kpointproto.GetKPointsRequest) (*kpointproto.GetKPointsResponse, error) {
	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithConds(in.Conds),
		kpoint.WithOffset(in.GetOffset()),
		kpoint.WithLimit(in.GetLimit()),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPoints",
			"In", in,
			"Error", err,
		)
		return &kpointproto.GetKPointsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	infos, total, err := handler.GetKPoints(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPoints",
			"In", in,
			"Error", err,
		)
		return &kpointproto.GetKPointsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &kpointproto.GetKPointsResponse{
		Infos: infos,
		Total: total,
	}, nil
}

func (s *Server) GetKPointTypes(ctx context.Context, in *kpointproto.GetKPointTypesRequest) (*kpointproto.GetKPointTypesResponse, error) {
	ret := []*kpointproto.KPointTypeInfo{}
	for _, v := range kptype.KPointTypeInfos {
		ret = append(ret, v)
	}

	sort.Slice(ret, func(i, j int) bool {
		return ret[i].Seconds < ret[j].Seconds
	})

	return &kpointproto.GetKPointTypesResponse{Infos: ret}, nil
}

func (s *Server) GetKPointsForLine(ctx context.Context, in *kpointproto.GetKPointsForLineRequest) (*kpointproto.GetKPointsForLineResponse, error) {
	conds := kpointproto.Conds{
		KPointType:  &kline.Uint32Val{Op: cruder.EQ, Value: uint32(in.KPointType)},
		TokenPairID: &kline.Uint32Val{Op: cruder.EQ, Value: in.TokenPairID},
	}

	handler, err := kpoint.NewHandler(
		ctx,
		kpoint.WithConds(&conds),
		kpoint.WithOffset(in.GetOffset()),
		kpoint.WithLimit(in.GetLimit()),
		kpoint.WithOriginalTime(&in.OriginalTime, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPointForLine",
			"In", in,
			"Error", err,
		)
		return &kpointproto.GetKPointsForLineResponse{}, status.Error(codes.Internal, "internal server err")
	}

	infos, total, err := handler.GetKPointsForLine(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetKPointForLine",
			"In", in,
			"Error", err,
		)
		return &kpointproto.GetKPointsForLineResponse{}, status.Error(codes.Internal, "internal server err")
	}
	return &kpointproto.GetKPointsForLineResponse{
		OriginalTime: *handler.OriginalTime,
		TokenPairID:  in.TokenPairID,
		KPointType:   in.KPointType,
		Offset:       in.Offset,
		Limit:        in.Limit,
		KPoints:      infos,
		Total:        total,
	}, nil
}
