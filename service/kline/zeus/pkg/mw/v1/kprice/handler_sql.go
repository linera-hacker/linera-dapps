package kprice

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/danced25519/linera-dapps/service/kline/zeus/pkg/db/ent/kprice"
)

type sqlHandler struct {
	*Handler
	BondTime        *uint32
	BondTokenPairID *uint32
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
		h.baseVals[kprice.FieldID] = string(strBytes)
	}
	if h.TokenPairID != nil {
		strBytes, err := json.Marshal(*h.TokenPairID)
		if err != nil {
			return err
		}
		h.baseVals[kprice.FieldTokenPairID] = string(strBytes)
		h.BondTokenPairID = h.TokenPairID
	}
	if h.Price != nil {
		strBytes, err := json.Marshal(*h.Price)
		if err != nil {
			return err
		}
		h.baseVals[kprice.FieldPrice] = string(strBytes)
	}
	if h.Timestamp != nil {
		strBytes, err := json.Marshal(*h.Timestamp)
		if err != nil {
			return err
		}
		h.baseVals[kprice.FieldTimestamp] = string(strBytes)
		h.BondTime = h.Timestamp
	}

	if h.BondTime == nil {
		return fmt.Errorf("please give time")
	}
	strBytes, err := json.Marshal(*h.BondTime)
	if err != nil {
		return err
	}
	h.bondVals[kprice.FieldTimestamp] = string(strBytes)

	if h.BondTokenPairID == nil {
		return fmt.Errorf("please give tokenpairid")
	}
	strBytes, err = json.Marshal(*h.BondTokenPairID)
	if err != nil {
		return err
	}
	h.bondVals[kprice.FieldTokenPairID] = string(strBytes)
	return nil
}

func (h *sqlHandler) idKeys() error {
	if h.ID != nil {
		strBytes, err := json.Marshal(*h.ID)
		if err != nil {
			return err
		}
		h.idVals[kprice.FieldID] = string(strBytes)
	}
	return nil
}

//nolint:gocognit
func (h *sqlHandler) genCreateSQL() (string, error) {
	err := h.baseKeys()
	if err != nil {
		return "", err
	}
	delete(h.baseVals, kprice.FieldID)

	now := uint32(time.Now().Unix())
	h.baseVals[kprice.FieldCreatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[kprice.FieldUpdatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[kprice.FieldDeletedAt] = fmt.Sprintf("%v", 0)

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
		kprice.Table,
		strings.Join(keys, ","),
		strings.Join(selectVals, ","),
		kprice.Table,
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
	delete(h.baseVals, kprice.FieldID)

	if len(h.baseVals) == 0 {
		return "", fmt.Errorf("update nothing")
	}

	now := uint32(time.Now().Unix())
	h.baseVals[kprice.FieldUpdatedAt] = fmt.Sprintf("%v", now)

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
		kprice.Table,
		strings.Join(keys, ","),
		strings.Join(idKeys, " AND "),
		kprice.Table,
		strings.Join(bondVals, " AND "),
	)

	return sql, nil
}
