package version

import (
	"fmt"

	"github.com/Geapefurit/kline-back/proto/kline"

	logger "github.com/NpoolPlatform/go-service-framework/pkg/logger"
	cv "github.com/NpoolPlatform/go-service-framework/pkg/version"
)

func Version() (*kline.VersionResponse, error) {
	info, err := cv.GetVersion()
	if err != nil {
		logger.Sugar().Errorf("get service version error: %+w", err)
		return nil, fmt.Errorf("get service version error: %w", err)
	}
	return &kline.VersionResponse{
		Info: info,
	}, nil
}
