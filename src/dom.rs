use std::collections::HashMap;
use std::fmt;

// the dom is just a tree of nodes
struct Node {
    children: Vec<Node>,
    node_type: NodeType

}

enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

struct ElementData {
    // represents tag name of element
    tag_name: String,
    // represents things like className, id, etc
    attributes: AttrMap
}

type AttrMap = HashMap<String, String>;


impl Node {
    // create a new node
    fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write(f, "{:?}", self.node_type)
    }
}