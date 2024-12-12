package applications

type Erc20App struct {
	*baseApp
}

func NewErc20App(serverAddr, chainID, appID string) *Erc20App {
	return &Erc20App{baseApp: newBaseApp(serverAddr, chainID, appID)}
}

func (app *Erc20App) GetTokenInfo() (resp *GetErc20InfoResponse, err error) {
	resp = &GetErc20InfoResponse{}
	err = app.post(GetErc20InfoReq, resp)
	return
}
