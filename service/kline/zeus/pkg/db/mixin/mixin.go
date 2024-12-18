package mixin

import (
	"entgo.io/ent"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/privacy"
	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/rule"
)

func (TimeMixin) Mixin() []ent.Mixin {
	return []ent.Mixin{
		TimeMixin{},
	}
}

func (TimeMixin) Policy() ent.Policy {
	return privacy.Policy{
		Query: privacy.QueryPolicy{
			rule.FilterTimeRule(),
		},
	}
}
