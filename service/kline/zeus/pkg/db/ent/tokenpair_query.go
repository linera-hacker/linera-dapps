// Code generated by ent, DO NOT EDIT.

package ent

import (
	"context"
	"errors"
	"fmt"
	"math"

	"entgo.io/ent/dialect"
	"entgo.io/ent/dialect/sql"
	"entgo.io/ent/dialect/sql/sqlgraph"
	"entgo.io/ent/schema/field"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/predicate"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/tokenpair"
)

// TokenPairQuery is the builder for querying TokenPair entities.
type TokenPairQuery struct {
	config
	limit      *int
	offset     *int
	unique     *bool
	order      []OrderFunc
	fields     []string
	predicates []predicate.TokenPair
	modifiers  []func(*sql.Selector)
	// intermediate query (i.e. traversal path).
	sql  *sql.Selector
	path func(context.Context) (*sql.Selector, error)
}

// Where adds a new predicate for the TokenPairQuery builder.
func (tpq *TokenPairQuery) Where(ps ...predicate.TokenPair) *TokenPairQuery {
	tpq.predicates = append(tpq.predicates, ps...)
	return tpq
}

// Limit adds a limit step to the query.
func (tpq *TokenPairQuery) Limit(limit int) *TokenPairQuery {
	tpq.limit = &limit
	return tpq
}

// Offset adds an offset step to the query.
func (tpq *TokenPairQuery) Offset(offset int) *TokenPairQuery {
	tpq.offset = &offset
	return tpq
}

// Unique configures the query builder to filter duplicate records on query.
// By default, unique is set to true, and can be disabled using this method.
func (tpq *TokenPairQuery) Unique(unique bool) *TokenPairQuery {
	tpq.unique = &unique
	return tpq
}

// Order adds an order step to the query.
func (tpq *TokenPairQuery) Order(o ...OrderFunc) *TokenPairQuery {
	tpq.order = append(tpq.order, o...)
	return tpq
}

// First returns the first TokenPair entity from the query.
// Returns a *NotFoundError when no TokenPair was found.
func (tpq *TokenPairQuery) First(ctx context.Context) (*TokenPair, error) {
	nodes, err := tpq.Limit(1).All(ctx)
	if err != nil {
		return nil, err
	}
	if len(nodes) == 0 {
		return nil, &NotFoundError{tokenpair.Label}
	}
	return nodes[0], nil
}

// FirstX is like First, but panics if an error occurs.
func (tpq *TokenPairQuery) FirstX(ctx context.Context) *TokenPair {
	node, err := tpq.First(ctx)
	if err != nil && !IsNotFound(err) {
		panic(err)
	}
	return node
}

// FirstID returns the first TokenPair ID from the query.
// Returns a *NotFoundError when no TokenPair ID was found.
func (tpq *TokenPairQuery) FirstID(ctx context.Context) (id uint32, err error) {
	var ids []uint32
	if ids, err = tpq.Limit(1).IDs(ctx); err != nil {
		return
	}
	if len(ids) == 0 {
		err = &NotFoundError{tokenpair.Label}
		return
	}
	return ids[0], nil
}

// FirstIDX is like FirstID, but panics if an error occurs.
func (tpq *TokenPairQuery) FirstIDX(ctx context.Context) uint32 {
	id, err := tpq.FirstID(ctx)
	if err != nil && !IsNotFound(err) {
		panic(err)
	}
	return id
}

// Only returns a single TokenPair entity found by the query, ensuring it only returns one.
// Returns a *NotSingularError when more than one TokenPair entity is found.
// Returns a *NotFoundError when no TokenPair entities are found.
func (tpq *TokenPairQuery) Only(ctx context.Context) (*TokenPair, error) {
	nodes, err := tpq.Limit(2).All(ctx)
	if err != nil {
		return nil, err
	}
	switch len(nodes) {
	case 1:
		return nodes[0], nil
	case 0:
		return nil, &NotFoundError{tokenpair.Label}
	default:
		return nil, &NotSingularError{tokenpair.Label}
	}
}

