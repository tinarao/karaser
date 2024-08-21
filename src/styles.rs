use crate::css::{Selector, Stylesheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;
use std::{fmt, str};

type PropertyMap<'a> = HashMap<&'a str, &'a Value>;

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap<'a>,
    pub children: Vec<StyledNode<'a>>,
}

pub enum Display {
    Block,
    Inline,
    InlineBlock,
    None,
}

impl<'a> StyledNode<'a> {
    pub fn new(node: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
        let mut style_children = Vec::new();

        for child in &node.children {
            match child.node_type {
                NodeType::Element(_) => style_children.push(StyledNode::new(&child, stylesheet)),
                _ => {}
            }
        }

        StyledNode {
            node,
            styles: match node.node_type {
                NodeType::Element(ref e) => StyledNode::get_styles(e, stylesheet),
                _ => PropertyMap::new(),
            },
            children: style_children,
        }
    }

    fn get_styles(el: &'a ElementData, stylesheet: &'a Stylesheet) -> PropertyMap<'a> {
        let mut styles = PropertyMap::new();

        for rule in &stylesheet.rules {
            for selector in &rule.selectors {
                if is_selector_matches(el, &selector) {
                    for dclr in &rule.declarations {
                        styles.insert(&dclr.property, &dclr.value);
                    }
                    break;
                }
            }
        }

        styles
    }

    pub fn value(&self, name: &str) -> Option<&&Value> {
        self.styles.get(name)
    }

    pub fn get_display(&self) -> Display {
        match self.value("display") {
            Some(s) => match **s {
                Value::Other(ref v) => match v.as_ref() {
                    "block" => Display::Block,
                    "none" => Display::None,
                    "inline" => Display::Inline,
                    "inline-block" => Display::InlineBlock,
                    _ => Display::Inline,
                },
                _ => Display::Inline,
            },
            None => Display::None,
        }
    }

    pub fn num_or(&self, name: &str, def: f32) -> f32 {
        match self.value(name) {
            Some(v) => match **v {
                Value::Length(n, _) => n,
                _ => def,
            },
            None => def,
        }
    }
}

impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.node, self.styles)
    }
}

fn is_selector_matches(el: &ElementData, sel: &Selector) -> bool {
    for simple in &sel.simple {
        let mut matches = true;

        match simple.tag_name {
            Some(ref t) => {
                if *t != el.tag_name {
                    continue;
                }
            }

            None => {}
        };

        match el.get_id() {
            Some(i) => match simple.id {
                Some(ref id) => {
                    if *i != *id {
                        continue;
                    }
                }
                None => {}
            },
            None => match simple.id {
                Some(_) => {
                    continue;
                }
                _ => {}
            },
        }

        let el_classes = el.get_classes();
        for class in &simple.classes {
            matches = matches & el_classes.contains::<str>(class);
        }

        if matches {
            return true;
        }
    }

    false
}

pub fn pretty_print(node: &StyledNode, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();
    println!("{}{:?}", indent, node);

    for child in node.children.iter() {
        pretty_print(&child, indent_size + 2);
    }
}
