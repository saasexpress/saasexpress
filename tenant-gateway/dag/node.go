package dag

func (node *Node) HasParents() bool {
	return len(node.Parents) > 0
}

func (node *Node) HasChildren() bool {
	return len(node.Children) > 0
}
