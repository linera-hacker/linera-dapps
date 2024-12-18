package token

import (
	"fmt"

	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent"
	enttoken "github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/token"
)

type Req struct {
	ID        *uint32
	Address   *string
	Site      *string
	Icon      *string
	Name      *string
	Symbol    *string
	DeletedAt *uint32
}

func CreateSet(c *ent.TokenCreate, req *Req) *ent.TokenCreate {
	if req.Address != nil {
		c.SetAddress(*req.Address)
	}
	if req.Site != nil {
		c.SetSite(*req.Site)
	}
	if req.Icon != nil {
		c.SetIcon(*req.Icon)
	}
	if req.Name != nil {
		c.SetName(*req.Name)
	}
	if req.Symbol != nil {
		c.SetSymbol(*req.Symbol)
	}
	return c
}

func UpdateSet(u *ent.TokenUpdateOne, req *Req) (*ent.TokenUpdateOne, error) {
	if req.Address != nil {
		u.SetAddress(*req.Address)
	}
	if req.Site != nil {
		u.SetSite(*req.Site)
	}
	if req.Icon != nil {
		u.SetIcon(*req.Icon)
	}
	if req.Name != nil {
		u.SetName(*req.Name)
	}
	if req.Symbol != nil {
		u.SetSymbol(*req.Symbol)
	}
	if req.DeletedAt != nil {
		u = u.SetDeletedAt(*req.DeletedAt)
	}
	return u, nil
}

type Conds struct {
	ID      *cruder.Cond
	IDs     *cruder.Cond
	Address *cruder.Cond
	Site    *cruder.Cond
	Icon    *cruder.Cond
	Name    *cruder.Cond
	Symbol  *cruder.Cond
}

func SetQueryConds(q *ent.TokenQuery, conds *Conds) (*ent.TokenQuery, error) { //nolint
	if conds.ID != nil {
		id, ok := conds.Address.Val.(uint32)
		if !ok {
			return nil, fmt.Errorf("invalid id")
		}
		switch conds.Address.Op {
		case cruder.EQ:
			q.Where(enttoken.ID(id))
		default:
			return nil, fmt.Errorf("invalid id field")
		}
	}
	if conds.IDs != nil {
		ids, ok := conds.IDs.Val.([]uint32)
		if !ok {
			return nil, fmt.Errorf("invalid ids")
		}
		switch conds.IDs.Op {
		case cruder.IN:
			q.Where(enttoken.IDIn(ids...))
		default:
			return nil, fmt.Errorf("invalid ids field")
		}
	}
	if conds.Address != nil {
		address, ok := conds.Address.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid address")
		}
		switch conds.Address.Op {
		case cruder.EQ:
			q.Where(enttoken.Address(address))
		default:
			return nil, fmt.Errorf("invalid address field")
		}
	}
	if conds.Site != nil {
		site, ok := conds.Site.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid site")
		}
		switch conds.Site.Op {
		case cruder.EQ:
			q.Where(enttoken.Site(site))
		default:
			return nil, fmt.Errorf("invalid site field")
		}
	}
	if conds.Icon != nil {
		icon, ok := conds.Icon.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid icon")
		}
		switch conds.Icon.Op {
		case cruder.EQ:
			q.Where(enttoken.Icon(icon))
		default:
			return nil, fmt.Errorf("invalid icon field")
		}
	}
	if conds.Name != nil {
		name, ok := conds.Name.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid name")
		}
		switch conds.Name.Op {
		case cruder.EQ:
			q.Where(enttoken.Name(name))
		default:
			return nil, fmt.Errorf("invalid name field")
		}
	}
	if conds.Symbol != nil {
		symbol, ok := conds.Symbol.Val.(string)
		if !ok {
			return nil, fmt.Errorf("invalid symbol")
		}
		switch conds.Symbol.Op {
		case cruder.EQ:
			q.Where(enttoken.Symbol(symbol))
		default:
			return nil, fmt.Errorf("invalid symbol field")
		}
	}
	return q, nil
}