// OnlyX is like Only, but panics if an error occurs.
func (tpq *TokenPairQuery) OnlyX(ctx context.Context) *TokenPair {
	node, err := tpq.Only(ctx)
	if err != nil {
		panic(err)
	}
	return node
}

// OnlyID is like Only, but returns the only TokenPair ID in the query.
// Returns a *NotSingularError when more than one TokenPair ID is found.
// Returns a *NotFoundError when no entities are found.
func (tpq *TokenPairQuery) OnlyID(ctx context.Context) (id uint32, err error) {
	var ids []uint32
	if ids, err = tpq.Limit(2).IDs(ctx); err != nil {
		return
	}
	switch len(ids) {
	case 1:
		id = ids[0]
	case 0:
		err = &NotFoundError{tokenpair.Label}
	default:
		err = &NotSingularError{tokenpair.Label}
	}
	return
}

// OnlyIDX is like OnlyID, but panics if an error occurs.
func (tpq *TokenPairQuery) OnlyIDX(ctx context.Context) uint32 {
	id, err := tpq.OnlyID(ctx)
	if err != nil {
		panic(err)
	}
	return id
}

// All executes the query and returns a list of TokenPairs.
func (tpq *TokenPairQuery) All(ctx context.Context) ([]*TokenPair, error) {
	if err := tpq.prepareQuery(ctx); err != nil {
		return nil, err
	}
	return tpq.sqlAll(ctx)
}

// AllX is like All, but panics if an error occurs.
func (tpq *TokenPairQuery) AllX(ctx context.Context) []*TokenPair {
	nodes, err := tpq.All(ctx)
	if err != nil {
		panic(err)
	}
	return nodes
}

// IDs executes the query and returns a list of TokenPair IDs.
func (tpq *TokenPairQuery) IDs(ctx context.Context) ([]uint32, error) {
	var ids []uint32
	if err := tpq.Select(tokenpair.FieldID).Scan(ctx, &ids); err != nil {
		return nil, err
	}
	return ids, nil
}

// IDsX is like IDs, but panics if an error occurs.
func (tpq *TokenPairQuery) IDsX(ctx context.Context) []uint32 {
	ids, err := tpq.IDs(ctx)
	if err != nil {
		panic(err)
	}
	return ids
}

// Count returns the count of the given query.
func (tpq *TokenPairQuery) Count(ctx context.Context) (int, error) {
	if err := tpq.prepareQuery(ctx); err != nil {
		return 0, err
	}
	return tpq.sqlCount(ctx)
}

// CountX is like Count, but panics if an error occurs.
func (tpq *TokenPairQuery) CountX(ctx context.Context) int {
	count, err := tpq.Count(ctx)
	if err != nil {
		panic(err)
	}
	return count
}

// Exist returns true if the query has elements in the graph.
func (tpq *TokenPairQuery) Exist(ctx context.Context) (bool, error) {
	if err := tpq.prepareQuery(ctx); err != nil {
		return false, err
	}
	return tpq.sqlExist(ctx)
}

// ExistX is like Exist, but panics if an error occurs.
func (tpq *TokenPairQuery) ExistX(ctx context.Context) bool {
	exist, err := tpq.Exist(ctx)
	if err != nil {
		panic(err)
	}
	return exist
}

// Clone returns a duplicate of the TokenPairQuery builder, including all associated steps. It can be
// used to prepare common query builders and use them differently after the clone is made.
func (tpq *TokenPairQuery) Clone() *TokenPairQuery {
	if tpq == nil {
		return nil
	}
	return &TokenPairQuery{
		config:     tpq.config,
		limit:      tpq.limit,
		offset:     tpq.offset,
		order:      append([]OrderFunc{}, tpq.order...),
		predicates: append([]predicate.TokenPair{}, tpq.predicates...),
		// clone intermediate query.
		sql:    tpq.sql.Clone(),
		path:   tpq.path,
		unique: tpq.unique,
	}
}

