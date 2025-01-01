package schema

import (
	"entgo.io/ent"
	"entgo.io/ent/schema/field"
	"entgo.io/ent/schema/index"
	basetype "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/basetype/v1"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/mixin"
)

type KPoint struct {
	ent.Schema
}

func (KPoint) Mixin() []ent.Mixin {
	return []ent.Mixin{
		mixin.TimeMixin{},
	}
}

func (KPoint) Fields() []ent.Field {
	return []ent.Field{
		field.Uint32("id"),
		field.Uint32("token_pair_id"),
		field.String("k_point_type").Optional().Default(basetype.KPointType_KPointTypeUnknown.String()),
		field.Float("open"),
		field.Float("high"),
		field.Float("low"),
		field.Float("close"),
		field.Uint32("start_time"),
		field.Uint32("end_time"),
		field.Uint32("start_date_timestamp"),
		field.Uint32("end_date_timestamp"),
	}
}

func (KPoint) Indexes() []ent.Index {
	return []ent.Index{
		index.Fields("token_pair_id"),
		index.Fields("token_pair_id", "k_point_type"),
		index.Fields("token_pair_id", "k_point_type", "start_date_timestamp"),
		index.Fields("token_pair_id", "k_point_type", "end_date_timestamp"),
	}
}
