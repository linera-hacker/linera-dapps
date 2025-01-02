package kptype

import (
	"time"

	basetype "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/basetype/v1"
	kpointproto "github.com/linera-hacker/linera-dapps/service/kline/proto/kline/zeus/v1/kpoint"
)

var KPointTypeInfos = map[basetype.KPointType]*kpointproto.KPointTypeInfo{
	basetype.KPointType_FiveSecond: {
		KPointType: basetype.KPointType_FiveSecond,
		Seconds:    5,
		ShortName:  "5s",
	},
	basetype.KPointType_OneMinute: {
		KPointType: basetype.KPointType_OneMinute,
		Seconds:    60,
		ShortName:  "1m",
	},
	basetype.KPointType_TenMinute: {
		KPointType: basetype.KPointType_TenMinute,
		Seconds:    60 * 10,
		ShortName:  "10m",
	},
	basetype.KPointType_OneHour: {
		KPointType: basetype.KPointType_OneHour,
		Seconds:    60 * 60,
		ShortName:  "1H",
	},
	basetype.KPointType_OneDay: {
		KPointType: basetype.KPointType_OneDay,
		Seconds:    60 * 60 * 24,
		ShortName:  "1D",
	},
	basetype.KPointType_OneWeek: {
		KPointType: basetype.KPointType_OneWeek,
		Seconds:    60 * 60 * 24 * 7,
		ShortName:  "1W",
	},
	basetype.KPointType_OneMonth: {
		KPointType: basetype.KPointType_OneMonth,
		Seconds:    60 * 60 * 24 * 30,
		ShortName:  "1M",
	},
}

var KPTypeSampleSecond = map[basetype.KPointType]uint32{
	basetype.KPointType_FiveSecond: 5,
	basetype.KPointType_OneMinute:  30,
	basetype.KPointType_TenMinute:  60,
	basetype.KPointType_OneHour:    60 * 5,
	basetype.KPointType_OneDay:     60 * 60,
	basetype.KPointType_OneWeek:    60 * 60,
	basetype.KPointType_OneMonth:   60 * 60,
}

func FormatU32Time(t uint32) string {
	tt := time.Unix(int64(t), 0)
	if t%KPointTypeInfos[basetype.KPointType_OneDay].Seconds == 0 {
		return tt.Format("2006-01-02")
	}
	if t%KPointTypeInfos[basetype.KPointType_OneHour].Seconds == 0 {
		return tt.Format("2006-01-02 14h")
	}

	if t%KPointTypeInfos[basetype.KPointType_OneMinute].Seconds == 0 {
		return tt.Format("06-1-2 15:04")
	}

	return tt.Format("06-1-2 15:04:05")
}
