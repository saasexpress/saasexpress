package eip

import (
	"saasexpress/tenant-gateway/dag"
	"saasexpress/tenant-gateway/operators"
)

func InitializeOperators() {
	dag.StepFactory["Append"] = operators.Append
	dag.StepFactory["JoinStrings"] = operators.JoinStrings
}
