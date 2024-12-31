package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/mixin"
)

type Transaction struct {
	ent.Schema
}

func (Transaction) Mixin() []ent.Mixin {
	return []ent.Mixin{
		mixin.TimeMixin{},
	}
}

func (Transaction) Fields() []ent.Field {
	return []ent.Field{
		field.Uint32("id"),
		field.Uint64("pool_id"),
		field.Uint64("transaction_id"),
		field.String("transaction_type"),
		field.String("chain_id"),
		field.String("owner"),
		field.Float("amount_zero_in"),
		field.Float("amount_one_in"),
		field.Float("amount_zero_out"),
		field.Float("amount_one_out"),
		field.Uint32("timestamp"),
	}
}

func (Transaction) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("timestamp"),
		index.Fields("transaction_id"),
		index.Fields("pool_id", "timestamp"),
		index.Fields("timestamp", "pool_id"),
	}
}
