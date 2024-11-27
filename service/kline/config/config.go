package config

import (
	"bytes"
	_ "embed"
	"fmt"
	"os"
	"reflect"

	"github.com/BurntSushi/toml"
)

//go:embed config.toml
var ConfigFS string

type Config struct {
	Version string  `toml:"version" env:"version"`
	Zeus    Zeus    `toml:"zeus" env:"zeus"`
	MySQL   MySQL   `toml:"mysql" env:"mysql"`
	SwapApp SwapApp `toml:"swap-app" env:"swap_app"`
}

type Zeus struct {
	Domain         string `toml:"domain" env:"domain"`
	HTTPPort       int    `toml:"http-port" env:"http_port"`
	GrpcPort       int    `toml:"grpc-port" env:"grpc_port"`
	LogFile        string `toml:"log-file" env:"log_file"`
	CollectionName string `toml:"collection-name" env:"collection_name"`
	Description    string `toml:"description" env:"description"`
}

type MySQL struct {
	Domain   string `toml:"domain" env:"domain"`
	Port     int    `toml:"port" env:"port"`
	User     string `toml:"user" env:"user"`
	Password string `toml:"password" env:"password"`
	Database string `toml:"database" env:"database"`
}

type SwapApp struct {
	ServerAddr string `toml:"server-addr" env:"server_addr"`
	ChainID    string `toml:"chain-id" env:"chain_id"`
	AppID      string `toml:"app-id" env:"app_id"`
}

// set default config
var config = &Config{
	Zeus: Zeus{
		HTTPPort: 30100,
		GrpcPort: 30101,
	},
}

type envMatcher struct {
	envMap map[string]string
}

func DetectEnv(co *Config) (err error) {
	e := &envMatcher{}
	e.envMap = make(map[string]string)
	ct := reflect.TypeOf(co)
	e.detectEnv(ct, "", "")
	_, err = toml.Decode(e.toToml(), co)
	return err
}

// read environment var
func (e *envMatcher) detectEnv(t reflect.Type, preffix, _preffix string) {
	if t.Kind() == reflect.Ptr {
		t = t.Elem()
	}
	for i := 0; i < t.NumField(); i++ {
		it := t.Field(i)
		envKey := fmt.Sprintf("%v%v", preffix, it.Tag.Get("env"))
		_envKey := fmt.Sprintf("%v%v", _preffix, it.Tag.Get("toml"))
		if it.Type.Kind() != reflect.Struct {
			if envValue, ok := os.LookupEnv(envKey); ok {
				if it.Type.Kind() == reflect.String {
					e.envMap[_envKey] = fmt.Sprintf("\"%v\"", envValue)
				} else {
					e.envMap[_envKey] = envValue
				}
			}
			continue
		}
		envKey = fmt.Sprintf("%v%v_", preffix, it.Tag.Get("env"))
		_envKey = fmt.Sprintf("%v%v.", _preffix, it.Tag.Get("toml"))
		e.detectEnv(it.Type, envKey, _envKey)
	}
}

func (e *envMatcher) toToml() string {
	var b bytes.Buffer

	for v := range e.envMap {
		b.WriteString(fmt.Sprintf("%v=%v\n", v, e.envMap[v]))
	}

	return b.String()
}

func init() {
	md, err := toml.Decode(ConfigFS, config)
	if err != nil {
		panic(fmt.Sprintf("failed to parse config file, %v", err))
	}
	if len(md.Undecoded()) > 0 {
		panic(fmt.Sprintf("cannot parse [%v] to config\n", md.Undecoded()))
	}
	err = DetectEnv(config)
	if err != nil {
		panic(fmt.Sprintf("environment variable parse failed, %v", err))
	}
}

func GetConfig() *Config {
	return config
}
