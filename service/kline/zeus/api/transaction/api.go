package transaction

import (
	"context"

	transaction "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/transaction"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	transaction.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	transaction.RegisterManagerServer(server, &Server{})
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	return transaction.RegisterManagerHandlerFromEndpoint(context.Background(), mux, endpoint, opts)

}
