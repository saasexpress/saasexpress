package pkg

import (
	"github.com/danielgtaylor/mexpr"
	"go.uber.org/zap"
)

func Mexpr(expr string, in any) any {
	log := GetLogger()

	result, err := mexpr.Eval(expr, in)
	if err != nil {
		log.Error("Parsing", zap.String("expr", expr), zap.Error(err))
		return nil
	}
	return result
}

func MexprString(expr string, in any, defValue string) string {
	log := GetLogger()

	result, err := mexpr.Eval(expr, in)
	if err != nil {
		log.Error("Parsing", zap.String("expr", expr), zap.Error(err))
		return defValue
	}
	return result.(string)
}
