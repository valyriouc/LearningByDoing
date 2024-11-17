use std::collections::HashMap;

// represents the actual content of a element node
pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

// represents the type of node
// todo: get an in-depth look about rust enums
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

// represents one node type in the dom
pub struct Node {

    pub children: Vec<Node>,

    pub node_type: NodeType,
}

// constructor functions
pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(), node_type: NodeType::Text(data)
    }
}

pub fn comment(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Comment(data)
    }
}

pub fn elem(tag_name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs})
    }
}