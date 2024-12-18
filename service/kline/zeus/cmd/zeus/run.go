package main

import (
	"fmt"
	"math/rand"
	"os"
	"os/signal"
	"syscall"
	"time"

	"github.com/linera-hacker/linera-dapps/service/kline/common/server"
	"github.com/linera-hacker/linera-dapps/service/kline/config"
	cli "github.com/urfave/cli/v2"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/beat"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
)

func init() {
	rand.Seed(time.Now().UnixNano())
}

var runCmd = &cli.Command{
	Name:    "run",
	Aliases: []string{"r"},
	Usage:   "Run NFT Meta daemon",
	After: func(c *cli.Context) error {
		return logger.Sync()
	},
	Before: func(ctx *cli.Context) error {
		return logger.Init(logger.DebugLevel, config.GetConfig().Zeus.LogFile)
	},
	Action: func(c *cli.Context) error {
		err := db.Init()
		if err != nil {
			panic(fmt.Errorf("mysql init err: %v", err))
		}

		go beat.RunSamplingKPoint(c.Context)
		go beat.RunSamplingKPrice(c.Context)
		go beat.RunSamplingTransaction(c.Context)

		go server.RunGRPCServer(config.GetConfig().Zeus.GrpcPort)
		go server.RunHTTPServer(config.GetConfig().Zeus.HTTPPort, config.GetConfig().Zeus.GrpcPort)

		sigchan := make(chan os.Signal, 1)
		signal.Notify(sigchan, syscall.SIGINT, syscall.SIGTERM)

		<-sigchan
		os.Exit(1)
		return nil
	},
}
