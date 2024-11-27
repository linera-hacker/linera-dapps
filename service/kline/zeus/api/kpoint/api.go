package kpoint

import (
	"context"

	kpoint "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	kpoint.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	kpoint.RegisterManagerServer(server, &Server{})
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	return kpoint.RegisterManagerHandlerFromEndpoint(context.Background(), mux, endpoint, opts)
}
