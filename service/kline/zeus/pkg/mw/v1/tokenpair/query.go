package tokenpair

import (
	"context"
	"fmt"

	"entgo.io/ent/dialect/sql"
	tokenpairproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/tokenpair"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/token"
	tokenpairent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/tokenpair"

	tokenpaircrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/tokenpair"
)

type queryHandler struct {
	*Handler
	stm   *ent.TokenPairSelect
	infos []*tokenpairproto.TokenPair
}

func (h *queryHandler) selectTokenPair(stm *ent.TokenPairQuery) {
	h.stm = stm.Select(
		tokenpairent.FieldID,
		tokenpairent.FieldPoolID,
		tokenpairent.FieldCreatedAt,
		tokenpairent.FieldUpdatedAt,
		tokenpairent.FieldTokenZeroID,
		tokenpairent.FieldTokenOneID,
		tokenpairent.FieldRemark,
	)
}

func (h *queryHandler) queryTokenPair(cli *ent.Client) error {
	if h.ID == nil {
		return fmt.Errorf("invalid id")
	}
	stm := cli.TokenPair.Query().Where(tokenpairent.DeletedAt(0))
	if h.ID != nil {
		stm.Where(tokenpairent.ID(*h.ID))
	}
	h.selectTokenPair(stm)
	return nil
}

func (h *queryHandler) queryTokenPairs(ctx context.Context, cli *ent.Client) error {
	stm, err := tokenpaircrud.SetQueryConds(cli.TokenPair.Query(), h.Conds)
	if err != nil {
		return err
	}

	h.selectTokenPair(stm)
	return nil
}

func (h *queryHandler) queryJoin() {
	h.stm.Modify(h.queryJoinToken)
}

func (h *queryHandler) queryJoinToken(s *sql.Selector) {
	tokenZeroT := sql.Table(token.Table)
	tokenOneT := sql.Table(token.Table)
	s.Join(tokenZeroT).On(
		s.C(tokenpairent.FieldTokenZeroID),
		tokenZeroT.C(token.FieldID),
	).OnP(
		sql.EQ(tokenZeroT.C(token.FieldDeletedAt), 0),
	).Join(tokenOneT).On(
		s.C(tokenpairent.FieldTokenOneID),
		tokenOneT.C(token.FieldID),
	).OnP(
		sql.EQ(tokenOneT.C(token.FieldDeletedAt), 0),
	).AppendSelect(
		sql.As(tokenZeroT.C(token.FieldName), "token_zero_name"),
		sql.As(tokenZeroT.C(token.FieldAddress), "token_zero_address"),
		sql.As(tokenZeroT.C(token.FieldIcon), "token_zero_icon"),
		sql.As(tokenZeroT.C(token.FieldSymbol), "token_zero_symbol"),
		sql.As(tokenOneT.C(token.FieldName), "token_one_name"),
		sql.As(tokenOneT.C(token.FieldAddress), "token_one_address"),
		sql.As(tokenOneT.C(token.FieldIcon), "token_one_icon"),
		sql.As(tokenOneT.C(token.FieldSymbol), "token_one_symbol"),
	).Distinct()
}

func (h *queryHandler) scan(ctx context.Context) error {
	return h.stm.Scan(ctx, &h.infos)
}

func (h *Handler) GetTokenPair(ctx context.Context) (*tokenpairproto.TokenPair, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTokenPair(cli); err != nil {
			return err
		}
		handler.queryJoin()
		const singleRowLimit = 2
		handler.stm.Offset(0).Limit(singleRowLimit)
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}
	if len(handler.infos) == 0 {
		return nil, nil
	}
	if len(handler.infos) > 1 {
		return nil, fmt.Errorf("too many record")
	}

	return handler.infos[0], nil
}

func (h *Handler) GetTokenPairs(ctx context.Context) ([]*tokenpairproto.TokenPair, error) {
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTokenPairs(ctx, cli); err != nil {
			return err
		}
		handler.queryJoin()
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(tokenpairent.FieldUpdatedAt))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, err
	}

	return handler.infos, nil
}
