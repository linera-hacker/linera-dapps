package api

import (
	"os"
	"strconv"
	"testing"

	_ "github.com/NpoolPlatform/go-service-framework/pkg/version"
)

func TestVersion(t *testing.T) {
	//nolint
	if runByGithubAction, err := strconv.ParseBool(os.Getenv("RUN_BY_GITHUB_ACTION")); err == nil && runByGithubAction {
		return
	}
}
