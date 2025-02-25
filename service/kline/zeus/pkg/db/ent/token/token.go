// Code generated by ent, DO NOT EDIT.

package token

import (
	"entgo.io/ent"
)

const (
	// Label holds the string label denoting the token type in the database.
	Label = "token"
	// FieldID holds the string denoting the id field in the database.
	FieldID = "id"
	// FieldCreatedAt holds the string denoting the created_at field in the database.
	FieldCreatedAt = "created_at"
	// FieldUpdatedAt holds the string denoting the updated_at field in the database.
	FieldUpdatedAt = "updated_at"
	// FieldDeletedAt holds the string denoting the deleted_at field in the database.
	FieldDeletedAt = "deleted_at"
	// FieldAddress holds the string denoting the address field in the database.
	FieldAddress = "address"
	// FieldSite holds the string denoting the site field in the database.
	FieldSite = "site"
	// FieldIconStoreType holds the string denoting the icon_store_type field in the database.
	FieldIconStoreType = "icon_store_type"
	// FieldIcon holds the string denoting the icon field in the database.
	FieldIcon = "icon"
	// FieldName holds the string denoting the name field in the database.
	FieldName = "name"
	// FieldSymbol holds the string denoting the symbol field in the database.
	FieldSymbol = "symbol"
	// Table holds the table name of the token in the database.
	Table = "tokens"
)

// Columns holds all SQL columns for token fields.
var Columns = []string{
	FieldID,
	FieldCreatedAt,
	FieldUpdatedAt,
	FieldDeletedAt,
	FieldAddress,
	FieldSite,
	FieldIconStoreType,
	FieldIcon,
	FieldName,
	FieldSymbol,
}

// ValidColumn reports if the column name is valid (part of the table columns).
func ValidColumn(column string) bool {
	for i := range Columns {
		if column == Columns[i] {
			return true
		}
	}
	return false
}

// Note that the variables below are initialized by the runtime
// package on the initialization of the application. Therefore,
// it should be imported in the main as follows:
//
//	import _ "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/runtime"
var (
	Hooks  [1]ent.Hook
	Policy ent.Policy
	// DefaultCreatedAt holds the default value on creation for the "created_at" field.
	DefaultCreatedAt func() uint32
	// DefaultUpdatedAt holds the default value on creation for the "updated_at" field.
	DefaultUpdatedAt func() uint32
	// UpdateDefaultUpdatedAt holds the default value on update for the "updated_at" field.
	UpdateDefaultUpdatedAt func() uint32
	// DefaultDeletedAt holds the default value on creation for the "deleted_at" field.
	DefaultDeletedAt func() uint32
	// DefaultIconStoreType holds the default value on creation for the "icon_store_type" field.
	DefaultIconStoreType string
)
