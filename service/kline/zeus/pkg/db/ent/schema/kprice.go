package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/Geapefurit/kline-back/zeus/pkg/db/mixin"
)

type KPrice struct {
	ent.Schema
}

func (KPrice) Mixin() []ent.Mixin {
	return []ent.Mixin{
		mixin.TimeMixin{},
	}
}

func (KPrice) Fields() []ent.Field {
	return []ent.Field{
		field.Uint32("id"),
		field.Uint32("token_pair_id"),
		field.Float("price"),
		field.Uint32("timestamp"),
	}
}

func (KPrice) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("timestamp"),
		index.Fields("token_pair_id", "timestamp"),
	}
}
