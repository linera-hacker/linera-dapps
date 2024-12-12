package kpoint

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent/kpoint"
)

type sqlHandler struct {
	*Handler
	BondKPointType  *string
	BondEndTime     *uint32
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
		h.baseVals[kpoint.FieldID] = string(strBytes)
	}
	if h.TokenPairID != nil {
		strBytes, err := json.Marshal(*h.TokenPairID)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldTokenPairID] = string(strBytes)
		h.BondTokenPairID = h.TokenPairID
	}
	if h.KPointType != nil {
		strBytes, err := json.Marshal(h.KPointType.String())
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldKPointType] = string(strBytes)
		kpType := h.KPointType.String()
		h.BondKPointType = &kpType
	}
	if h.Open != nil {
		strBytes, err := json.Marshal(*h.Open)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldOpen] = string(strBytes)
	}
	if h.High != nil {
		strBytes, err := json.Marshal(*h.High)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldHigh] = string(strBytes)
	}
	if h.Low != nil {
		strBytes, err := json.Marshal(*h.Low)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldLow] = string(strBytes)
	}
	if h.Close != nil {
		strBytes, err := json.Marshal(*h.Close)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldClose] = string(strBytes)
	}
	if h.StartTime != nil {
		strBytes, err := json.Marshal(*h.StartTime)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldStartTime] = string(strBytes)
	}
	if h.EndTime != nil {
		strBytes, err := json.Marshal(*h.EndTime)
		if err != nil {
			return err
		}
		h.baseVals[kpoint.FieldEndTime] = string(strBytes)
		h.BondEndTime = h.EndTime
	}
	if h.BondKPointType == nil {
		return fmt.Errorf("please give kpointtype")
	}
	strBytes, err := json.Marshal(*h.BondKPointType)
	if err != nil {
		return err
	}
	h.bondVals[kpoint.FieldKPointType] = string(strBytes)

	if h.BondEndTime == nil {
		return fmt.Errorf("please give endtime")
	}
	strBytes, err = json.Marshal(*h.BondEndTime)
	if err != nil {
		return err
	}
	h.bondVals[kpoint.FieldEndTime] = string(strBytes)

	if h.BondTokenPairID == nil {
		return fmt.Errorf("please give tokenpairid")
	}
	strBytes, err = json.Marshal(*h.BondTokenPairID)
	if err != nil {
		return err
	}
	h.bondVals[kpoint.FieldTokenPairID] = string(strBytes)
	return nil
}

func (h *sqlHandler) idKeys() error {
	if h.ID != nil {
		strBytes, err := json.Marshal(*h.ID)
		if err != nil {
			return err
		}
		h.idVals[kpoint.FieldID] = string(strBytes)
	}
	return nil
}

//nolint:gocognit
func (h *sqlHandler) genCreateSQL() (string, error) {
	err := h.baseKeys()
	if err != nil {
		return "", err
	}
	delete(h.baseVals, kpoint.FieldID)

	now := uint32(time.Now().Unix())
	h.baseVals[kpoint.FieldCreatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[kpoint.FieldUpdatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[kpoint.FieldDeletedAt] = fmt.Sprintf("%v", 0)

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
		kpoint.Table,
		strings.Join(keys, ","),
		strings.Join(selectVals, ","),
		kpoint.Table,
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
	delete(h.baseVals, kpoint.FieldID)

	if len(h.baseVals) == 0 {
		return "", fmt.Errorf("update nothing")
	}

	now := uint32(time.Now().Unix())
	h.baseVals[kpoint.FieldUpdatedAt] = fmt.Sprintf("%v", now)

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
		kpoint.Table,
		strings.Join(keys, ","),
		strings.Join(idKeys, " AND "),
		kpoint.Table,
		strings.Join(bondVals, " AND "),
	)

	return sql, nil
}
