use std::{collections::HashMap, hash::Hash};

#[path = "../dom/mod.rs"]
mod dom;

pub struct Parser {
    pos: usize, // "usize" is an unsigned integer, similar to "size_t" in C
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!(
                "Expected {:?} at byte {} but it was not found.",
                s, self.pos
            )
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        return c;
    }

    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..'z' | 'A'..'Z' | '0'..'9'))
    }

    // HTML Parser
    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attrs();
        self.expect(">");

        let children = self.parse_nodes();

        self.expect("</");
        self.expect(&tag_name);
        self.expect(">");

        return dom::elem(tag_name, attrs, children);
    }

    // Single <... key="value" /> pair
    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        return value;
    }

    fn parse_attrs(&mut self) -> dom::AttrMap {
        let mut attrs = HashMap::new();

        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attrs.insert(name, value);
        }
        return attrs;
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        return nodes;
    }

    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser {
            pos: 0,
            input: source,
        }
        .parse_nodes();

        if nodes.len() == 1 {
            return nodes.remove(0);
        } else {
            return dom::elem("html".to_string(), HashMap::new(), nodes);
        }
    }
}
