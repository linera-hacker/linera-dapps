package token

import (
	"context"
	"fmt"

	tokenproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/token"
	tokencrud "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/crud/v1/token"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	tokenent "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/token"
)

type queryHandler struct {
	*Handler
	stm   *ent.TokenSelect
	infos []*tokenproto.Token
	total uint32
}

func (h *queryHandler) selectToken(stm *ent.TokenQuery) {
	h.stm = stm.Select(
		tokenent.FieldID,
		tokenent.FieldAddress,
		tokenent.FieldSite,
		tokenent.FieldIcon,
		tokenent.FieldName,
		tokenent.FieldSymbol,
		tokenent.FieldCreatedAt,
		tokenent.FieldUpdatedAt,
	)
}

func (h *queryHandler) queryToken(cli *ent.Client) error {
	if h.ID == nil {
		return fmt.Errorf("invalid id")
	}
	stm := cli.Token.Query().Where(tokenent.DeletedAt(0))
	if h.ID != nil {
		stm.Where(tokenent.ID(*h.ID))
	}
	h.selectToken(stm)
	return nil
}

func (h *queryHandler) queryTokens(ctx context.Context, cli *ent.Client) error {
	stm, err := tokencrud.SetQueryConds(cli.Token.Query(), h.Conds)
	if err != nil {
		return err
	}

	total, err := stm.Count(ctx)
	if err != nil {
		return err
	}
	h.total = uint32(total)

	h.selectToken(stm)
	return nil
}

func (h *queryHandler) scan(ctx context.Context) error {
	return h.stm.Scan(ctx, &h.infos)
}

func (h *Handler) GetToken(ctx context.Context) (*tokenproto.Token, error) {
	handler := &queryHandler{
		Handler: h,
	}
	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryToken(cli); err != nil {
			return err
		}
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

func (h *Handler) GetTokens(ctx context.Context) ([]*tokenproto.Token, uint32, error) {
	if h.Conds == nil {
		return nil, 0, fmt.Errorf("the conds is nil")
	}
	handler := &queryHandler{
		Handler: h,
	}

	err := db.WithClient(ctx, func(_ctx context.Context, cli *ent.Client) error {
		if err := handler.queryTokens(ctx, cli); err != nil {
			return err
		}
		handler.stm.
			Offset(int(h.Offset)).
			Limit(int(h.Limit)).
			Order(ent.Desc(tokenent.FieldUpdatedAt))
		return handler.scan(_ctx)
	})
	if err != nil {
		return nil, 0, err
	}
	return handler.infos, handler.total, nil
}
