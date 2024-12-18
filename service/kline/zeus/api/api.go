package api

import (
	"context"

	mw "github.com/danced25519/linera-dapps/service/kline/proto/kline/zeus/v1"

	"github.com/danced25519/linera-dapps/service/kline/zeus/api/kpoint"
	"github.com/danced25519/linera-dapps/service/kline/zeus/api/kprice"
	"github.com/danced25519/linera-dapps/service/kline/zeus/api/summary"
	"github.com/danced25519/linera-dapps/service/kline/zeus/api/token"
	"github.com/danced25519/linera-dapps/service/kline/zeus/api/tokenpair"
	"github.com/danced25519/linera-dapps/service/kline/zeus/api/transaction"

	"github.com/grpc-ecosystem/grpc-gateway/v2/runtime"
	"google.golang.org/grpc"
)

type Server struct {
	mw.UnimplementedManagerServer
}

func Register(server grpc.ServiceRegistrar) {
	mw.RegisterManagerServer(server, &Server{})
	kprice.Register(server)
	kpoint.Register(server)
	token.Register(server)
	tokenpair.Register(server)
	transaction.Register(server)
	summary.Register(server)
}

func RegisterGateway(mux *runtime.ServeMux, endpoint string, opts []grpc.DialOption) error {
	if err := mw.RegisterManagerHandlerFromEndpoint(context.Background(), mux, endpoint, opts); err != nil {
		return err
	}
	if err := kprice.RegisterGateway(mux, endpoint, opts); err != nil {
		return err
	}
	if err := kpoint.RegisterGateway(mux, endpoint, opts); err != nil {
		return err
	}
	if err := token.RegisterGateway(mux, endpoint, opts); err != nil {
		return err
	}
	if err := tokenpair.RegisterGateway(mux, endpoint, opts); err != nil {
		return err
	}
	if err := transaction.RegisterGateway(mux, endpoint, opts); err != nil {
		return err
	}
	if err := summary.RegisterGateway(mux, endpoint, opts); err != nil {
		return err
	}
	return nil
}
