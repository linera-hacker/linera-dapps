//nolint:nolintlint,dupl
package transaction

import (
	"context"
	"fmt"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	transactionproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	transaction "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/transaction"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) UpdateTransaction(ctx context.Context, in *transactionproto.UpdateTransactionRequest) (*transactionproto.UpdateTransactionResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"UpdateTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.UpdateTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithID(req.ID, true),
		transaction.WithAmountZeroIn(req.AmountZeroIn, false),
		transaction.WithAmountOneIn(req.AmountOneIn, false),
		transaction.WithAmountZeroOut(req.AmountZeroOut, false),
		transaction.WithAmountOneOut(req.AmountOneOut, false),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.UpdateTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.UpdateTransaction(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"UpdateTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.UpdateTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.UpdateTransactionResponse{}, nil
}
