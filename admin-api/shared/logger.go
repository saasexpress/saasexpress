package shared

import (
	"sync"

	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

var (
	logger *zap.Logger
	once   sync.Once
)

// GetLogger returns the singleton instance of the zap logger.
func GetLogger() *zap.Logger {
	once.Do(func() {
		var err error
		//logger, err = zap.NewProduction()
		config := zap.NewDevelopmentConfig()
		config.Level.SetLevel(zap.InfoLevel)
		config.EncoderConfig.EncodeLevel = zapcore.CapitalColorLevelEncoder
		logger, err = config.Build()

		if err != nil {
			panic(err)
		}
		//defer logger.Sync()
	})

	return logger
}
