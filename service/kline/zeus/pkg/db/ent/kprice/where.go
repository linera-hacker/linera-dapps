// Code generated by ent, DO NOT EDIT.

package kprice

import (
	"entgo.io/ent/dialect/sql"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/predicate"
)

// ID filters vertices based on their ID field.
func ID(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldID), id))
	})
}

// IDEQ applies the EQ predicate on the ID field.
func IDEQ(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldID), id))
	})
}

// IDNEQ applies the NEQ predicate on the ID field.
func IDNEQ(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldID), id))
	})
}

// IDIn applies the In predicate on the ID field.
func IDIn(ids ...uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		v := make([]interface{}, len(ids))
		for i := range v {
			v[i] = ids[i]
		}
		s.Where(sql.In(s.C(FieldID), v...))
	})
}

// IDNotIn applies the NotIn predicate on the ID field.
func IDNotIn(ids ...uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		v := make([]interface{}, len(ids))
		for i := range v {
			v[i] = ids[i]
		}
		s.Where(sql.NotIn(s.C(FieldID), v...))
	})
}

// IDGT applies the GT predicate on the ID field.
func IDGT(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldID), id))
	})
}

// IDGTE applies the GTE predicate on the ID field.
func IDGTE(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldID), id))
	})
}

// IDLT applies the LT predicate on the ID field.
func IDLT(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldID), id))
	})
}

// IDLTE applies the LTE predicate on the ID field.
func IDLTE(id uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldID), id))
	})
}

// CreatedAt applies equality check predicate on the "created_at" field. It's identical to CreatedAtEQ.
func CreatedAt(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldCreatedAt), v))
	})
}

// UpdatedAt applies equality check predicate on the "updated_at" field. It's identical to UpdatedAtEQ.
func UpdatedAt(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldUpdatedAt), v))
	})
}

// DeletedAt applies equality check predicate on the "deleted_at" field. It's identical to DeletedAtEQ.
func DeletedAt(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldDeletedAt), v))
	})
}

// TokenPairID applies equality check predicate on the "token_pair_id" field. It's identical to TokenPairIDEQ.
func TokenPairID(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldTokenPairID), v))
	})
}

// Price applies equality check predicate on the "price" field. It's identical to PriceEQ.
func Price(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldPrice), v))
	})
}

// Timestamp applies equality check predicate on the "timestamp" field. It's identical to TimestampEQ.
func Timestamp(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldTimestamp), v))
	})
}

// CreatedAtEQ applies the EQ predicate on the "created_at" field.
func CreatedAtEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldCreatedAt), v))
	})
}

// CreatedAtNEQ applies the NEQ predicate on the "created_at" field.
func CreatedAtNEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldCreatedAt), v))
	})
}

// CreatedAtIn applies the In predicate on the "created_at" field.
func CreatedAtIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.In(s.C(FieldCreatedAt), v...))
	})
}

// CreatedAtNotIn applies the NotIn predicate on the "created_at" field.
func CreatedAtNotIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NotIn(s.C(FieldCreatedAt), v...))
	})
}

// CreatedAtGT applies the GT predicate on the "created_at" field.
func CreatedAtGT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldCreatedAt), v))
	})
}

// CreatedAtGTE applies the GTE predicate on the "created_at" field.
func CreatedAtGTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldCreatedAt), v))
	})
}

// CreatedAtLT applies the LT predicate on the "created_at" field.
func CreatedAtLT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldCreatedAt), v))
	})
}

// CreatedAtLTE applies the LTE predicate on the "created_at" field.
func CreatedAtLTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldCreatedAt), v))
	})
}

// UpdatedAtEQ applies the EQ predicate on the "updated_at" field.
func UpdatedAtEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldUpdatedAt), v))
	})
}

// UpdatedAtNEQ applies the NEQ predicate on the "updated_at" field.
func UpdatedAtNEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldUpdatedAt), v))
	})
}

// UpdatedAtIn applies the In predicate on the "updated_at" field.
func UpdatedAtIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.In(s.C(FieldUpdatedAt), v...))
	})
}

// UpdatedAtNotIn applies the NotIn predicate on the "updated_at" field.
func UpdatedAtNotIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NotIn(s.C(FieldUpdatedAt), v...))
	})
}

// UpdatedAtGT applies the GT predicate on the "updated_at" field.
func UpdatedAtGT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldUpdatedAt), v))
	})
}

// UpdatedAtGTE applies the GTE predicate on the "updated_at" field.
func UpdatedAtGTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldUpdatedAt), v))
	})
}

// UpdatedAtLT applies the LT predicate on the "updated_at" field.
func UpdatedAtLT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldUpdatedAt), v))
	})
}

// UpdatedAtLTE applies the LTE predicate on the "updated_at" field.
func UpdatedAtLTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldUpdatedAt), v))
	})
}

// DeletedAtEQ applies the EQ predicate on the "deleted_at" field.
func DeletedAtEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldDeletedAt), v))
	})
}

// DeletedAtNEQ applies the NEQ predicate on the "deleted_at" field.
func DeletedAtNEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldDeletedAt), v))
	})
}

// DeletedAtIn applies the In predicate on the "deleted_at" field.
func DeletedAtIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.In(s.C(FieldDeletedAt), v...))
	})
}

