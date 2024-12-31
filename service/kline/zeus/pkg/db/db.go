package db

import (
	"context"
	"database/sql"
	"fmt"
	"net"
	"sync"
	"time"

	"github.com/NpoolPlatform/go-service-framework/pkg/logger"

	"github.com/linera-hacker/linera-dapps/service/kline/config"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"

	"entgo.io/ent/dialect"
	entsql "entgo.io/ent/dialect/sql"

	// ent policy runtime
	_ "github.com/go-sql-driver/mysql"
	_ "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/runtime"
)

type db struct {
	db      *sql.DB
	address string
}

var (
	mu        = sync.Mutex{}
	mysqlConn *db
)

func client() (*ent.Client, error) {

	conn, err := GetConn()
	if err != nil {
		return nil, err
	}

	drv := entsql.OpenDB(dialect.MySQL, conn)
	return ent.NewClient(ent.Driver(drv)), nil
}

func GetConn() (conn *sql.DB, err error) {
	masterMysqlIP, err := GetMasterIP()
	if err != nil {
		return nil, err
	}

	mu.Lock()
	if mysqlConn != nil {
		conn = mysqlConn.db
		mu.Unlock()
		return
	}
	mu.Unlock()

	myConfig := config.GetConfig().MySQL
	dataSourceName := fmt.Sprintf("%v:%v@tcp(%v:%v)/%v?parseTime=true&interpolateParams=true",
		myConfig.User, myConfig.Password,
		masterMysqlIP,
		myConfig.Port,
		myConfig.Database,
	)

	conn, err = open("mysql", dataSourceName)
	if err != nil {
		logger.Sugar().Warnf("call open error: %v", err)
		return nil, err
	}

	return
}

func open(driverName, dataSourceName string) (conn *sql.DB, err error) {
	// double lock check
	mu.Lock()
	if mysqlConn != nil && mysqlConn.address == dataSourceName {
		conn = mysqlConn.db
		mu.Unlock()
		return
	}

	logger.Sugar().Infof("Reopen database %v: %v", driverName, dataSourceName)
	conn, err = sql.Open(driverName, dataSourceName)
	if err != nil {
		mu.Unlock()
		logger.Sugar().Warnf("call Open error: %v", err)
		return nil, err
	}

	// https://github.com/go-sql-driver/mysql
	// See "Important settings" section.
	conn.SetConnMaxLifetime(time.Minute * 10)
	conn.SetMaxOpenConns(10)
	conn.SetMaxIdleConns(2)

	// maybe should close
	if mysqlConn != nil {
		mysqlConn.db.Close()
	}

	mysqlConn = &db{db: conn, address: dataSourceName}
	mu.Unlock()

	return conn, nil
}

func InitDatabase() error {
	mu.Lock()
	defer mu.Unlock()
	masterMysqlIP, err := GetMasterIP()
	if err != nil {
		return err
	}
	myConfig := config.GetConfig().MySQL

	withoutDBMSN := fmt.Sprintf("%v:%v@tcp(%v:%v)/?parseTime=true&interpolateParams=true",
		myConfig.User, myConfig.Password,
		masterMysqlIP,
		myConfig.Port,
	)

	createSQL := fmt.Sprintf("create database if not exists %v;", myConfig.Database)
	conn, err := sql.Open("mysql", withoutDBMSN)
	if err != nil {
		logger.Sugar().Warnf("call Open error: %v", err)
		return err
	}

	_, err = conn.Exec(createSQL)
	if err != nil {
		logger.Sugar().Warnf("exec sql failed: %v", err)
		return err
	}
	return conn.Close()
}

func GetMasterIP() (string, error) {
	myConfig := config.GetConfig().MySQL

	ip := net.ParseIP(myConfig.Domain)
	if ip != nil {
		return ip.String(), nil
	}

	ips, err := net.LookupHost(myConfig.Domain)
	if err != nil {
		return "", err
	}
	for _, ip := range ips {
		withoutDBMSN := fmt.Sprintf("%v:%v@tcp(%v:%v)/?parseTime=true&interpolateParams=true",
			myConfig.User, myConfig.Password,
			ip,
			myConfig.Port,
		)

		checkReadOnly := "SELECT @@read_only;"
		s := ""
		func() {
			conn, err := sql.Open("mysql", withoutDBMSN)
			if err != nil {
				logger.Sugar().Warnf("call Open error: %v, ip: %v", err, ip)
			}
			defer conn.Close()

			result := conn.QueryRow(checkReadOnly)
			if err != nil {
				logger.Sugar().Warnf("check read only failed: %v, ip: %v", err, ip)
			}
			err = result.Scan(&s)
			if err != nil {
				logger.Sugar().Warnf("check read only failed: %v, ip: %v", err, ip)
			}
		}()

		if s == "0" {
			return ip, nil
		}
	}
	return "", fmt.Errorf("cannot find mysql master node")
}

func Init() error {
	var err error

	err = InitDatabase()
	if err != nil {
		panic(err)
	}

	cli, err := client()
	if err != nil {
		panic(err)
	}

	return cli.Schema.Create(context.Background())
}

func Client() (*ent.Client, error) {
	return client()
}

func WithTx(ctx context.Context, fn func(ctx context.Context, tx *ent.Tx) error) error {
	cli, err := Client()
	if err != nil {
		return err
	}
	// defer cli.Close()

	tx, err := cli.Tx(ctx)
	if err != nil {
		return fmt.Errorf("fail get client transaction: %v", err)
	}

	succ := false
	defer func() {
		if !succ {
			err := tx.Rollback()
			if err != nil {
				logger.Sugar().Errorf("fail rollback: %v", err)
				return
			}
		}
	}()

	if err := fn(ctx, tx); err != nil {
		return err
	}

	if err := tx.Commit(); err != nil {
		return fmt.Errorf("committing transaction: %v", err)
	}

	succ = true
	return nil
}

func WithClient(ctx context.Context, fn func(ctx context.Context, cli *ent.Client) error) error {
	cli, err := Client()
	if err != nil {
		return fmt.Errorf("fail get db client: %v", err)
	}
	// defer cli.Close()

	if err := fn(ctx, cli); err != nil {
		return err
	}
	return nil
}
