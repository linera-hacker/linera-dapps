// Code generated by ent, DO NOT EDIT.

package ent

import (
	"context"
	"errors"
	"fmt"
	"log"

	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/migrate"

	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/kpoint"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/kprice"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/token"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/tokenpair"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/transaction"

	"entgo.io/ent/dialect"
	"entgo.io/ent/dialect/sql"
)

// Client is the client that holds all ent builders.
type Client struct {
	config
	// Schema is the client for creating, migrating and dropping schema.
	Schema *migrate.Schema
	// KPoint is the client for interacting with the KPoint builders.
	KPoint *KPointClient
	// KPrice is the client for interacting with the KPrice builders.
	KPrice *KPriceClient
	// Token is the client for interacting with the Token builders.
	Token *TokenClient
	// TokenPair is the client for interacting with the TokenPair builders.
	TokenPair *TokenPairClient
	// Transaction is the client for interacting with the Transaction builders.
	Transaction *TransactionClient
}

// NewClient creates a new client configured with the given options.
func NewClient(opts ...Option) *Client {
	cfg := config{log: log.Println, hooks: &hooks{}}
	cfg.options(opts...)
	client := &Client{config: cfg}
	client.init()
	return client
}

func (c *Client) init() {
	c.Schema = migrate.NewSchema(c.driver)
	c.KPoint = NewKPointClient(c.config)
	c.KPrice = NewKPriceClient(c.config)
	c.Token = NewTokenClient(c.config)
	c.TokenPair = NewTokenPairClient(c.config)
	c.Transaction = NewTransactionClient(c.config)
}

// Open opens a database/sql.DB specified by the driver name and
// the data source name, and returns a new client attached to it.
// Optional parameters can be added for configuring the client.
func Open(driverName, dataSourceName string, options ...Option) (*Client, error) {
	switch driverName {
	case dialect.MySQL, dialect.Postgres, dialect.SQLite:
		drv, err := sql.Open(driverName, dataSourceName)
		if err != nil {
			return nil, err
		}
		return NewClient(append(options, Driver(drv))...), nil
	default:
		return nil, fmt.Errorf("unsupported driver: %q", driverName)
	}
}

// Tx returns a new transactional client. The provided context
// is used until the transaction is committed or rolled back.
func (c *Client) Tx(ctx context.Context) (*Tx, error) {
	if _, ok := c.driver.(*txDriver); ok {
		return nil, errors.New("ent: cannot start a transaction within a transaction")
	}
	tx, err := newTx(ctx, c.driver)
	if err != nil {
		return nil, fmt.Errorf("ent: starting a transaction: %w", err)
	}
	cfg := c.config
	cfg.driver = tx
	return &Tx{
		ctx:         ctx,
		config:      cfg,
		KPoint:      NewKPointClient(cfg),
		KPrice:      NewKPriceClient(cfg),
		Token:       NewTokenClient(cfg),
		TokenPair:   NewTokenPairClient(cfg),
		Transaction: NewTransactionClient(cfg),
	}, nil
}

// BeginTx returns a transactional client with specified options.
func (c *Client) BeginTx(ctx context.Context, opts *sql.TxOptions) (*Tx, error) {
	if _, ok := c.driver.(*txDriver); ok {
		return nil, errors.New("ent: cannot start a transaction within a transaction")
	}
	tx, err := c.driver.(interface {
		BeginTx(context.Context, *sql.TxOptions) (dialect.Tx, error)
	}).BeginTx(ctx, opts)
	if err != nil {
		return nil, fmt.Errorf("ent: starting a transaction: %w", err)
	}
	cfg := c.config
	cfg.driver = &txDriver{tx: tx, drv: c.driver}
	return &Tx{
		ctx:         ctx,
		config:      cfg,
		KPoint:      NewKPointClient(cfg),
		KPrice:      NewKPriceClient(cfg),
		Token:       NewTokenClient(cfg),
		TokenPair:   NewTokenPairClient(cfg),
		Transaction: NewTransactionClient(cfg),
	}, nil
}

// Debug returns a new debug-client. It's used to get verbose logging on specific operations.
//
//	client.Debug().
//		KPoint.
//		Query().
//		Count(ctx)
func (c *Client) Debug() *Client {
	if c.debug {
		return c
	}
	cfg := c.config
	cfg.driver = dialect.Debug(c.driver, c.log)
	client := &Client{config: cfg}
	client.init()
	return client
}

