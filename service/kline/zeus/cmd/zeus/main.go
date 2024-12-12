package main

import (
	"fmt"
	"log"
	"os"

	"github.com/Geapefurit/kline-back/zeus/pkg/servicename"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/NpoolPlatform/go-service-framework/pkg/version"
	banner "github.com/common-nighthawk/go-figure"
	cli "github.com/urfave/cli/v2"
)

const (
	usageText = "Zeus Service"
)

func main() {
	commands := cli.Commands{runCmd}

	description := fmt.Sprintf(
		"%v service cli\nFor help on any individual command run <%v COMMAND -h>\n",
		servicename.ServiceName,
		servicename.ServiceName,
	)
	banner.NewColorFigure(servicename.ServiceName, "", "green", true).Print()
	vesion, err := version.GetVersion()
	if err != nil {
		log.Fatalf("fail to get version, %v", err)
	}

	app := &cli.App{
		Name:        servicename.ServiceName,
		Version:     vesion,
		Description: description,
		Usage:       usageText,
		Commands:    commands,
	}

	if err != nil {
		logger.Sugar().Errorf("fail to create %v: %v", servicename.ServiceName, err)
		return
	}

	err = app.Run(os.Args)
	if err != nil {
		log.Fatalf("fail to run %v: %v", servicename.ServiceName, err)
	}
}