// GroupBy is used to group vertices by one or more fields/columns.
// It is often used with aggregate functions, like: count, max, mean, min, sum.
//
// Example:
//
//	var v []struct {
//		CreatedAt uint32 `json:"created_at,omitempty"`
//		Count int `json:"count,omitempty"`
//	}
//
//	client.TokenPair.Query().
//		GroupBy(tokenpair.FieldCreatedAt).
//		Aggregate(ent.Count()).
//		Scan(ctx, &v)
func (tpq *TokenPairQuery) GroupBy(field string, fields ...string) *TokenPairGroupBy {
	grbuild := &TokenPairGroupBy{config: tpq.config}
	grbuild.fields = append([]string{field}, fields...)
	grbuild.path = func(ctx context.Context) (prev *sql.Selector, err error) {
		if err := tpq.prepareQuery(ctx); err != nil {
			return nil, err
		}
		return tpq.sqlQuery(ctx), nil
	}
	grbuild.label = tokenpair.Label
	grbuild.flds, grbuild.scan = &grbuild.fields, grbuild.Scan
	return grbuild
}

// Select allows the selection one or more fields/columns for the given query,
// instead of selecting all fields in the entity.
//
// Example:
//
//	var v []struct {
//		CreatedAt uint32 `json:"created_at,omitempty"`
//	}
//
//	client.TokenPair.Query().
//		Select(tokenpair.FieldCreatedAt).
//		Scan(ctx, &v)
func (tpq *TokenPairQuery) Select(fields ...string) *TokenPairSelect {
	tpq.fields = append(tpq.fields, fields...)
	selbuild := &TokenPairSelect{TokenPairQuery: tpq}
	selbuild.label = tokenpair.Label
	selbuild.flds, selbuild.scan = &tpq.fields, selbuild.Scan
	return selbuild
}

func (tpq *TokenPairQuery) prepareQuery(ctx context.Context) error {
	for _, f := range tpq.fields {
		if !tokenpair.ValidColumn(f) {
			return &ValidationError{Name: f, err: fmt.Errorf("ent: invalid field %q for query", f)}
		}
	}
	if tpq.path != nil {
		prev, err := tpq.path(ctx)
		if err != nil {
			return err
		}
		tpq.sql = prev
	}
	if tokenpair.Policy == nil {
		return errors.New("ent: uninitialized tokenpair.Policy (forgotten import ent/runtime?)")
	}
	if err := tokenpair.Policy.EvalQuery(ctx, tpq); err != nil {
		return err
	}
	return nil
}

func (tpq *TokenPairQuery) sqlAll(ctx context.Context, hooks ...queryHook) ([]*TokenPair, error) {
	var (
		nodes = []*TokenPair{}
		_spec = tpq.querySpec()
	)
	_spec.ScanValues = func(columns []string) ([]interface{}, error) {
		return (*TokenPair).scanValues(nil, columns)
	}
	_spec.Assign = func(columns []string, values []interface{}) error {
		node := &TokenPair{config: tpq.config}
		nodes = append(nodes, node)
		return node.assignValues(columns, values)
	}
	if len(tpq.modifiers) > 0 {
		_spec.Modifiers = tpq.modifiers
	}
	for i := range hooks {
		hooks[i](ctx, _spec)
	}
	if err := sqlgraph.QueryNodes(ctx, tpq.driver, _spec); err != nil {
		return nil, err
	}
	if len(nodes) == 0 {
		return nodes, nil
	}
	return nodes, nil
}

func (tpq *TokenPairQuery) sqlCount(ctx context.Context) (int, error) {
	_spec := tpq.querySpec()
	if len(tpq.modifiers) > 0 {
		_spec.Modifiers = tpq.modifiers
	}
	_spec.Node.Columns = tpq.fields
	if len(tpq.fields) > 0 {
		_spec.Unique = tpq.unique != nil && *tpq.unique
	}
	return sqlgraph.CountNodes(ctx, tpq.driver, _spec)
}

