//nolint:nolintlint,dupl
package transaction

import (
	"context"

	"github.com/Geapefurit/kline-back/proto/kline"
	transactionproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/transaction"
	transaction "github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/transaction"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"

	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/status"
)

func (s *Server) GetTransaction(ctx context.Context, in *transactionproto.GetTransactionRequest) (*transactionproto.GetTransactionResponse, error) {
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithID(&in.ID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.GetTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	info, err := handler.GetTransaction(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTransaction",
			"In", in,
			"Error", err,
		)
		return &transactionproto.GetTransactionResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.GetTransactionResponse{
		Info: info,
	}, nil
}

func (s *Server) GetTransactions(ctx context.Context, in *transactionproto.GetTransactionsRequest) (*transactionproto.GetTransactionsResponse, error) {
	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithConds(in.Conds),
		transaction.WithOffset(in.GetOffset()),
		transaction.WithLimit(in.GetLimit()),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTransactions",
			"In", in,
			"Error", err,
		)
		return &transactionproto.GetTransactionsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	infos, total, err := handler.GetTransactions(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTransactions",
			"In", in,
			"Error", err,
		)
		return &transactionproto.GetTransactionsResponse{}, status.Error(codes.Internal, "internal server err")
	}

	return &transactionproto.GetTransactionsResponse{
		Infos: infos,
		Total: total,
	}, nil
}

func (s *Server) GetTransactionsForLine(ctx context.Context, in *transactionproto.GetTransactionsForLineRequest) (*transactionproto.GetTransactionsForLineResponse, error) {
	conds := transactionproto.Conds{
		PoolID: &kline.Uint64Val{Op: cruder.EQ, Value: in.PoolID},
	}

	handler, err := transaction.NewHandler(
		ctx,
		transaction.WithConds(&conds),
		transaction.WithOffset(in.GetOffset()),
		transaction.WithLimit(in.GetLimit()),
		transaction.WithOriginalTxID(&in.OriginalTxID, true),
	)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTransactionForLine",
			"In", in,
			"Error", err,
		)
		return &transactionproto.GetTransactionsForLineResponse{}, status.Error(codes.Internal, "internal server err")
	}

	infos, total, err := handler.GetTransactionsForLine(ctx)
	if err != nil {
		logger.Sugar().Errorw(
			"GetTransactionForLine",
			"In", in,
			"Error", err,
		)
		return &transactionproto.GetTransactionsForLineResponse{}, status.Error(codes.Internal, "internal server err")
	}
	return &transactionproto.GetTransactionsForLineResponse{
		OriginalTxID: *handler.OriginalTxID,
		PoolID:       in.PoolID,
		Offset:       in.Offset,
		Limit:        in.Limit,
		Transactions: infos,
		Total:        total,
	}, nil
}
