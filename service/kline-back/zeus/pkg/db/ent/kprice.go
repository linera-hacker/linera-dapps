// Code generated by ent, DO NOT EDIT.

package ent

import (
	"fmt"
	"strings"

	"entgo.io/ent/dialect/sql"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent/kprice"
)

// KPrice is the model entity for the KPrice schema.
type KPrice struct {
	config `json:"-"`
	// ID of the ent.
	ID uint32 `json:"id,omitempty"`
	// CreatedAt holds the value of the "created_at" field.
	CreatedAt uint32 `json:"created_at,omitempty"`
	// UpdatedAt holds the value of the "updated_at" field.
	UpdatedAt uint32 `json:"updated_at,omitempty"`
	// DeletedAt holds the value of the "deleted_at" field.
	DeletedAt uint32 `json:"deleted_at,omitempty"`
	// TokenPairID holds the value of the "token_pair_id" field.
	TokenPairID uint32 `json:"token_pair_id,omitempty"`
	// Price holds the value of the "price" field.
	Price float64 `json:"price,omitempty"`
	// Timestamp holds the value of the "timestamp" field.
	Timestamp uint32 `json:"timestamp,omitempty"`
}

// scanValues returns the types for scanning values from sql.Rows.
func (*KPrice) scanValues(columns []string) ([]interface{}, error) {
	values := make([]interface{}, len(columns))
	for i := range columns {
		switch columns[i] {
		case kprice.FieldPrice:
			values[i] = new(sql.NullFloat64)
		case kprice.FieldID, kprice.FieldCreatedAt, kprice.FieldUpdatedAt, kprice.FieldDeletedAt, kprice.FieldTokenPairID, kprice.FieldTimestamp:
			values[i] = new(sql.NullInt64)
		default:
			return nil, fmt.Errorf("unexpected column %q for type KPrice", columns[i])
		}
	}
	return values, nil
}

// assignValues assigns the values that were returned from sql.Rows (after scanning)
// to the KPrice fields.
func (k *KPrice) assignValues(columns []string, values []interface{}) error {
	if m, n := len(values), len(columns); m < n {
		return fmt.Errorf("mismatch number of scan values: %d != %d", m, n)
	}
	for i := range columns {
		switch columns[i] {
		case kprice.FieldID:
			value, ok := values[i].(*sql.NullInt64)
			if !ok {
				return fmt.Errorf("unexpected type %T for field id", value)
			}
			k.ID = uint32(value.Int64)
		case kprice.FieldCreatedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field created_at", values[i])
			} else if value.Valid {
				k.CreatedAt = uint32(value.Int64)
			}
		case kprice.FieldUpdatedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field updated_at", values[i])
			} else if value.Valid {
				k.UpdatedAt = uint32(value.Int64)
			}
		case kprice.FieldDeletedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field deleted_at", values[i])
			} else if value.Valid {
				k.DeletedAt = uint32(value.Int64)
			}
		case kprice.FieldTokenPairID:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field token_pair_id", values[i])
			} else if value.Valid {
				k.TokenPairID = uint32(value.Int64)
			}
		case kprice.FieldPrice:
			if value, ok := values[i].(*sql.NullFloat64); !ok {
				return fmt.Errorf("unexpected type %T for field price", values[i])
			} else if value.Valid {
				k.Price = value.Float64
			}
		case kprice.FieldTimestamp:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field timestamp", values[i])
			} else if value.Valid {
				k.Timestamp = uint32(value.Int64)
			}
		}
	}
	return nil
}

// Update returns a builder for updating this KPrice.
// Note that you need to call KPrice.Unwrap() before calling this method if this KPrice
// was returned from a transaction, and the transaction was committed or rolled back.
func (k *KPrice) Update() *KPriceUpdateOne {
	return (&KPriceClient{config: k.config}).UpdateOne(k)
}

// Unwrap unwraps the KPrice entity that was returned from a transaction after it was closed,
// so that all future queries will be executed through the driver which created the transaction.
func (k *KPrice) Unwrap() *KPrice {
	_tx, ok := k.config.driver.(*txDriver)
	if !ok {
		panic("ent: KPrice is not a transactional entity")
	}
	k.config.driver = _tx.drv
	return k
}

// String implements the fmt.Stringer.
func (k *KPrice) String() string {
	var builder strings.Builder
	builder.WriteString("KPrice(")
	builder.WriteString(fmt.Sprintf("id=%v, ", k.ID))
	builder.WriteString("created_at=")
	builder.WriteString(fmt.Sprintf("%v", k.CreatedAt))
	builder.WriteString(", ")
	builder.WriteString("updated_at=")
	builder.WriteString(fmt.Sprintf("%v", k.UpdatedAt))
	builder.WriteString(", ")
	builder.WriteString("deleted_at=")
	builder.WriteString(fmt.Sprintf("%v", k.DeletedAt))
	builder.WriteString(", ")
	builder.WriteString("token_pair_id=")
	builder.WriteString(fmt.Sprintf("%v", k.TokenPairID))
	builder.WriteString(", ")
	builder.WriteString("price=")
	builder.WriteString(fmt.Sprintf("%v", k.Price))
	builder.WriteString(", ")
	builder.WriteString("timestamp=")
	builder.WriteString(fmt.Sprintf("%v", k.Timestamp))
	builder.WriteByte(')')
	return builder.String()
}

// KPrices is a parsable slice of KPrice.
type KPrices []*KPrice

func (k KPrices) config(cfg config) {
	for _i := range k {
		k[_i].config = cfg
	}
}
