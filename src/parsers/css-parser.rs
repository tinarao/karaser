use std::iter::Peekable;
use std::str::Chars;

use crate::css::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Unit, Value};

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CssParser<'a> {
    pub fn new(full_css: &str) -> CssParser {
        CssParser {
            chars: full_css.chars().peekable(),
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::default();

        while self.chars.peek().is_some() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            stylesheet.rules.push(rule);
        }

        stylesheet
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_selector();

            if selector != Selector::default() {
                selectors.push(selector);
            }

            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == ',') {
                self.chars.next();
            }
        }
        self.chars.next();

        selectors
    }

    fn parse_selector(&mut self) -> Selector {
        let mut simple_sel = SimpleSelector::default();
        let mut selector = Selector::default();

        self.consume_while(char::is_whitespace);

        simple_sel.tag_name = match self.chars.peek() {
            Some(&c) if is_valid_start_ident(c) => Some(self.parse_identifier()),
            _ => None,
        };

        let mut multiple_ids = false;
        while self
            .chars
            .peek()
            .map_or(false, |c| *c == ',' && *c != '{' && !(*c).is_whitespace())
        {
            match self.chars.peek() {
                Some(&c) if c == '#' => {
                    self.chars.next();

                    if simple_sel.id.is_some() || multiple_ids {
                        simple_sel.id = None;
                        multiple_ids = true;
                        self.parse_id();
                    } else {
                        simple_sel.id = self.parse_id();
                    }
                }
                Some(&c) if c == '.' => {
                    self.chars.next();
                    let class_name = self.parse_identifier();

                    if class_name != String::from("") {
                        simple_sel.classes.push(class_name);
                    }
                }
                _ => {
                    self.consume_while(|c| c != ',' && c != '{');
                }
            }
        }

        if simple_sel != SimpleSelector::default() {
            selector.simple.push(simple_sel);
        }

        selector
    }

    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();

        match self.chars.peek() {
            Some(&c) => {
                if is_valid_start_ident(c) {
                    ident.push_str(&self.consume_while(is_valid_ident))
                }
            }
            None => {}
        }

        ident.to_lowercase()
    }

    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_identifier()[..] {
            "" => None,
            s @ _ => Some(s.to_string()),
            // @
            // https://doc.rust-lang.org/reference/patterns.html#identifier-patterns
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut decls = Vec::<Declaration>::new();

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(char::is_whitespace);
            let property = self.consume_while(|x| x != ':').to_lowercase();

            self.chars.next();
            self.consume_while(char::is_whitespace);

            let val = self
                .consume_while(|x| x != ';' && x != '\n' && x != '{')
                .to_lowercase();

            let value_enum = match property.as_ref() {
                "background-color" | "border-color" | "color" => {
                    Value::Color(translate_color(&val))
                }
                "margin"
                | "padding"
                | "margin-top"
                | "margin-left"
                | "margin-right"
                | "margin-bottom"
                | "padding-top"
                | "padding-left"
                | "padding-right"
                | "padding-bottom"
                | "border-top-width"
                | "border-left-width"
                | "border-right-width"
                | "border-bottom-width"
                | "width"
                | "height" => translate_length(&val),
                _ => Value::Other(val),
            };

            let declaration = Declaration::new(property, value_enum);

            if self.chars.peek().map_or(false, |c| *c == ';') {
                decls.push(declaration);
                self.chars.next();
            } else {
                self.consume_while(char::is_whitespace);
                if self.chars.peek().map_or(false, |c| *c == '}') {
                    decls.push(declaration);
                }
            }
            self.consume_while(char::is_whitespace);
        }
        self.chars.next();

        decls
    }

    //

    fn consume_while<F>(&mut self, condition: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

fn translate_color(color: &str) -> Color {
    // Все цвета: https://colorscheme.ru/html-colors.html
    // TODO: Дописать все цвета. Сюда напишу основные.
    // TODO: Дописать все форматы. Пока будут только текстовые идентификаторы.

    // занятие блять на недельку другую

    return match color {
        "black" => Color::new(0.0, 0.0, 0.0, 1.0),
        "white" => Color::new(1.0, 1.0, 1.0, 1.0),
        "red" => Color::new(1.0, 0.0, 0.0, 1.0),
        "green" => Color::new(0.0, 1.0, 0.0, 1.0),
        "blue" => Color::new(0.0, 0.0, 1.0, 1.0),
        _ => Color::new(0.0, 0.0, 0.0, 1.0),
    };
}

fn translate_length(length: &str) -> Value {
    let mut num_str = String::new();
    let mut unit = String::new();
    let mut parsing_num = true;

    for ch in length.chars() {
        if ch.is_numeric() && parsing_num {
            num_str.push(ch);
        } else {
            unit.push(ch);
            parsing_num = false;
        }
    }

    let num: f32 = num_str.parse().unwrap_or(0.0);

    match unit.as_ref() {
        "px" => Value::Length(num, Unit::Px),
        "em" => Value::Length(num, Unit::Em),
        "rem" => Value::Length(num, Unit::Rem),
        "vh" => Value::Length(num, Unit::Vh),
        "vw" => Value::Length(num, Unit::Vw),
        "vmin" => Value::Length(num, Unit::Vmin),
        "vmax" => Value::Length(num, Unit::Vmax),

        _ => Value::Length(num, Unit::Px),
    }
}

fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}
