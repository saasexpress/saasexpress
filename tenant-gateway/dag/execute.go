package dag

import (
	"fmt"
	"reflect"
	"saasexpress/tenant-gateway/internal/pkg"

	"github.com/reactivex/rxgo/v2"
	"go.uber.org/zap"
)

func (dagctx *DAGContext) Execute(in Message) (interface{}, error) {
	log := pkg.GetLogger()

	startNode := dagctx.DagModel.Nodes[dagctx.StartID]
	totalLeafs := dagctx.DagModel.CountLeafs()

	end := dagctx.End
	endObservable := rxgo.FromChannel(end)

	startNode.ch <- rxgo.Of(in)

	count := 0
	var results []interface{}

	tracer := in.Context.DagModel.Tracer
	tracerContext := in.Context.DagModel.TracerContext

	_, span := tracer.Start(tracerContext, "Execute")
	defer span.End()

	for item := range endObservable.Observe() {
		message := item.V.(Message)

		log.Debug("End Observed", zap.Int("Count", count), zap.Int("Expecting", totalLeafs))

		count++

		if message.Data != nil {
			typ := reflect.TypeOf(message.Data).String()
			log.Debug("Execute", zap.String("type", typ))
		}

		if message.Data != nil {
			results = append(results, message.Data)
		} else {
			log.Debug("Message Value null, ignore")
		}

		// all leafs need to respond one way or another
		// otherwise we will timeout waiting for response
		if count == totalLeafs {
			close(end)
			log.Debug("Length = ", zap.Int("len", len(results)))
			if len(results) == 0 {
				log.Error("Did not get a response - returning error")
				return nil, fmt.Errorf("did not get a response")
			}
			if len(results) == 1 {
				return results[0], nil
			} else {
				return results, nil
			}
		}
	}

	return nil, fmt.Errorf("did not get a response")
}
