package dag

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	nodeState = promauto.NewCounterVec(prometheus.CounterOpts{
		Name: "node_status",
		Help: "Node status",
	}, []string{"node", "status"})
)
