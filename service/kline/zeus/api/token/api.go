package token

import (
	"context"

	token "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/token"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	token.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	token.RegisterManagerServer(server, &Server{})
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	return token.RegisterManagerHandlerFromEndpoint(context.Background(), mux, endpoint, opts)
}
