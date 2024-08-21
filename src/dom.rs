use core::fmt;
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(PartialEq, Eq, Clone)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(PartialEq, Eq, Clone)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

impl ElementData {
    pub fn new(tag_name: String, attributes: AttrMap) -> ElementData {
        ElementData {
            tag_name,
            attributes,
        }
    }

    fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new(node_type: NodeType, children: Vec<Node>) -> Node {
        Node {
            node_type,
            children,
        }
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t) | NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attr_str = String::new();

        for (attr, val) in self.attributes.iter() {
            attr_str.push_str(&format!(" {}=\"{}\" ", attr, val));
        }

        write!(f, "<{},{}>", self.tag_name, attr_str)
    }
}

fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
    }

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}</{}>", indent, e.tag_name),
        _ => {}
    }
}
