//nolint:nolintlint,dupl
package token

import (
	"context"
	"fmt"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	tokenproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/token"
	token "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/token"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) UpdateToken(ctx context.Context, in *tokenproto.UpdateTokenRequest) (*tokenproto.UpdateTokenResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"UpdateToken",
			"In", in,
			"Error", err,
		)
		return &tokenproto.UpdateTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := token.NewHandler(
		ctx,
		token.WithID(req.ID, true),
		token.WithSite(req.Site, false),
		token.WithIcon(req.Icon, false),
		token.WithIcon(req.Name, false),
		token.WithSymbol(req.Symbol, false),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateToken",
			"In", in,
			"Error", err,
		)
		return &tokenproto.UpdateTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	token, err := handler.UpdateToken(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateToken",
			"In", in,
			"Error", err,
		)
		return &tokenproto.UpdateTokenResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &tokenproto.UpdateTokenResponse{Info: token}, nil
}
