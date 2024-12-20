// Code generated by ent, DO NOT EDIT.

package ent

import (
	"fmt"
	"strings"

	"entgo.io/ent/dialect/sql"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/kpoint"
)

// KPoint is the model entity for the KPoint schema.
type KPoint struct {
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
	// KPointType holds the value of the "k_point_type" field.
	KPointType string `json:"k_point_type,omitempty"`
	// Open holds the value of the "open" field.
	Open float64 `json:"open,omitempty"`
	// High holds the value of the "high" field.
	High float64 `json:"high,omitempty"`
	// Low holds the value of the "low" field.
	Low float64 `json:"low,omitempty"`
	// Close holds the value of the "close" field.
	Close float64 `json:"close,omitempty"`
	// StartTime holds the value of the "start_time" field.
	StartTime uint32 `json:"start_time,omitempty"`
	// EndTime holds the value of the "end_time" field.
	EndTime uint32 `json:"end_time,omitempty"`
}

// scanValues returns the types for scanning values from sql.Rows.
func (*KPoint) scanValues(columns []string) ([]interface{}, error) {
	values := make([]interface{}, len(columns))
	for i := range columns {
		switch columns[i] {
		case kpoint.FieldOpen, kpoint.FieldHigh, kpoint.FieldLow, kpoint.FieldClose:
			values[i] = new(sql.NullFloat64)
		case kpoint.FieldID, kpoint.FieldCreatedAt, kpoint.FieldUpdatedAt, kpoint.FieldDeletedAt, kpoint.FieldTokenPairID, kpoint.FieldStartTime, kpoint.FieldEndTime:
			values[i] = new(sql.NullInt64)
		case kpoint.FieldKPointType:
			values[i] = new(sql.NullString)
		default:
			return nil, fmt.Errorf("unexpected column %q for type KPoint", columns[i])
		}
	}
	return values, nil
}

// assignValues assigns the values that were returned from sql.Rows (after scanning)
// to the KPoint fields.
func (k *KPoint) assignValues(columns []string, values []interface{}) error {
	if m, n := len(values), len(columns); m < n {
		return fmt.Errorf("mismatch number of scan values: %d != %d", m, n)
	}
	for i := range columns {
		switch columns[i] {
		case kpoint.FieldID:
			value, ok := values[i].(*sql.NullInt64)
			if !ok {
				return fmt.Errorf("unexpected type %T for field id", value)
			}
			k.ID = uint32(value.Int64)
		case kpoint.FieldCreatedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field created_at", values[i])
			} else if value.Valid {
				k.CreatedAt = uint32(value.Int64)
			}
		case kpoint.FieldUpdatedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field updated_at", values[i])
			} else if value.Valid {
				k.UpdatedAt = uint32(value.Int64)
			}
		case kpoint.FieldDeletedAt:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field deleted_at", values[i])
			} else if value.Valid {
				k.DeletedAt = uint32(value.Int64)
			}
		case kpoint.FieldTokenPairID:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field token_pair_id", values[i])
			} else if value.Valid {
				k.TokenPairID = uint32(value.Int64)
			}
		case kpoint.FieldKPointType:
			if value, ok := values[i].(*sql.NullString); !ok {
				return fmt.Errorf("unexpected type %T for field k_point_type", values[i])
			} else if value.Valid {
				k.KPointType = value.String
			}
		case kpoint.FieldOpen:
			if value, ok := values[i].(*sql.NullFloat64); !ok {
				return fmt.Errorf("unexpected type %T for field open", values[i])
			} else if value.Valid {
				k.Open = value.Float64
			}
		case kpoint.FieldHigh:
			if value, ok := values[i].(*sql.NullFloat64); !ok {
				return fmt.Errorf("unexpected type %T for field high", values[i])
			} else if value.Valid {
				k.High = value.Float64
			}
		case kpoint.FieldLow:
			if value, ok := values[i].(*sql.NullFloat64); !ok {
				return fmt.Errorf("unexpected type %T for field low", values[i])
			} else if value.Valid {
				k.Low = value.Float64
			}
		case kpoint.FieldClose:
			if value, ok := values[i].(*sql.NullFloat64); !ok {
				return fmt.Errorf("unexpected type %T for field close", values[i])
			} else if value.Valid {
				k.Close = value.Float64
			}
		case kpoint.FieldStartTime:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field start_time", values[i])
			} else if value.Valid {
				k.StartTime = uint32(value.Int64)
			}
		case kpoint.FieldEndTime:
			if value, ok := values[i].(*sql.NullInt64); !ok {
				return fmt.Errorf("unexpected type %T for field end_time", values[i])
			} else if value.Valid {
				k.EndTime = uint32(value.Int64)
			}
		}
	}
	return nil
}

// Update returns a builder for updating this KPoint.
// Note that you need to call KPoint.Unwrap() before calling this method if this KPoint
// was returned from a transaction, and the transaction was committed or rolled back.
func (k *KPoint) Update() *KPointUpdateOne {
	return (&KPointClient{config: k.config}).UpdateOne(k)
}

// Unwrap unwraps the KPoint entity that was returned from a transaction after it was closed,
// so that all future queries will be executed through the driver which created the transaction.
func (k *KPoint) Unwrap() *KPoint {
	_tx, ok := k.config.driver.(*txDriver)
	if !ok {
		panic("ent: KPoint is not a transactional entity")
	}
	k.config.driver = _tx.drv
	return k
}

// String implements the fmt.Stringer.
func (k *KPoint) String() string {
	var builder strings.Builder
	builder.WriteString("KPoint(")
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
	builder.WriteString("k_point_type=")
	builder.WriteString(k.KPointType)
	builder.WriteString(", ")
	builder.WriteString("open=")
	builder.WriteString(fmt.Sprintf("%v", k.Open))
	builder.WriteString(", ")
	builder.WriteString("high=")
	builder.WriteString(fmt.Sprintf("%v", k.High))
	builder.WriteString(", ")
	builder.WriteString("low=")
	builder.WriteString(fmt.Sprintf("%v", k.Low))
	builder.WriteString(", ")
	builder.WriteString("close=")
	builder.WriteString(fmt.Sprintf("%v", k.Close))
	builder.WriteString(", ")
	builder.WriteString("start_time=")
	builder.WriteString(fmt.Sprintf("%v", k.StartTime))
	builder.WriteString(", ")
	builder.WriteString("end_time=")
	builder.WriteString(fmt.Sprintf("%v", k.EndTime))
	builder.WriteByte(')')
	return builder.String()
}

// KPoints is a parsable slice of KPoint.
type KPoints []*KPoint

func (k KPoints) config(cfg config) {
	for _i := range k {
		k[_i].config = cfg
	}
}
