enum NodeType {
    Program,
    Operation,
    Constant,
}

struct Node {
    node_type: NodeType,
}