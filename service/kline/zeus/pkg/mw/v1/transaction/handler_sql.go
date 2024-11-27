package transaction

import (
	"encoding/json"
	"fmt"
	"strings"
	"time"

	"github.com/Geapefurit/kline-back/zeus/pkg/db/ent/transaction"
)

type sqlHandler struct {
	*Handler
	BondTransactionID *uint64
	bondVals          map[string]string
	baseVals          map[string]string
	idVals            map[string]string
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
		h.baseVals[transaction.FieldID] = string(strBytes)
	}
	if h.TransactionID != nil {
		strBytes, err := json.Marshal(*h.TransactionID)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldTransactionID] = string(strBytes)
		h.BondTransactionID = h.TransactionID
	}
	if h.PoolID != nil {
		strBytes, err := json.Marshal(*h.PoolID)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldPoolID] = string(strBytes)
	}
	if h.TransactionType != nil {
		strBytes, err := json.Marshal(*h.TransactionType)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldTransactionType] = string(strBytes)
	}
	if h.AmountZeroIn != nil {
		strBytes, err := json.Marshal(*h.AmountZeroIn)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldAmountZeroIn] = string(strBytes)
	}
	if h.AmountOneIn != nil {
		strBytes, err := json.Marshal(*h.AmountOneIn)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldAmountOneIn] = string(strBytes)
	}
	if h.AmountZeroOut != nil {
		strBytes, err := json.Marshal(*h.AmountZeroOut)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldAmountZeroOut] = string(strBytes)
	}
	if h.AmountOneOut != nil {
		strBytes, err := json.Marshal(*h.AmountOneOut)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldAmountOneOut] = string(strBytes)
	}
	if h.Timestamp != nil {
		strBytes, err := json.Marshal(*h.Timestamp)
		if err != nil {
			return err
		}
		h.baseVals[transaction.FieldTimestamp] = string(strBytes)
	}

	if h.BondTransactionID == nil {
		return fmt.Errorf("please give tokenpairid")
	}
	strBytes, err := json.Marshal(*h.BondTransactionID)
	if err != nil {
		return err
	}
	h.bondVals[transaction.FieldTransactionID] = string(strBytes)
	return nil
}

func (h *sqlHandler) idKeys() error {
	if h.ID != nil {
		strBytes, err := json.Marshal(*h.ID)
		if err != nil {
			return err
		}
		h.idVals[transaction.FieldID] = string(strBytes)
	}
	return nil
}

//nolint:gocognit
func (h *sqlHandler) genCreateSQL() (string, error) {
	err := h.baseKeys()
	if err != nil {
		return "", err
	}
	delete(h.baseVals, transaction.FieldID)

	now := uint32(time.Now().Unix())
	h.baseVals[transaction.FieldCreatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[transaction.FieldUpdatedAt] = fmt.Sprintf("%v", now)
	h.baseVals[transaction.FieldDeletedAt] = fmt.Sprintf("%v", 0)

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
		transaction.Table,
		strings.Join(keys, ","),
		strings.Join(selectVals, ","),
		transaction.Table,
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
	delete(h.baseVals, transaction.FieldID)

	if len(h.baseVals) == 0 {
		return "", fmt.Errorf("update nothing")
	}

	now := uint32(time.Now().Unix())
	h.baseVals[transaction.FieldUpdatedAt] = fmt.Sprintf("%v", now)

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
		transaction.Table,
		strings.Join(keys, ","),
		strings.Join(idKeys, " AND "),
		transaction.Table,
		strings.Join(bondVals, " AND "),
	)

	return sql, nil
}
