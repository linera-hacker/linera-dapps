//nolint:nolintlint,dupl
package transaction

import (
	"context"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	transactionproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"
	transaction "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/mw/v1/transaction"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) ExistTransaction(ctx context.Context, in *transactionproto.ExistTransactionRequest) (*transactionproto.ExistTransactionResponse, error) {
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.ExistTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistTransaction(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.ExistTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.ExistTransactionResponse{
		Exist: exist,
	}, nil
}

func (s *Server) ExistTransactionConds(ctx context.Context, in *transactionproto.ExistTransactionCondsRequest) (*transactionproto.ExistTransactionCondsResponse, error) {
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithConds(in.Conds),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTransactionConds",
			"In", in,
			"Error", err,
		)
		return &transactionproto.ExistTransactionCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	exist, err := handler.ExistTransactionConds(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"ExistTransactionConds",
			"In", in,
			"Error", err,
		)
		return &transactionproto.ExistTransactionCondsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.ExistTransactionCondsResponse{
		Exist: exist,
	}, nil
}
