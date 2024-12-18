package kprice

import (
	kprice "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kprice"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	kprice.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	kprice.RegisterManagerServer(server, &Server{})
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	return nil
}
