package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/mixin"
)

type Token struct {
	ent.Schema
}

func (Token) Mixin() []ent.Mixin {
	return []ent.Mixin{
		mixin.TimeMixin{},
	}
}

func (Token) Fields() []ent.Field {
	return []ent.Field{
		field.Uint32("id"),
		field.String("address"),
		field.String("site"),
		field.Text("icon"),
		field.String("name"),
		field.String("symbol"),
	}
}

func (Token) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("address"),
		index.Fields("symbol"),
	}
}
