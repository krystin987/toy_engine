//! Basic DOM data structures.

use std::collections::{HashMap,HashSet};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node { // struct - with 2 fields
    // data common to all nodes:
    pub children: Vec<Node>,

    // data specific to each node type:
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType { // enumberable
    Element(ElementData),
    Text(String),
}

#[derive(Debug)]
pub struct ElementData { // constructor
    pub tag_name: String,
    pub attributes: AttrMap,
}

pub fn text(data: String) -> Node { // method
    Node { children: vec![], node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node { // method
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

// Element methods
impl ElementData { // implementation
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classlist) => classlist.split(' ').collect(),
            None => HashSet::new()
        }
    }
}
