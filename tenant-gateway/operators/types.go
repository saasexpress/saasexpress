package operators

import "saasexpress/tenant-gateway/dag"

type OperatorInterface interface {
	dag.NodeOperator
	Spec() *OperatorSpec
}

type BaseOperator struct {
	// OperatorInterface
	// Name string
}

type OperatorSpec struct {
	Name   string
	Icon   string
	Schema interface{}
}
