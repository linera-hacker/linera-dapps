package tokenpair

import (
	"context"

	tokenpair "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/tokenpair"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	tokenpair.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	tokenpair.RegisterManagerServer(server, &Server{})
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	return tokenpair.RegisterManagerHandlerFromEndpoint(context.Background(), mux, endpoint, opts)
}