// Close closes the database connection and prevents new queries from starting.
func (c *Client) Close() error {
	return c.driver.Close()
}

// Use adds the mutation hooks to all the entity clients.
// In order to add hooks to a specific client, call: `client.Node.Use(...)`.
func (c *Client) Use(hooks ...Hook) {
	c.KPoint.Use(hooks...)
	c.KPrice.Use(hooks...)
	c.Token.Use(hooks...)
	c.TokenPair.Use(hooks...)
	c.Transaction.Use(hooks...)
}

// KPointClient is a client for the KPoint schema.
type KPointClient struct {
	config
}

// NewKPointClient returns a client for the KPoint from the given config.
func NewKPointClient(c config) *KPointClient {
	return &KPointClient{config: c}
}

// Use adds a list of mutation hooks to the hooks stack.
// A call to `Use(f, g, h)` equals to `kpoint.Hooks(f(g(h())))`.
func (c *KPointClient) Use(hooks ...Hook) {
	c.hooks.KPoint = append(c.hooks.KPoint, hooks...)
}

// Create returns a builder for creating a KPoint entity.
func (c *KPointClient) Create() *KPointCreate {
	mutation := newKPointMutation(c.config, OpCreate)
	return &KPointCreate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// CreateBulk returns a builder for creating a bulk of KPoint entities.
func (c *KPointClient) CreateBulk(builders ...*KPointCreate) *KPointCreateBulk {
	return &KPointCreateBulk{config: c.config, builders: builders}
}

// Update returns an update builder for KPoint.
func (c *KPointClient) Update() *KPointUpdate {
	mutation := newKPointMutation(c.config, OpUpdate)
	return &KPointUpdate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOne returns an update builder for the given entity.
func (c *KPointClient) UpdateOne(k *KPoint) *KPointUpdateOne {
	mutation := newKPointMutation(c.config, OpUpdateOne, withKPoint(k))
	return &KPointUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOneID returns an update builder for the given id.
func (c *KPointClient) UpdateOneID(id uint32) *KPointUpdateOne {
	mutation := newKPointMutation(c.config, OpUpdateOne, withKPointID(id))
	return &KPointUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// Delete returns a delete builder for KPoint.
func (c *KPointClient) Delete() *KPointDelete {
	mutation := newKPointMutation(c.config, OpDelete)
	return &KPointDelete{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// DeleteOne returns a builder for deleting the given entity.
func (c *KPointClient) DeleteOne(k *KPoint) *KPointDeleteOne {
	return c.DeleteOneID(k.ID)
}

// DeleteOne returns a builder for deleting the given entity by its id.
func (c *KPointClient) DeleteOneID(id uint32) *KPointDeleteOne {
	builder := c.Delete().Where(kpoint.ID(id))
	builder.mutation.id = &id
	builder.mutation.op = OpDeleteOne
	return &KPointDeleteOne{builder}
}

// Query returns a query builder for KPoint.
func (c *KPointClient) Query() *KPointQuery {
	return &KPointQuery{
		config: c.config,
	}
}

// Get returns a KPoint entity by its id.
func (c *KPointClient) Get(ctx context.Context, id uint32) (*KPoint, error) {
	return c.Query().Where(kpoint.ID(id)).Only(ctx)
}

// GetX is like Get, but panics if an error occurs.
func (c *KPointClient) GetX(ctx context.Context, id uint32) *KPoint {
	obj, err := c.Get(ctx, id)
	if err != nil {
		panic(err)
	}
	return obj
}

// Hooks returns the client hooks.
func (c *KPointClient) Hooks() []Hook {
	hooks := c.hooks.KPoint
	return append(hooks[:len(hooks):len(hooks)], kpoint.Hooks[:]...)
}

// KPriceClient is a client for the KPrice schema.
type KPriceClient struct {
	config
}

// NewKPriceClient returns a client for the KPrice from the given config.
func NewKPriceClient(c config) *KPriceClient {
	return &KPriceClient{config: c}
}

// Use adds a list of mutation hooks to the hooks stack.
// A call to `Use(f, g, h)` equals to `kprice.Hooks(f(g(h())))`.
func (c *KPriceClient) Use(hooks ...Hook) {
	c.hooks.KPrice = append(c.hooks.KPrice, hooks...)
}

// Create returns a builder for creating a KPrice entity.
func (c *KPriceClient) Create() *KPriceCreate {
	mutation := newKPriceMutation(c.config, OpCreate)
	return &KPriceCreate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// CreateBulk returns a builder for creating a bulk of KPrice entities.
func (c *KPriceClient) CreateBulk(builders ...*KPriceCreate) *KPriceCreateBulk {
	return &KPriceCreateBulk{config: c.config, builders: builders}
}

// Update returns an update builder for KPrice.
func (c *KPriceClient) Update() *KPriceUpdate {
	mutation := newKPriceMutation(c.config, OpUpdate)
	return &KPriceUpdate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOne returns an update builder for the given entity.
func (c *KPriceClient) UpdateOne(k *KPrice) *KPriceUpdateOne {
	mutation := newKPriceMutation(c.config, OpUpdateOne, withKPrice(k))
	return &KPriceUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOneID returns an update builder for the given id.
func (c *KPriceClient) UpdateOneID(id uint32) *KPriceUpdateOne {
	mutation := newKPriceMutation(c.config, OpUpdateOne, withKPriceID(id))
	return &KPriceUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// Delete returns a delete builder for KPrice.
func (c *KPriceClient) Delete() *KPriceDelete {
	mutation := newKPriceMutation(c.config, OpDelete)
	return &KPriceDelete{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// DeleteOne returns a builder for deleting the given entity.
func (c *KPriceClient) DeleteOne(k *KPrice) *KPriceDeleteOne {
	return c.DeleteOneID(k.ID)
}

// DeleteOne returns a builder for deleting the given entity by its id.
func (c *KPriceClient) DeleteOneID(id uint32) *KPriceDeleteOne {
	builder := c.Delete().Where(kprice.ID(id))
	builder.mutation.id = &id
	builder.mutation.op = OpDeleteOne
	return &KPriceDeleteOne{builder}
}

// Query returns a query builder for KPrice.
func (c *KPriceClient) Query() *KPriceQuery {
	return &KPriceQuery{
		config: c.config,
	}
}

// Get returns a KPrice entity by its id.
func (c *KPriceClient) Get(ctx context.Context, id uint32) (*KPrice, error) {
	return c.Query().Where(kprice.ID(id)).Only(ctx)
}

// GetX is like Get, but panics if an error occurs.
func (c *KPriceClient) GetX(ctx context.Context, id uint32) *KPrice {
	obj, err := c.Get(ctx, id)
	if err != nil {
		panic(err)
	}
	return obj
}

// Hooks returns the client hooks.
func (c *KPriceClient) Hooks() []Hook {
	hooks := c.hooks.KPrice
	return append(hooks[:len(hooks):len(hooks)], kprice.Hooks[:]...)
}

// TokenClient is a client for the Token schema.
type TokenClient struct {
	config
}

// NewTokenClient returns a client for the Token from the given config.
func NewTokenClient(c config) *TokenClient {
	return &TokenClient{config: c}
}

// Use adds a list of mutation hooks to the hooks stack.
// A call to `Use(f, g, h)` equals to `token.Hooks(f(g(h())))`.
func (c *TokenClient) Use(hooks ...Hook) {
	c.hooks.Token = append(c.hooks.Token, hooks...)
}

// Create returns a builder for creating a Token entity.
func (c *TokenClient) Create() *TokenCreate {
	mutation := newTokenMutation(c.config, OpCreate)
	return &TokenCreate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// CreateBulk returns a builder for creating a bulk of Token entities.
func (c *TokenClient) CreateBulk(builders ...*TokenCreate) *TokenCreateBulk {
	return &TokenCreateBulk{config: c.config, builders: builders}
}

// Update returns an update builder for Token.
func (c *TokenClient) Update() *TokenUpdate {
	mutation := newTokenMutation(c.config, OpUpdate)
	return &TokenUpdate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOne returns an update builder for the given entity.
func (c *TokenClient) UpdateOne(t *Token) *TokenUpdateOne {
	mutation := newTokenMutation(c.config, OpUpdateOne, withToken(t))
	return &TokenUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOneID returns an update builder for the given id.
func (c *TokenClient) UpdateOneID(id uint32) *TokenUpdateOne {
	mutation := newTokenMutation(c.config, OpUpdateOne, withTokenID(id))
	return &TokenUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// Delete returns a delete builder for Token.
func (c *TokenClient) Delete() *TokenDelete {
	mutation := newTokenMutation(c.config, OpDelete)
	return &TokenDelete{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// DeleteOne returns a builder for deleting the given entity.
func (c *TokenClient) DeleteOne(t *Token) *TokenDeleteOne {
	return c.DeleteOneID(t.ID)
}

// DeleteOne returns a builder for deleting the given entity by its id.
func (c *TokenClient) DeleteOneID(id uint32) *TokenDeleteOne {
	builder := c.Delete().Where(token.ID(id))
	builder.mutation.id = &id
	builder.mutation.op = OpDeleteOne
	return &TokenDeleteOne{builder}
}

// Query returns a query builder for Token.
func (c *TokenClient) Query() *TokenQuery {
	return &TokenQuery{
		config: c.config,
	}
}

// Get returns a Token entity by its id.
func (c *TokenClient) Get(ctx context.Context, id uint32) (*Token, error) {
	return c.Query().Where(token.ID(id)).Only(ctx)
}

// GetX is like Get, but panics if an error occurs.
func (c *TokenClient) GetX(ctx context.Context, id uint32) *Token {
	obj, err := c.Get(ctx, id)
	if err != nil {
		panic(err)
	}
	return obj
}

// Hooks returns the client hooks.
func (c *TokenClient) Hooks() []Hook {
	hooks := c.hooks.Token
	return append(hooks[:len(hooks):len(hooks)], token.Hooks[:]...)
}

// TokenPairClient is a client for the TokenPair schema.
type TokenPairClient struct {
	config
}

// NewTokenPairClient returns a client for the TokenPair from the given config.
func NewTokenPairClient(c config) *TokenPairClient {
	return &TokenPairClient{config: c}
}

// Use adds a list of mutation hooks to the hooks stack.
// A call to `Use(f, g, h)` equals to `tokenpair.Hooks(f(g(h())))`.
func (c *TokenPairClient) Use(hooks ...Hook) {
	c.hooks.TokenPair = append(c.hooks.TokenPair, hooks...)
}

// Create returns a builder for creating a TokenPair entity.
func (c *TokenPairClient) Create() *TokenPairCreate {
	mutation := newTokenPairMutation(c.config, OpCreate)
	return &TokenPairCreate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// CreateBulk returns a builder for creating a bulk of TokenPair entities.
func (c *TokenPairClient) CreateBulk(builders ...*TokenPairCreate) *TokenPairCreateBulk {
	return &TokenPairCreateBulk{config: c.config, builders: builders}
}

// Update returns an update builder for TokenPair.
func (c *TokenPairClient) Update() *TokenPairUpdate {
	mutation := newTokenPairMutation(c.config, OpUpdate)
	return &TokenPairUpdate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOne returns an update builder for the given entity.
func (c *TokenPairClient) UpdateOne(tp *TokenPair) *TokenPairUpdateOne {
	mutation := newTokenPairMutation(c.config, OpUpdateOne, withTokenPair(tp))
	return &TokenPairUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOneID returns an update builder for the given id.
func (c *TokenPairClient) UpdateOneID(id uint32) *TokenPairUpdateOne {
	mutation := newTokenPairMutation(c.config, OpUpdateOne, withTokenPairID(id))
	return &TokenPairUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// Delete returns a delete builder for TokenPair.
func (c *TokenPairClient) Delete() *TokenPairDelete {
	mutation := newTokenPairMutation(c.config, OpDelete)
	return &TokenPairDelete{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// DeleteOne returns a builder for deleting the given entity.
func (c *TokenPairClient) DeleteOne(tp *TokenPair) *TokenPairDeleteOne {
	return c.DeleteOneID(tp.ID)
}

// DeleteOne returns a builder for deleting the given entity by its id.
func (c *TokenPairClient) DeleteOneID(id uint32) *TokenPairDeleteOne {
	builder := c.Delete().Where(tokenpair.ID(id))
	builder.mutation.id = &id
	builder.mutation.op = OpDeleteOne
	return &TokenPairDeleteOne{builder}
}

// Query returns a query builder for TokenPair.
func (c *TokenPairClient) Query() *TokenPairQuery {
	return &TokenPairQuery{
		config: c.config,
	}
}

// Get returns a TokenPair entity by its id.
func (c *TokenPairClient) Get(ctx context.Context, id uint32) (*TokenPair, error) {
	return c.Query().Where(tokenpair.ID(id)).Only(ctx)
}

// GetX is like Get, but panics if an error occurs.
func (c *TokenPairClient) GetX(ctx context.Context, id uint32) *TokenPair {
	obj, err := c.Get(ctx, id)
	if err != nil {
		panic(err)
	}
	return obj
}

// Hooks returns the client hooks.
func (c *TokenPairClient) Hooks() []Hook {
	hooks := c.hooks.TokenPair
	return append(hooks[:len(hooks):len(hooks)], tokenpair.Hooks[:]...)
}

// TransactionClient is a client for the Transaction schema.
type TransactionClient struct {
	config
}

// NewTransactionClient returns a client for the Transaction from the given config.
func NewTransactionClient(c config) *TransactionClient {
	return &TransactionClient{config: c}
}

// Use adds a list of mutation hooks to the hooks stack.
// A call to `Use(f, g, h)` equals to `transaction.Hooks(f(g(h())))`.
func (c *TransactionClient) Use(hooks ...Hook) {
	c.hooks.Transaction = append(c.hooks.Transaction, hooks...)
}

// Create returns a builder for creating a Transaction entity.
func (c *TransactionClient) Create() *TransactionCreate {
	mutation := newTransactionMutation(c.config, OpCreate)
	return &TransactionCreate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// CreateBulk returns a builder for creating a bulk of Transaction entities.
func (c *TransactionClient) CreateBulk(builders ...*TransactionCreate) *TransactionCreateBulk {
	return &TransactionCreateBulk{config: c.config, builders: builders}
}

// Update returns an update builder for Transaction.
func (c *TransactionClient) Update() *TransactionUpdate {
	mutation := newTransactionMutation(c.config, OpUpdate)
	return &TransactionUpdate{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOne returns an update builder for the given entity.
func (c *TransactionClient) UpdateOne(t *Transaction) *TransactionUpdateOne {
	mutation := newTransactionMutation(c.config, OpUpdateOne, withTransaction(t))
	return &TransactionUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// UpdateOneID returns an update builder for the given id.
func (c *TransactionClient) UpdateOneID(id uint32) *TransactionUpdateOne {
	mutation := newTransactionMutation(c.config, OpUpdateOne, withTransactionID(id))
	return &TransactionUpdateOne{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// Delete returns a delete builder for Transaction.
func (c *TransactionClient) Delete() *TransactionDelete {
	mutation := newTransactionMutation(c.config, OpDelete)
	return &TransactionDelete{config: c.config, hooks: c.Hooks(), mutation: mutation}
}

// DeleteOne returns a builder for deleting the given entity.
func (c *TransactionClient) DeleteOne(t *Transaction) *TransactionDeleteOne {
	return c.DeleteOneID(t.ID)
}

// DeleteOne returns a builder for deleting the given entity by its id.
func (c *TransactionClient) DeleteOneID(id uint32) *TransactionDeleteOne {
	builder := c.Delete().Where(transaction.ID(id))
	builder.mutation.id = &id
	builder.mutation.op = OpDeleteOne
	return &TransactionDeleteOne{builder}
}

// Query returns a query builder for Transaction.
func (c *TransactionClient) Query() *TransactionQuery {
	return &TransactionQuery{
		config: c.config,
	}
}

// Get returns a Transaction entity by its id.
func (c *TransactionClient) Get(ctx context.Context, id uint32) (*Transaction, error) {
	return c.Query().Where(transaction.ID(id)).Only(ctx)
}

// GetX is like Get, but panics if an error occurs.
func (c *TransactionClient) GetX(ctx context.Context, id uint32) *Transaction {
	obj, err := c.Get(ctx, id)
	if err != nil {
		panic(err)
	}
	return obj
}

// Hooks returns the client hooks.
func (c *TransactionClient) Hooks() []Hook {
	hooks := c.hooks.Transaction
	return append(hooks[:len(hooks):len(hooks)], transaction.Hooks[:]...)
}
