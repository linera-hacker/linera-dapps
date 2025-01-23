// Code generated by ent, DO NOT EDIT.

package ent

import (
	"context"
	"errors"
	"fmt"

	"entgo.io/ent/dialect/sql"
	"entgo.io/ent/dialect/sql/sqlgraph"
	"entgo.io/ent/schema/field"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/predicate"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/token"
)

// TokenUpdate is the builder for updating Token entities.
type TokenUpdate struct {
	config
	hooks     []Hook
	mutation  *TokenMutation
	modifiers []func(*sql.UpdateBuilder)
}

// Where appends a list predicates to the TokenUpdate builder.
func (tu *TokenUpdate) Where(ps ...predicate.Token) *TokenUpdate {
	tu.mutation.Where(ps...)
	return tu
}

// SetCreatedAt sets the "created_at" field.
func (tu *TokenUpdate) SetCreatedAt(u uint32) *TokenUpdate {
	tu.mutation.ResetCreatedAt()
	tu.mutation.SetCreatedAt(u)
	return tu
}

// SetNillableCreatedAt sets the "created_at" field if the given value is not nil.
func (tu *TokenUpdate) SetNillableCreatedAt(u *uint32) *TokenUpdate {
	if u != nil {
		tu.SetCreatedAt(*u)
	}
	return tu
}

// AddCreatedAt adds u to the "created_at" field.
func (tu *TokenUpdate) AddCreatedAt(u int32) *TokenUpdate {
	tu.mutation.AddCreatedAt(u)
	return tu
}

// SetUpdatedAt sets the "updated_at" field.
func (tu *TokenUpdate) SetUpdatedAt(u uint32) *TokenUpdate {
	tu.mutation.ResetUpdatedAt()
	tu.mutation.SetUpdatedAt(u)
	return tu
}

// AddUpdatedAt adds u to the "updated_at" field.
func (tu *TokenUpdate) AddUpdatedAt(u int32) *TokenUpdate {
	tu.mutation.AddUpdatedAt(u)
	return tu
}

// SetDeletedAt sets the "deleted_at" field.
func (tu *TokenUpdate) SetDeletedAt(u uint32) *TokenUpdate {
	tu.mutation.ResetDeletedAt()
	tu.mutation.SetDeletedAt(u)
	return tu
}

// SetNillableDeletedAt sets the "deleted_at" field if the given value is not nil.
func (tu *TokenUpdate) SetNillableDeletedAt(u *uint32) *TokenUpdate {
	if u != nil {
		tu.SetDeletedAt(*u)
	}
	return tu
}

// AddDeletedAt adds u to the "deleted_at" field.
func (tu *TokenUpdate) AddDeletedAt(u int32) *TokenUpdate {
	tu.mutation.AddDeletedAt(u)
	return tu
}

// SetAddress sets the "address" field.
func (tu *TokenUpdate) SetAddress(s string) *TokenUpdate {
	tu.mutation.SetAddress(s)
	return tu
}

// SetSite sets the "site" field.
func (tu *TokenUpdate) SetSite(s string) *TokenUpdate {
	tu.mutation.SetSite(s)
	return tu
}

// SetIconStoreType sets the "icon_store_type" field.
func (tu *TokenUpdate) SetIconStoreType(s string) *TokenUpdate {
	tu.mutation.SetIconStoreType(s)
	return tu
}

// SetNillableIconStoreType sets the "icon_store_type" field if the given value is not nil.
func (tu *TokenUpdate) SetNillableIconStoreType(s *string) *TokenUpdate {
	if s != nil {
		tu.SetIconStoreType(*s)
	}
	return tu
}

// SetIcon sets the "icon" field.
func (tu *TokenUpdate) SetIcon(s string) *TokenUpdate {
	tu.mutation.SetIcon(s)
	return tu
}

// SetName sets the "name" field.
func (tu *TokenUpdate) SetName(s string) *TokenUpdate {
	tu.mutation.SetName(s)
	return tu
}

