package transaction

import (
	"context"

	transactionproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	transaction "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/transaction"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) DeleteTransaction(ctx context.Context, in *transactionproto.DeleteTransactionRequest) (*transactionproto.DeleteTransactionResponse, error) {
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.DeleteTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.DeleteTransaction(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"DeleteTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.DeleteTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.DeleteTransactionResponse{}, nil
}
