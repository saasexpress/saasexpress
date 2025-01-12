package operators

import (
	"fmt"
	"saasexpress/tenant-gateway/dag"
)

func Append(node *dag.Node, input interface{}) (interface{}, error) {
	fmt.Println("APPEND", input)
	data, ok := input.([]any)
	if !ok {
		return nil, fmt.Errorf("(Append) input is not an array")
	}
	var appendedData []any
	appendedData = append(appendedData, data...)
	appendedData = append(appendedData, fmt.Sprintf("[%s] Append line", node.ID))

	return appendedData, nil
}