// SetSymbol sets the "symbol" field.
func (tu *TokenUpdate) SetSymbol(s string) *TokenUpdate {
	tu.mutation.SetSymbol(s)
	return tu
}

// Mutation returns the TokenMutation object of the builder.
func (tu *TokenUpdate) Mutation() *TokenMutation {
	return tu.mutation
}

// Save executes the query and returns the number of nodes affected by the update operation.
func (tu *TokenUpdate) Save(ctx context.Context) (int, error) {
	var (
		err      error
		affected int
	)
	if err := tu.defaults(); err != nil {
		return 0, err
	}
	if len(tu.hooks) == 0 {
		affected, err = tu.sqlSave(ctx)
	} else {
		var mut Mutator = MutateFunc(func(ctx context.Context, m Mutation) (Value, error) {
			mutation, ok := m.(*TokenMutation)
			if !ok {
				return nil, fmt.Errorf("unexpected mutation type %T", m)
			}
			tu.mutation = mutation
			affected, err = tu.sqlSave(ctx)
			mutation.done = true
			return affected, err
		})
		for i := len(tu.hooks) - 1; i >= 0; i-- {
			if tu.hooks[i] == nil {
				return 0, fmt.Errorf("ent: uninitialized hook (forgotten import ent/runtime?)")
			}
			mut = tu.hooks[i](mut)
		}
		if _, err := mut.Mutate(ctx, tu.mutation); err != nil {
			return 0, err
		}
	}
	return affected, err
}

// SaveX is like Save, but panics if an error occurs.
func (tu *TokenUpdate) SaveX(ctx context.Context) int {
	affected, err := tu.Save(ctx)
	if err != nil {
		panic(err)
	}
	return affected
}

// Exec executes the query.
func (tu *TokenUpdate) Exec(ctx context.Context) error {
	_, err := tu.Save(ctx)
	return err
}

// ExecX is like Exec, but panics if an error occurs.
func (tu *TokenUpdate) ExecX(ctx context.Context) {
	if err := tu.Exec(ctx); err != nil {
		panic(err)
	}
}

// defaults sets the default values of the builder before save.
func (tu *TokenUpdate) defaults() error {
	if _, ok := tu.mutation.UpdatedAt(); !ok {
		if token.UpdateDefaultUpdatedAt == nil {
			return fmt.Errorf("ent: uninitialized token.UpdateDefaultUpdatedAt (forgotten import ent/runtime?)")
		}
		v := token.UpdateDefaultUpdatedAt()
		tu.mutation.SetUpdatedAt(v)
	}
	return nil
}

// Modify adds a statement modifier for attaching custom logic to the UPDATE statement.
func (tu *TokenUpdate) Modify(modifiers ...func(u *sql.UpdateBuilder)) *TokenUpdate {
	tu.modifiers = append(tu.modifiers, modifiers...)
	return tu
}

