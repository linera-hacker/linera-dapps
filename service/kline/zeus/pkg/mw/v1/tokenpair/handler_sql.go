package tokenpair

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/linera-hacker/linera-dapps/service/kline/zeus/pkg/db/ent/tokenpair"
)

type sqlHandler struct {
	*Handler
	BondTokenZeroID *uint32
	BondTokenOneID  *uint32
	bondVals        map[string]string
	baseVals        map[string]string
	idVals          map[string]string
}

func (h *Handler) newSQLHandler() *sqlHandler {
	return &sqlHandler{
		Handler:  h,
		bondVals: make(map[string]string),
		baseVals: make(map[string]string),
		idVals:   make(map[string]string),
	}
}

//nolint:gocognit
func (h *sqlHandler) baseKeys() error {
	if h.ID != nil {
		strBytes, err := json.Marshal(*h.ID)
		if err != nil {
			return err
		}
		h.baseVals[tokenpair.FieldID] = string(strBytes)
	}
	if h.PoolID != nil {
		strBytes, err := json.Marshal(*h.PoolID)
		if err != nil {
			return err
		}
		h.baseVals[tokenpair.FieldPoolID] = string(strBytes)
	}
	if h.TokenZeroID != nil {
		strBytes, err := json.Marshal(*h.TokenZeroID)
		if err != nil {
			return err
		}
		h.baseVals[tokenpair.FieldTokenZeroID] = string(strBytes)
		h.BondTokenZeroID = h.TokenZeroID
	}
	if h.Remark != nil {
		strBytes, err := json.Marshal(*h.Remark)
		if err != nil {
			return err
		}
		h.baseVals[tokenpair.FieldRemark] = string(strBytes)
	}
	if h.TokenOneID != nil {
		strBytes, err := json.Marshal(*h.TokenOneID)
		if err != nil {
			return err
		}
		h.baseVals[tokenpair.FieldTokenOneID] = string(strBytes)
		h.BondTokenOneID = h.TokenOneID
	}
	if h.BondTokenZeroID == nil {
		return fmt.Errorf("please give tokenzeroid")
	}
	strBytes, err := json.Marshal(*h.BondTokenZeroID)
	if err != nil {
		return err
	}
	h.bondVals[tokenpair.FieldTokenZeroID] = string(strBytes)

	if h.BondTokenOneID == nil {
		return fmt.Errorf("please give tokenoneid")
	}
	strBytes, err = json.Marshal(*h.BondTokenOneID)
	if err != nil {
		return err
	}
	h.bondVals[tokenpair.FieldTokenOneID] = string(strBytes)
	return nil
}

func (h *sqlHandler) idKeys() error {
	if h.ID != nil {
		strBytes, err := json.Marshal(*h.ID)
		if err != nil {
			return err
		}
		h.idVals[tokenpair.FieldID] = string(strBytes)
	}
	return nil
}

//nolint:gocognit
func (h *sqlHandler) genCreateSQL() (string, error) {
	err := h.baseKeys()
	if err != nil {
		return "", err
	}
	delete(h.baseVals, tokenpair.FieldID)

	now := uint32(time.Now().Unix())
	h.baseVals[tokenpair.FieldCreatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[tokenpair.FieldUpdatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[tokenpair.FieldDeletedAt] = fmt.Sprintf("%v", 0)

	keys := []string{}
	selectVals := []string{}
	bondVals := []string{}

	for k, v := range h.baseVals {
		keys = append(keys, k)
		selectVals = append(selectVals, fmt.Sprintf("%v as %v", v, k))
	}

	for k, v := range h.bondVals {
		bondVals = append(bondVals, fmt.Sprintf("%v=%v", k, v))
	}

	sql := fmt.Sprintf("insert into %v (%v) select * from (select %v) as tmp where not exists (select * from %v where %v and deleted_at=0);",
		tokenpair.Table,
		strings.Join(keys, ","),
		strings.Join(selectVals, ","),
		tokenpair.Table,
		strings.Join(bondVals, " AND "),
	)

	return sql, nil
}

//nolint:gocognit
func (h *sqlHandler) genUpdateSQL() (string, error) {
	// get normal feilds
	err := h.baseKeys()
	if err != nil {
		return "", err
	}
	delete(h.baseVals, tokenpair.FieldID)

	if len(h.baseVals) == 0 {
		return "", fmt.Errorf("update nothing")
	}

	now := uint32(time.Now().Unix())
	h.baseVals[tokenpair.FieldUpdatedAt] = fmt.Sprintf("%v", now)

	keys := []string{}
	for k, v := range h.baseVals {
		keys = append(keys, fmt.Sprintf("%v=%v", k, v))
	}

	err = h.idKeys()
	if err != nil {
		return "", err
	}
	if len(h.idVals) == 0 {
		return "", fmt.Errorf("have no id")
	}

	// get id and ent_id feilds
	idKeys := []string{}
	// get sub query feilds
	bondVals := []string{}
	for k, v := range h.idVals {
		idKeys = append(idKeys, fmt.Sprintf("%v=%v", k, v))
		bondVals = append(bondVals, fmt.Sprintf("tmp_table.%v!=%v", k, v))
	}

	for k, v := range h.bondVals {
		bondVals = append(bondVals, fmt.Sprintf("tmp_table.%v=%v", k, v))
	}

	sql := fmt.Sprintf("update %v set %v where %v and deleted_at=0 and  not exists (select 1 from(select * from %v as tmp_table where %v and tmp_table.deleted_at=0 limit 1) as tmp);",
		tokenpair.Table,
		strings.Join(keys, ","),
		strings.Join(idKeys, " AND "),
		tokenpair.Table,
		strings.Join(bondVals, " AND "),
	)

	return sql, nil
}