func (tpq *TokenPairQuery) sqlExist(ctx context.Context) (bool, error) {
	n, err := tpq.sqlCount(ctx)
	if err != nil {
		return false, fmt.Errorf("ent: check existence: %w", err)
	}
	return n > 0, nil
}

func (tpq *TokenPairQuery) querySpec() *sqlgraph.QuerySpec {
	_spec := &sqlgraph.QuerySpec{
		Node: &sqlgraph.NodeSpec{
			Table:   tokenpair.Table,
			Columns: tokenpair.Columns,
			ID: &sqlgraph.FieldSpec{
				Type:   field.TypeUint32,
				Column: tokenpair.FieldID,
			},
		},
		From:   tpq.sql,
		Unique: true,
	}
	if unique := tpq.unique; unique != nil {
		_spec.Unique = *unique
	}
	if fields := tpq.fields; len(fields) > 0 {
		_spec.Node.Columns = make([]string, 0, len(fields))
		_spec.Node.Columns = append(_spec.Node.Columns, tokenpair.FieldID)
		for i := range fields {
			if fields[i] != tokenpair.FieldID {
				_spec.Node.Columns = append(_spec.Node.Columns, fields[i])
			}
		}
	}
	if ps := tpq.predicates; len(ps) > 0 {
		_spec.Predicate = func(selector *sql.Selector) {
			for i := range ps {
				ps[i](selector)
			}
		}
	}
	if limit := tpq.limit; limit != nil {
		_spec.Limit = *limit
	}
	if offset := tpq.offset; offset != nil {
		_spec.Offset = *offset
	}
	if ps := tpq.order; len(ps) > 0 {
		_spec.Order = func(selector *sql.Selector) {
			for i := range ps {
				ps[i](selector)
			}
		}
	}
	return _spec
}

func (tpq *TokenPairQuery) sqlQuery(ctx context.Context) *sql.Selector {
	builder := sql.Dialect(tpq.driver.Dialect())
	t1 := builder.Table(tokenpair.Table)
	columns := tpq.fields
	if len(columns) == 0 {
		columns = tokenpair.Columns
	}
	selector := builder.Select(t1.Columns(columns...)...).From(t1)
	if tpq.sql != nil {
		selector = tpq.sql
		selector.Select(selector.Columns(columns...)...)
	}
	if tpq.unique != nil && *tpq.unique {
		selector.Distinct()
	}
	for _, m := range tpq.modifiers {
		m(selector)
	}
	for _, p := range tpq.predicates {
		p(selector)
	}
	for _, p := range tpq.order {
		p(selector)
	}
	if offset := tpq.offset; offset != nil {
		// limit is mandatory for offset clause. We start
		// with default value, and override it below if needed.
		selector.Offset(*offset).Limit(math.MaxInt32)
	}
	if limit := tpq.limit; limit != nil {
		selector.Limit(*limit)
	}
	return selector
}

// ForUpdate locks the selected rows against concurrent updates, and prevent them from being
// updated, deleted or "selected ... for update" by other sessions, until the transaction is
// either committed or rolled-back.
func (tpq *TokenPairQuery) ForUpdate(opts ...sql.LockOption) *TokenPairQuery {
	if tpq.driver.Dialect() == dialect.Postgres {
		tpq.Unique(false)
	}
	tpq.modifiers = append(tpq.modifiers, func(s *sql.Selector) {
		s.ForUpdate(opts...)
	})
	return tpq
}

// ForShare behaves similarly to ForUpdate, except that it acquires a shared mode lock
// on any rows that are read. Other sessions can read the rows, but cannot modify them
// until your transaction commits.
func (tpq *TokenPairQuery) ForShare(opts ...sql.LockOption) *TokenPairQuery {
	if tpq.driver.Dialect() == dialect.Postgres {
		tpq.Unique(false)
	}
	tpq.modifiers = append(tpq.modifiers, func(s *sql.Selector) {
		s.ForShare(opts...)
	})
	return tpq
}