func (tu *TokenUpdate) sqlSave(ctx context.Context) (n int, err error) {
	_spec := &sqlgraph.UpdateSpec{
		Node: &sqlgraph.NodeSpec{
			Table:   token.Table,
			Columns: token.Columns,
			ID: &sqlgraph.FieldSpec{
				Type:   field.TypeUint32,
				Column: token.FieldID,
			},
		},
	}
	if ps := tu.mutation.predicates; len(ps) > 0 {
		_spec.Predicate = func(selector *sql.Selector) {
			for i := range ps {
				ps[i](selector)
			}
		}
	}
	if value, ok := tu.mutation.CreatedAt(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldCreatedAt,
		})
	}
	if value, ok := tu.mutation.AddedCreatedAt(); ok {
		_spec.Fields.Add = append(_spec.Fields.Add, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldCreatedAt,
		})
	}
	if value, ok := tu.mutation.UpdatedAt(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldUpdatedAt,
		})
	}
	if value, ok := tu.mutation.AddedUpdatedAt(); ok {
		_spec.Fields.Add = append(_spec.Fields.Add, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldUpdatedAt,
		})
	}
	if value, ok := tu.mutation.DeletedAt(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldDeletedAt,
		})
	}
	if value, ok := tu.mutation.AddedDeletedAt(); ok {
		_spec.Fields.Add = append(_spec.Fields.Add, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldDeletedAt,
		})
	}
	if value, ok := tu.mutation.Address(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldAddress,
		})
	}
	if value, ok := tu.mutation.Site(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldSite,
		})
	}
	if value, ok := tu.mutation.IconStoreType(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldIconStoreType,
		})
	}
	if value, ok := tu.mutation.Icon(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldIcon,
		})
	}
	if value, ok := tu.mutation.Name(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldName,
		})
	}
	if value, ok := tu.mutation.Symbol(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldSymbol,
		})
	}
	_spec.Modifiers = tu.modifiers
	if n, err = sqlgraph.UpdateNodes(ctx, tu.driver, _spec); err != nil {
		if _, ok := err.(*sqlgraph.NotFoundError); ok {
			err = &NotFoundError{token.Label}
		} else if sqlgraph.IsConstraintError(err) {
			err = &ConstraintError{msg: err.Error(), wrap: err}
		}
		return 0, err
	}
	return n, nil
}

// TokenUpdateOne is the builder for updating a single Token entity.
type TokenUpdateOne struct {
	config
	fields    []string
	hooks     []Hook
	mutation  *TokenMutation
	modifiers []func(*sql.UpdateBuilder)
}

// SetCreatedAt sets the "created_at" field.
func (tuo *TokenUpdateOne) SetCreatedAt(u uint32) *TokenUpdateOne {
	tuo.mutation.ResetCreatedAt()
	tuo.mutation.SetCreatedAt(u)
	return tuo
}

// SetNillableCreatedAt sets the "created_at" field if the given value is not nil.
func (tuo *TokenUpdateOne) SetNillableCreatedAt(u *uint32) *TokenUpdateOne {
	if u != nil {
		tuo.SetCreatedAt(*u)
	}
	return tuo
}

// AddCreatedAt adds u to the "created_at" field.
func (tuo *TokenUpdateOne) AddCreatedAt(u int32) *TokenUpdateOne {
	tuo.mutation.AddCreatedAt(u)
	return tuo
}

// SetUpdatedAt sets the "updated_at" field.
func (tuo *TokenUpdateOne) SetUpdatedAt(u uint32) *TokenUpdateOne {
	tuo.mutation.ResetUpdatedAt()
	tuo.mutation.SetUpdatedAt(u)
	return tuo
}

// AddUpdatedAt adds u to the "updated_at" field.
func (tuo *TokenUpdateOne) AddUpdatedAt(u int32) *TokenUpdateOne {
	tuo.mutation.AddUpdatedAt(u)
	return tuo
}

// SetDeletedAt sets the "deleted_at" field.
func (tuo *TokenUpdateOne) SetDeletedAt(u uint32) *TokenUpdateOne {
	tuo.mutation.ResetDeletedAt()
	tuo.mutation.SetDeletedAt(u)
	return tuo
}

// SetNillableDeletedAt sets the "deleted_at" field if the given value is not nil.
func (tuo *TokenUpdateOne) SetNillableDeletedAt(u *uint32) *TokenUpdateOne {
	if u != nil {
		tuo.SetDeletedAt(*u)
	}
	return tuo
}

// AddDeletedAt adds u to the "deleted_at" field.
func (tuo *TokenUpdateOne) AddDeletedAt(u int32) *TokenUpdateOne {
	tuo.mutation.AddDeletedAt(u)
	return tuo
}

// SetAddress sets the "address" field.
func (tuo *TokenUpdateOne) SetAddress(s string) *TokenUpdateOne {
	tuo.mutation.SetAddress(s)
	return tuo
}

// SetSite sets the "site" field.
func (tuo *TokenUpdateOne) SetSite(s string) *TokenUpdateOne {
	tuo.mutation.SetSite(s)
	return tuo
}

