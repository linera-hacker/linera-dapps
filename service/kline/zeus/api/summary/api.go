package summary

import (
	"context"

	summary "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/summary"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	summary.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	summary.RegisterManagerServer(server, &Server{})
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	return summary.RegisterManagerHandlerFromEndpoint(context.Background(), mux, endpoint, opts)
}
