package beat

import (
	"context"
	"fmt"
	"time"

	"github.com/Geapefurit/kline-back/common/kptype"
	"github.com/Geapefurit/kline-back/proto/kline"
	basetype "github.com/Geapefurit/kline-back/proto/kline/basetype/v1"
	kpointproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kpoint"
	kpriceproto "github.com/Geapefurit/kline-back/proto/kline/zeus/v1/kprice"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kpoint"
	"github.com/Geapefurit/kline-back/zeus/pkg/mw/v1/kprice"
	"github.com/NpoolPlatform/go-service-framework/pkg/logger"
	"github.com/NpoolPlatform/libent-cruder/pkg/cruder"
)

// 1 second
var updateGraceTime = uint32(1)

type SamplingKPointTask struct {
	interval  uint32
	kpType    basetype.KPointType
	closeChan chan struct{}
}

func GetSamplingKPointTask(kpType basetype.KPointType) (*SamplingKPointTask, error) {
	task := SamplingKPointTask{kpType: kpType}
	info, ok := kptype.KPointTypeInfos[kpType]
	if !ok {
		return nil, fmt.Errorf("invalid kptype")
	}
	task.interval = info.Seconds
	return &task, nil
}

func (task *SamplingKPointTask) createInitialKPoint(ctx context.Context) error {
	kpH, err := kprice.NewHandler(ctx,
		kprice.WithConds(
			&kpriceproto.Conds{},
		),
		kprice.WithOffset(0),
		kprice.WithLimit(1),
	)

	if err != nil {
		return err
	}

	earlistKPs, _, err := kpH.GetEarlistKPrices(ctx)
	if err != nil {
		return err
	}
	if len(earlistKPs) == 0 {
		return nil
	}

	latestKPs, _, err := kpH.GetLatestKPrices(ctx)
	if err != nil {
		return err
	}
	if len(latestKPs) == 0 {
		return nil
	}

	startTime := earlistKPs[0].Timestamp - earlistKPs[0].Timestamp%task.interval
	endTime := startTime + task.interval

	if endTime+updateGraceTime > latestKPs[0].Timestamp {
		return nil
	}

	kpReqs, err := kpH.GetKPointFromKPrice(ctx, startTime, endTime, task.kpType)
	if err != nil {
		return err
	}

	mulKPH, err := kpoint.NewMultiCreateHandler(ctx, kpReqs, true)
	if err != nil {
		return err
	}

	if err := mulKPH.CreateKPoints(ctx); err != nil {
		return err
	}

	return nil
}

func (task *SamplingKPointTask) createKPoints(ctx context.Context, startTime uint32) error {
	now := uint32(time.Now().Unix())
	if startTime+task.interval+updateGraceTime >= now {
		return nil
	}

	kpH, err := kprice.NewHandler(ctx)
	if err != nil {
		return err
	}

	_startTime := startTime
	timePeriodsLen := (now - startTime) / task.interval
	for i := uint32(0); i < timePeriodsLen; i++ {
		kpReqs, err := kpH.GetKPointFromKPrice(ctx, _startTime, _startTime+task.interval, task.kpType)
		if err != nil {
			return err
		}
		_startTime += task.interval

		if len(kpReqs) == 0 {
			continue
		}

		mulKPH, err := kpoint.NewMultiCreateHandler(ctx, kpReqs, true)
		if err != nil {
			return err
		}
		if err := mulKPH.CreateKPoints(ctx); err != nil {
			return err
		}
	}

	return nil
}

func (task *SamplingKPointTask) samplingAndStore(ctx context.Context) error {
	kpH, err := kpoint.NewHandler(ctx,
		kpoint.WithConds(
			&kpointproto.Conds{
				KPointType: &kline.Uint32Val{
					Op:    cruder.EQ,
					Value: uint32(task.kpType),
				},
				EndAt: &kline.Uint32Val{
					Op:    cruder.LT,
					Value: uint32(time.Now().Unix()),
				},
			},
		),
		kpoint.WithLimit(1),
		kpoint.WithOffset(0))
	if err != nil {
		return err
	}
	kpoints, _, err := kpH.GetLatestKPoints(ctx)
	if err != nil {
		return err
	}
	if len(kpoints) == 0 {
		if err := task.createInitialKPoint(ctx); err != nil {
			return err
		}
		return nil
	}

	return task.createKPoints(ctx, kpoints[0].EndTime)
}

func (task *SamplingKPointTask) StartSampling(ctx context.Context, seconds uint32) {
	task.closeChan = make(chan struct{})
	for {
		select {
		case <-time.NewTicker(time.Second * time.Duration(seconds)).C:
			go func() {
				err := task.samplingAndStore(ctx)
				if err != nil {
					logger.Sugar().Error(err)
				}
			}()
		case <-ctx.Done():
			return
		case <-task.closeChan:
			return
		}
	}
}

func (task *SamplingKPointTask) Close() {
	close(task.closeChan)
}

func RunSamplingKPoint(ctx context.Context) {
	for _kptype, interval := range kptype.KPTypeSampleSecond {
		task, err := GetSamplingKPointTask(_kptype)
		if err != nil {
			panic(err)
		}
		go task.StartSampling(ctx, interval)
		// let tasks not be triggered at the same second
		time.Sleep(time.Second)
	}
}
