package operators

import (
	"fmt"
	"saasexpress/tenant-gateway/dag"
	"slices"
)

func JoinStrings(node *dag.Node, input interface{}) (interface{}, error) {
	data, ok := input.([]any)
	if !ok {
		return nil, fmt.Errorf("input is not an array %s", input)
	}
	var aggregatedData []string

	for _, item := range data[0].([]any) {
		aggregatedData = append(aggregatedData, item.(string))
	}
	for _, item := range data[1].([]any) {
		aggregatedData = append(aggregatedData, item.(string))
	}
	slices.Sort(aggregatedData)

	aggregatedData = append(aggregatedData, fmt.Sprintf("[%s] JoinStrings", node.ID))

	return aggregatedData, nil
	// return nil, fmt.Errorf("Oops")
}
