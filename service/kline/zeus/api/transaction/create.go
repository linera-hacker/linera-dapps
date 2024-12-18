//nolint:nolintlint,dupl
package transaction

import (
	"context"
	"fmt"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	transactionproto "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	transaction "github.com/danced25519/linera-dapps/service/kline/zeus/pkg/mw/v1/transaction"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) CreateTransaction(ctx context.Context, in *transactionproto.CreateTransactionRequest) (*transactionproto.CreateTransactionResponse, error) {
	if in.GetInfo() == nil {
		err := fmt.Errorf("request is nil")
		logger.Sugar().Errorw(
			"CreateTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.CreateTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	req := in.GetInfo()
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithPoolID(req.PoolID, true),
		transaction.WithTransactionID(req.TransactionID, true),
		transaction.WithTransactionType(req.TransactionType, true),
		transaction.WithChainID(req.ChainID, true),
		transaction.WithOwner(req.Owner, true),
		transaction.WithAmountZeroIn(req.AmountZeroIn, false),
		transaction.WithAmountOneIn(req.AmountOneIn, false),
		transaction.WithAmountZeroOut(req.AmountZeroOut, false),
		transaction.WithAmountOneOut(req.AmountOneOut, false),
		transaction.WithTimestamp(req.Timestamp, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.CreateTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	err = handler.CreateTransaction(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"CreateTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.CreateTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.CreateTransactionResponse{}, nil
}