// SetIconStoreType sets the "icon_store_type" field.
func (tuo *TokenUpdateOne) SetIconStoreType(s string) *TokenUpdateOne {
	tuo.mutation.SetIconStoreType(s)
	return tuo
}

// SetNillableIconStoreType sets the "icon_store_type" field if the given value is not nil.
func (tuo *TokenUpdateOne) SetNillableIconStoreType(s *string) *TokenUpdateOne {
	if s != nil {
		tuo.SetIconStoreType(*s)
	}
	return tuo
}

// SetIcon sets the "icon" field.
func (tuo *TokenUpdateOne) SetIcon(s string) *TokenUpdateOne {
	tuo.mutation.SetIcon(s)
	return tuo
}

// SetName sets the "name" field.
func (tuo *TokenUpdateOne) SetName(s string) *TokenUpdateOne {
	tuo.mutation.SetName(s)
	return tuo
}

// SetSymbol sets the "symbol" field.
func (tuo *TokenUpdateOne) SetSymbol(s string) *TokenUpdateOne {
	tuo.mutation.SetSymbol(s)
	return tuo
}

// Mutation returns the TokenMutation object of the builder.
func (tuo *TokenUpdateOne) Mutation() *TokenMutation {
	return tuo.mutation
}

// Select allows selecting one or more fields (columns) of the returned entity.
// The default is selecting all fields defined in the entity schema.
func (tuo *TokenUpdateOne) Select(field string, fields ...string) *TokenUpdateOne {
	tuo.fields = append([]string{field}, fields...)
	return tuo
}

// Save executes the query and returns the updated Token entity.
func (tuo *TokenUpdateOne) Save(ctx context.Context) (*Token, error) {
	var (
		err  error
		node *Token
	)
	if err := tuo.defaults(); err != nil {
		return nil, err
	}
	if len(tuo.hooks) == 0 {
		node, err = tuo.sqlSave(ctx)
	} else {
		var mut Mutator = MutateFunc(func(ctx context.Context, m Mutation) (Value, error) {
			mutation, ok := m.(*TokenMutation)
			if !ok {
				return nil, fmt.Errorf("unexpected mutation type %T", m)
			}
			tuo.mutation = mutation
			node, err = tuo.sqlSave(ctx)
			mutation.done = true
			return node, err
		})
		for i := len(tuo.hooks) - 1; i >= 0; i-- {
			if tuo.hooks[i] == nil {
				return nil, fmt.Errorf("ent: uninitialized hook (forgotten import ent/runtime?)")
			}
			mut = tuo.hooks[i](mut)
		}
		v, err := mut.Mutate(ctx, tuo.mutation)
		if err != nil {
			return nil, err
		}
		nv, ok := v.(*Token)
		if !ok {
			return nil, fmt.Errorf("unexpected node type %T returned from TokenMutation", v)
		}
		node = nv
	}
	return node, err
}

// SaveX is like Save, but panics if an error occurs.
func (tuo *TokenUpdateOne) SaveX(ctx context.Context) *Token {
	node, err := tuo.Save(ctx)
	if err != nil {
		panic(err)
	}
	return node
}

// Exec executes the query on the entity.
func (tuo *TokenUpdateOne) Exec(ctx context.Context) error {
	_, err := tuo.Save(ctx)
	return err
}

// ExecX is like Exec, but panics if an error occurs.
func (tuo *TokenUpdateOne) ExecX(ctx context.Context) {
	if err := tuo.Exec(ctx); err != nil {
		panic(err)
	}
}

// defaults sets the default values of the builder before save.
func (tuo *TokenUpdateOne) defaults() error {
	if _, ok := tuo.mutation.UpdatedAt(); !ok {
		if token.UpdateDefaultUpdatedAt == nil {
			return fmt.Errorf("ent: uninitialized token.UpdateDefaultUpdatedAt (forgotten import ent/runtime?)")
		}
		v := token.UpdateDefaultUpdatedAt()
		tuo.mutation.SetUpdatedAt(v)
	}
	return nil
}

