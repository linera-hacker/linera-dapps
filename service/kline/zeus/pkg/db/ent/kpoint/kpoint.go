// Code generated by ent, DO NOT EDIT.

package kpoint

import (
	"entgo.io/ent"
)

const (
	// Label holds the string label denoting the kpoint type in the database.
	Label = "kpoint"
	// FieldID holds the string denoting the id field in the database.
	FieldID = "id"
	// FieldCreatedAt holds the string denoting the created_at field in the database.
	FieldCreatedAt = "created_at"
	// FieldUpdatedAt holds the string denoting the updated_at field in the database.
	FieldUpdatedAt = "updated_at"
	// FieldDeletedAt holds the string denoting the deleted_at field in the database.
	FieldDeletedAt = "deleted_at"
	// FieldTokenPairID holds the string denoting the token_pair_id field in the database.
	FieldTokenPairID = "token_pair_id"
	// FieldKPointType holds the string denoting the k_point_type field in the database.
	FieldKPointType = "k_point_type"
	// FieldOpen holds the string denoting the open field in the database.
	FieldOpen = "open"
	// FieldHigh holds the string denoting the high field in the database.
	FieldHigh = "high"
	// FieldLow holds the string denoting the low field in the database.
	FieldLow = "low"
	// FieldClose holds the string denoting the close field in the database.
	FieldClose = "close"
	// FieldStartTime holds the string denoting the start_time field in the database.
	FieldStartTime = "start_time"
	// FieldEndTime holds the string denoting the end_time field in the database.
	FieldEndTime = "end_time"
	// Table holds the table name of the kpoint in the database.
	Table = "kpoints"
)

// Columns holds all SQL columns for kpoint fields.
var Columns = []string{
	FieldID,
	FieldCreatedAt,
	FieldUpdatedAt,
	FieldDeletedAt,
	FieldTokenPairID,
	FieldKPointType,
	FieldOpen,
	FieldHigh,
	FieldLow,
	FieldClose,
	FieldStartTime,
	FieldEndTime,
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
	// DefaultKPointType holds the default value on creation for the "k_point_type" field.
	DefaultKPointType string
)
