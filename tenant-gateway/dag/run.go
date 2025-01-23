package dag

import (
	"bytes"
	"fmt"
	"io"
	"reflect"
	"saasexpress/tenant-gateway/internal/pkg"

	"github.com/reactivex/rxgo/v2"
	"go.uber.org/zap"
)

func (dagContext *DAGContext) RunDAG(inputBuffer io.Reader) (*bytes.Buffer, error) {
	log := pkg.GetLogger()

	message := Message{
		Context: dagContext,
		Data:    inputBuffer,
	}

	end := make(chan rxgo.Item)
	dagContext.End = end

	output, err := dagContext.Execute(message)

	if err != nil {
		log.Error("error executing DAG", zap.Error(err))
		return nil, fmt.Errorf("error executing DAG")
	}

	if output == nil {
		log.Error("error no output from DAG")
		return nil, fmt.Errorf("error producing output from DAG")
	}

	log.Debug("Processing complete returning", zap.String("type", reflect.TypeOf(output).String()))
	return output.(*bytes.Buffer), nil
}
