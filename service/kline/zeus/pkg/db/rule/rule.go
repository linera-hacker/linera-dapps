package rule

import (
	"context"

	"entgo.io/ent/entql"
	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent/privacy"
)

func FilterTimeRule() privacy.QueryMutationRule {
	return privacy.FilterFunc(func(ctx context.Context, f privacy.Filter) error {
		f.Where(entql.FieldEQ("deleted_at", 0))
		return privacy.Skip
	})
}