// DeletedAtNotIn applies the NotIn predicate on the "deleted_at" field.
func DeletedAtNotIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NotIn(s.C(FieldDeletedAt), v...))
	})
}

// DeletedAtGT applies the GT predicate on the "deleted_at" field.
func DeletedAtGT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldDeletedAt), v))
	})
}

// DeletedAtGTE applies the GTE predicate on the "deleted_at" field.
func DeletedAtGTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldDeletedAt), v))
	})
}

// DeletedAtLT applies the LT predicate on the "deleted_at" field.
func DeletedAtLT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldDeletedAt), v))
	})
}

// DeletedAtLTE applies the LTE predicate on the "deleted_at" field.
func DeletedAtLTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldDeletedAt), v))
	})
}

// TokenPairIDEQ applies the EQ predicate on the "token_pair_id" field.
func TokenPairIDEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldTokenPairID), v))
	})
}

// TokenPairIDNEQ applies the NEQ predicate on the "token_pair_id" field.
func TokenPairIDNEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldTokenPairID), v))
	})
}

// TokenPairIDIn applies the In predicate on the "token_pair_id" field.
func TokenPairIDIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.In(s.C(FieldTokenPairID), v...))
	})
}

// TokenPairIDNotIn applies the NotIn predicate on the "token_pair_id" field.
func TokenPairIDNotIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NotIn(s.C(FieldTokenPairID), v...))
	})
}

// TokenPairIDGT applies the GT predicate on the "token_pair_id" field.
func TokenPairIDGT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldTokenPairID), v))
	})
}

// TokenPairIDGTE applies the GTE predicate on the "token_pair_id" field.
func TokenPairIDGTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldTokenPairID), v))
	})
}

// TokenPairIDLT applies the LT predicate on the "token_pair_id" field.
func TokenPairIDLT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldTokenPairID), v))
	})
}

// TokenPairIDLTE applies the LTE predicate on the "token_pair_id" field.
func TokenPairIDLTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldTokenPairID), v))
	})
}

// PriceEQ applies the EQ predicate on the "price" field.
func PriceEQ(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldPrice), v))
	})
}

// PriceNEQ applies the NEQ predicate on the "price" field.
func PriceNEQ(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldPrice), v))
	})
}

// PriceIn applies the In predicate on the "price" field.
func PriceIn(vs ...float64) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.In(s.C(FieldPrice), v...))
	})
}

// PriceNotIn applies the NotIn predicate on the "price" field.
func PriceNotIn(vs ...float64) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NotIn(s.C(FieldPrice), v...))
	})
}

// PriceGT applies the GT predicate on the "price" field.
func PriceGT(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldPrice), v))
	})
}

// PriceGTE applies the GTE predicate on the "price" field.
func PriceGTE(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldPrice), v))
	})
}

// PriceLT applies the LT predicate on the "price" field.
func PriceLT(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldPrice), v))
	})
}

// PriceLTE applies the LTE predicate on the "price" field.
func PriceLTE(v float64) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldPrice), v))
	})
}

// TimestampEQ applies the EQ predicate on the "timestamp" field.
func TimestampEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.EQ(s.C(FieldTimestamp), v))
	})
}

// TimestampNEQ applies the NEQ predicate on the "timestamp" field.
func TimestampNEQ(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NEQ(s.C(FieldTimestamp), v))
	})
}

// TimestampIn applies the In predicate on the "timestamp" field.
func TimestampIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.In(s.C(FieldTimestamp), v...))
	})
}

// TimestampNotIn applies the NotIn predicate on the "timestamp" field.
func TimestampNotIn(vs ...uint32) predicate.KPrice {
	v := make([]interface{}, len(vs))
	for i := range v {
		v[i] = vs[i]
	}
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.NotIn(s.C(FieldTimestamp), v...))
	})
}

// TimestampGT applies the GT predicate on the "timestamp" field.
func TimestampGT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GT(s.C(FieldTimestamp), v))
	})
}

// TimestampGTE applies the GTE predicate on the "timestamp" field.
func TimestampGTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.GTE(s.C(FieldTimestamp), v))
	})
}

// TimestampLT applies the LT predicate on the "timestamp" field.
func TimestampLT(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LT(s.C(FieldTimestamp), v))
	})
}

// TimestampLTE applies the LTE predicate on the "timestamp" field.
func TimestampLTE(v uint32) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s.Where(sql.LTE(s.C(FieldTimestamp), v))
	})
}

// And groups predicates with the AND operator between them.
func And(predicates ...predicate.KPrice) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s1 := s.Clone().SetP(nil)
		for _, p := range predicates {
			p(s1)
		}
		s.Where(s1.P())
	})
}

// Or groups predicates with the OR operator between them.
func Or(predicates ...predicate.KPrice) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		s1 := s.Clone().SetP(nil)
		for i, p := range predicates {
			if i > 0 {
				s1.Or()
			}
			p(s1)
		}
		s.Where(s1.P())
	})
}

// Not applies the not operator on the given predicate.
func Not(p predicate.KPrice) predicate.KPrice {
	return predicate.KPrice(func(s *sql.Selector) {
		p(s.Not())
	})
}
