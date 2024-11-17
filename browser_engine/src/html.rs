﻿use std::collections::HashMap;
use crate::dom;
use crate::css;

pub struct Parser {
    pos: usize,
    input: String
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
            panic!("Expected {:?} at byte {} but it was not found", s, self.pos)
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    // return the current character, and advance self.pos to the next character.
    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    // consume characters until `test` returns false.
    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    // parse a tag or attribute name
    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<!--") {
            self.parse_comment()
        }
        else if self.starts_with("<") {
            self.parse_element()
        }
        else {
            self.parse_text()
        }
    }

    fn parse_comment(&mut self) -> dom::Node {
        self.expect("<!--");
        self.consume_whitespace();
        let node = dom::comment(self.consume_while(|c| c != '-'));
        self.consume_whitespace();
        self.expect("-->");
        node
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
        nodes
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        let children = self.parse_nodes();

        self.expect("</");
        self.expect(&tag_name);
        self.expect(">");

        dom::elem(tag_name, attrs, children)
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        value
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_simple_selector(&mut self) -> css::SimpleSelector {
        let mut selector = css::SimpleSelector {
            tag_name: None,
            id: None,
            class: Vec::new()
        };

        while !self.eof() {
            match self.next_char() {
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                }
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '*' => {
                    self.consume_char();
                }
                c if Self::valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }
    }

    fn valid_identifier_char(c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_')
    }

    fn parse_rule(&mut self) -> css::Rule {
        css::Rule {
            selectors: self.parse_selectors(),
            declarations: self.parse_declarations()
        }
    }

    // todo: finish the css parser
    // todo: writing the parsers from new on
    fn parse_selectors(&mut self) -> Vec<css::Selector> {
        let mut selectors = Vec::new();
        loop {
            selectors.push(css::Selector::Simple(self.parse_simple_selector()));
            self.consume_whitespace();
            match self.next_char() {
                ',' => { self.consume_char(); self.consume_whitespace(); }
                '{' => break,
                c => panic!("Unexpected character {} in selector list", c)
            }
        }
        selectors.sort_by(|a,b| b.specificity().cmp(&a.specificity()));
        selectors
    }

    pub fn parse(source: String) -> dom::Node {
        let mut nodes = Parser { pos: 0, input: source}.parse_nodes();

        if nodes.len() == 1 {
            nodes.remove(0)
        } else {
            dom::elem("html".to_string(), HashMap::new(), nodes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_handle_simple_html() {
        // Add your test cases here
        let html = r#"
        <html>
            <body>
                <h1>Title</h1>
                <div id="main" class="test">
                    <p>Hello <em>world</em>!</p>
                </div>
            </body>
        </html>
        "#;
    }
}