// Modify adds a statement modifier for attaching custom logic to the UPDATE statement.
func (tuo *TokenUpdateOne) Modify(modifiers ...func(u *sql.UpdateBuilder)) *TokenUpdateOne {
	tuo.modifiers = append(tuo.modifiers, modifiers...)
	return tuo
}

func (tuo *TokenUpdateOne) sqlSave(ctx context.Context) (_node *Token, err error) {
	_spec := &sqlgraph.UpdateSpec{
		Node: &sqlgraph.NodeSpec{
			Table:   token.Table,
			Columns: token.Columns,
			ID: &sqlgraph.FieldSpec{
				Type:   field.TypeUint32,
				Column: token.FieldID,
			},
		},
	}
	id, ok := tuo.mutation.ID()
	if !ok {
		return nil, &ValidationError{Name: "id", err: errors.New(`ent: missing "Token.id" for update`)}
	}
	_spec.Node.ID.Value = id
	if fields := tuo.fields; len(fields) > 0 {
		_spec.Node.Columns = make([]string, 0, len(fields))
		_spec.Node.Columns = append(_spec.Node.Columns, token.FieldID)
		for _, f := range fields {
			if !token.ValidColumn(f) {
				return nil, &ValidationError{Name: f, err: fmt.Errorf("ent: invalid field %q for query", f)}
			}
			if f != token.FieldID {
				_spec.Node.Columns = append(_spec.Node.Columns, f)
			}
		}
	}
	if ps := tuo.mutation.predicates; len(ps) > 0 {
		_spec.Predicate = func(selector *sql.Selector) {
			for i := range ps {
				ps[i](selector)
			}
		}
	}
	if value, ok := tuo.mutation.CreatedAt(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldCreatedAt,
		})
	}
	if value, ok := tuo.mutation.AddedCreatedAt(); ok {
		_spec.Fields.Add = append(_spec.Fields.Add, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldCreatedAt,
		})
	}
	if value, ok := tuo.mutation.UpdatedAt(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldUpdatedAt,
		})
	}
	if value, ok := tuo.mutation.AddedUpdatedAt(); ok {
		_spec.Fields.Add = append(_spec.Fields.Add, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldUpdatedAt,
		})
	}
	if value, ok := tuo.mutation.DeletedAt(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldDeletedAt,
		})
	}
	if value, ok := tuo.mutation.AddedDeletedAt(); ok {
		_spec.Fields.Add = append(_spec.Fields.Add, &sqlgraph.FieldSpec{
			Type:   field.TypeUint32,
			Value:  value,
			Column: token.FieldDeletedAt,
		})
	}
	if value, ok := tuo.mutation.Address(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldAddress,
		})
	}
	if value, ok := tuo.mutation.Site(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldSite,
		})
	}
	if value, ok := tuo.mutation.IconStoreType(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldIconStoreType,
		})
	}
	if value, ok := tuo.mutation.Icon(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldIcon,
		})
	}
	if value, ok := tuo.mutation.Name(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldName,
		})
	}
	if value, ok := tuo.mutation.Symbol(); ok {
		_spec.Fields.Set = append(_spec.Fields.Set, &sqlgraph.FieldSpec{
			Type:   field.TypeString,
			Value:  value,
			Column: token.FieldSymbol,
		})
	}
	_spec.Modifiers = tuo.modifiers
	_node = &Token{config: tuo.config}
	_spec.Assign = _node.assignValues
	_spec.ScanValues = _node.scanValues
	if err = sqlgraph.UpdateNode(ctx, tuo.driver, _spec); err != nil {
		if _, ok := err.(*sqlgraph.NotFoundError); ok {
			err = &NotFoundError{token.Label}
		} else if sqlgraph.IsConstraintError(err) {
			err = &ConstraintError{msg: err.Error(), wrap: err}
		}
		return nil, err
	}
	return _node, nil
}
