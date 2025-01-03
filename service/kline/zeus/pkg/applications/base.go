package applications

import (
	"fmt"
	"net/http"

	"github.com/go-resty/resty/v2"
)

type baseApp struct {
	serverAddr string
	chainID    string
	appID      string
}

func newBaseApp(serverAddr, chainID, appID string) *baseApp {
	return &baseApp{serverAddr: serverAddr, chainID: chainID, appID: appID}
}

func (app *baseApp) post(req, resp interface{}) error {
	url := fmt.Sprintf("%v/chains/%v/applications/%v", app.serverAddr, app.chainID, app.appID)

	headers := make(map[string]string)
	headers["Content-Type"] = "application/json"
	ret, err := resty.
		New().
		SetHeaders(headers).
		R().
		SetBody(req).
		SetResult(resp).
		Post(url)

	if err != nil {
		return err
	}

	if ret.StatusCode() != http.StatusOK {
		return fmt.Errorf("wrong response,status code: %v,response: %v", ret.StatusCode(), string(ret.Body()))
	}
	return nil
}