// Modify adds a query modifier for attaching custom logic to queries.
func (tpq *TokenPairQuery) Modify(modifiers ...func(s *sql.Selector)) *TokenPairSelect {
	tpq.modifiers = append(tpq.modifiers, modifiers...)
	return tpq.Select()
}

// TokenPairGroupBy is the group-by builder for TokenPair entities.
type TokenPairGroupBy struct {
	config
	selector
	fields []string
	fns    []AggregateFunc
	// intermediate query (i.e. traversal path).
	sql  *sql.Selector
	path func(context.Context) (*sql.Selector, error)
}

// Aggregate adds the given aggregation functions to the group-by query.
func (tpgb *TokenPairGroupBy) Aggregate(fns ...AggregateFunc) *TokenPairGroupBy {
	tpgb.fns = append(tpgb.fns, fns...)
	return tpgb
}

// Scan applies the group-by query and scans the result into the given value.
func (tpgb *TokenPairGroupBy) Scan(ctx context.Context, v interface{}) error {
	query, err := tpgb.path(ctx)
	if err != nil {
		return err
	}
	tpgb.sql = query
	return tpgb.sqlScan(ctx, v)
}

func (tpgb *TokenPairGroupBy) sqlScan(ctx context.Context, v interface{}) error {
	for _, f := range tpgb.fields {
		if !tokenpair.ValidColumn(f) {
			return &ValidationError{Name: f, err: fmt.Errorf("invalid field %q for group-by", f)}
		}
	}
	selector := tpgb.sqlQuery()
	if err := selector.Err(); err != nil {
		return err
	}
	rows := &sql.Rows{}
	query, args := selector.Query()
	if err := tpgb.driver.Query(ctx, query, args, rows); err != nil {
		return err
	}
	defer rows.Close()
	return sql.ScanSlice(rows, v)
}

func (tpgb *TokenPairGroupBy) sqlQuery() *sql.Selector {
	selector := tpgb.sql.Select()
	aggregation := make([]string, 0, len(tpgb.fns))
	for _, fn := range tpgb.fns {
		aggregation = append(aggregation, fn(selector))
	}
	// If no columns were selected in a custom aggregation function, the default
	// selection is the fields used for "group-by", and the aggregation functions.
	if len(selector.SelectedColumns()) == 0 {
		columns := make([]string, 0, len(tpgb.fields)+len(tpgb.fns))
		for _, f := range tpgb.fields {
			columns = append(columns, selector.C(f))
		}
		columns = append(columns, aggregation...)
		selector.Select(columns...)
	}
	return selector.GroupBy(selector.Columns(tpgb.fields...)...)
}

// TokenPairSelect is the builder for selecting fields of TokenPair entities.
type TokenPairSelect struct {
	*TokenPairQuery
	selector
	// intermediate query (i.e. traversal path).
	sql *sql.Selector
}

// Scan applies the selector query and scans the result into the given value.
func (tps *TokenPairSelect) Scan(ctx context.Context, v interface{}) error {
	if err := tps.prepareQuery(ctx); err != nil {
		return err
	}
	tps.sql = tps.TokenPairQuery.sqlQuery(ctx)
	return tps.sqlScan(ctx, v)
}

func (tps *TokenPairSelect) sqlScan(ctx context.Context, v interface{}) error {
	rows := &sql.Rows{}
	query, args := tps.sql.Query()
	if err := tps.driver.Query(ctx, query, args, rows); err != nil {
		return err
	}
	defer rows.Close()
	return sql.ScanSlice(rows, v)
}

// Modify adds a query modifier for attaching custom logic to queries.
func (tps *TokenPairSelect) Modify(modifiers ...func(s *sql.Selector)) *TokenPairSelect {
	tps.modifiers = append(tps.modifiers, modifiers...)
	return tps
}
