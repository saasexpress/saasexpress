package dag

import (
	"context"
	"fmt"
	"reflect"
	"saasexpress/tenant-gateway/internal/pkg"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/reactivex/rxgo/v2"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/codes"
	"go.uber.org/zap"
)

func producer(_ context.CancelFunc, node *Node) rxgo.Observable {
	log := pkg.GetLogger()

	transfer := make(chan rxgo.Item)

	producer := rxgo.FromChannel(transfer, rxgo.WithPublishStrategy())

	producerObservable := rxgo.FromChannel(node.ch)

	producerObservable.DoOnNext(func(in interface{}) {
		message := in.(Message)
		end := message.Context.End

		tracer := message.Context.DagModel.Tracer
		tracerContext := message.Context.DagModel.TracerContext

		_, span := tracer.Start(tracerContext, node.ID)
		defer span.End()

		if node.ID == "node2" {
			span.SetStatus(codes.Error, "things don't look good")
			span.SetAttributes(attribute.Bool("isTrue", true), attribute.String("stringAttr", "hi!"))
		}

		startT := time.Now()

		if err := node.Operator.HandleHook(PreProcess, node, &message); err != nil {
			log.Error("PreProcess error", zap.Error(err))
			end <- rxgo.Of(nil)
			return
		}

		// Schema validation (input)

		if message.Data != nil {

			typ := reflect.TypeOf(message.Data).String()
			log.Debug("PreProcess", zap.String("type", typ))
			if typ != "*bytes.Buffer" && typ != "map[string]interface {}" && typ != "[]interface {}" && typ != "[]string" {
				log.Error("PreProcess error", zap.String("node", node.ID), zap.String("type", typ), zap.Error(fmt.Errorf("strict check on input")))
				end <- rxgo.Of(nil)
				return
			}
		}

		output, err := node.Operator.Process(node, &message)

		if err != nil {
			log.Error("Operator error", zap.Error(err))
			nodeState.With(prometheus.Labels{"node": node.ID, "status": "error"}).Inc()
			end <- rxgo.Of(nil)
			//cancel()
			return
		}

		if output != nil {
			typ := reflect.TypeOf(output).String()
			log.Debug("PostProcess", zap.String("type", typ))
			if typ != "*bytes.Buffer" && typ != "map[string]interface {}" && typ != "[]interface {}" && typ != "[]string" {
				log.Error("PostProcess error", zap.String("node", node.ID), zap.String("type", typ), zap.Error(fmt.Errorf("strict check on output")))
				end <- rxgo.Of(nil)
				return
			}
		}

		// Schema validation (output)

		messageOUT := Message{
			Context:         message.Context,
			Data:            output,
			BroadcastFilter: nil,
		}

		if err := node.Operator.HandleHook(PostProcess, node, &messageOUT); err != nil {
			log.Error("PostProcess error", zap.Error(err))
			end <- rxgo.Of(nil)
			return
		}

		endT := time.Now()

		log.Info("Executed", zap.Int("Duration", int(endT.Sub(startT).Milliseconds())), zap.String("Node", node.ID), zap.String("Op", node.Name))

		nodeState.With(prometheus.Labels{"node": node.ID, "status": "success"}).Inc()

		if len(node.Children) == 0 || output == nil {
			end <- rxgo.Of(messageOUT)
		} else {
			//
			// Usecase: Content-Based-Router
			messageOUT.BroadcastFilter = message.BroadcastFilter

			transfer <- rxgo.Of(messageOUT)
		}
	})

	// If it is a leaf node, send it to itself so that
	// the action can run and then sent to the "end" channel
	if len(node.Children) == 0 {
		producer.DoOnNext(func(item interface{}) {
			node.ch <- rxgo.Of(item)
		})
	}

	return producer
}
