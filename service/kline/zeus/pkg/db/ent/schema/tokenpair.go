package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/mixin"
)

type TokenPair struct {
	ent.Schema
}

func (TokenPair) Mixin() []ent.Mixin {
	return []ent.Mixin{
		mixin.TimeMixin{},
	}
}

func (TokenPair) Fields() []ent.Field {
	return []ent.Field{
		field.Uint32("id"),
		field.Uint64("pool_id"),
		field.Uint32("token_zero_id"),
		field.Uint32("token_one_id"),
		field.String("remark").Optional(),
	}
}

func (TokenPair) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("pool_id"),
		index.Fields("token_zero_id", "token_one_id").Unique(),
	}
}
