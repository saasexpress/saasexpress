package dag

var StepFactory = map[string]NodeFunction{}

// func Append(node *Node, input interface{}) (interface{}, error) {
// 	fmt.Println("APPEND", input)
// 	data, ok := input.([]any)
// 	if !ok {
// 		return nil, fmt.Errorf("(Append) input is not an array")
// 	}
// 	var appendedData []any
// 	appendedData = append(appendedData, data...)
// 	appendedData = append(appendedData, fmt.Sprintf("[%s] Append line", node.ID))

// 	return appendedData, nil
// }

// func AggregateStrings(node *Node, input interface{}) (interface{}, error) {
// 	data, ok := input.([]any)
// 	if !ok {
// 		return nil, fmt.Errorf("input is not an array %s", input)
// 	}
// 	var aggregatedData []string

// 	for _, item := range data[0].([]any) {
// 		aggregatedData = append(aggregatedData, item.(string))
// 	}
// 	for _, item := range data[1].([]any) {
// 		aggregatedData = append(aggregatedData, item.(string))
// 	}
// 	slices.Sort(aggregatedData)

// 	aggregatedData = append(aggregatedData, fmt.Sprintf("[%s] AggregateStrings", node.ID))

// 	return aggregatedData, nil
// 	// return nil, fmt.Errorf("Oops")
// }

// func Translate(node *Node, input interface{}) (interface{}, error) {
// 	data, ok := input.(map[string]interface{})
// 	if !ok {
// 		return nil, fmt.Errorf("(Translate) input is not a map")
// 	}
// 	translatedData := map[string]interface{}{
// 		"name": "Dr. " + data["name"].(string),
// 		"role": "Translate",
// 	}
// 	return translatedData, nil
// }

// func Split(node *Node, input interface{}) (interface{}, error) {
// 	data, ok := input.(map[string]interface{})
// 	if !ok {
// 		return nil, fmt.Errorf("(Split) input is not a map %s", input)
// 	}

// 	splitData := map[string]interface{}{
// 		"a":    data["name"].(string),
// 		"b":    data["name"].(string),
// 		"role": "Split",
// 	}
// 	return splitData, nil
// }

// func Aggregate(node *Node, input interface{}) (interface{}, error) {
// 	data, ok := input.([]interface{})
// 	if !ok {
// 		return nil, fmt.Errorf("input is not an array %s", input)
// 	}
// 	aggregatedData := map[string]interface{}{
// 		"role": "Aggregate",
// 	}
// 	for i, item := range data {
// 		aggregatedData["ID"+strconv.Itoa(i)] = item
// 	}
// 	return interface{}(aggregatedData), nil
// }

// func AggregateOld(node *Node, input interface{}) (interface{}, error) {
// 	data, ok := input.(map[string]interface{})
// 	if !ok {
// 		return nil, fmt.Errorf("input is not a map %s", input)
// 	}
// 	dataA, ok := data["a"].(map[string]interface{})
// 	if !ok {
// 		return nil, fmt.Errorf("data[a] is not a map")
// 	}
// 	dataB, ok := data["b"].(map[string]interface{})
// 	if !ok {
// 		return nil, fmt.Errorf("data[a] is not a map")
// 	}
// 	aggregatedData := map[string]interface{}{
// 		"name":      dataA["name"].(string) + " and " + dataB["name"].(string),
// 		"a":         dataA,
// 		"b":         dataA,
// 		"orig_role": data["role"],
// 		"role":      "Aggregate",
// 	}
// 	return interface{}(aggregatedData), nil
// }

// func Filter(node *Node, input interface{}) (interface{}, error) {
// 	filteredData := map[string]interface{}{
// 		"role": "Filter",
// 		"data": input,
// 	}
// 	return filteredData, nil
// }

// func Enrich(node *Node, input interface{}) (interface{}, error) {
// 	enrichedData := map[string]interface{}{
// 		"role": "Enrich",
// 		"data": input,
// 	}
// 	return enrichedData, nil
// }
