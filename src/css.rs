use std::default::Default;
use std::fmt;

pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(PartialEq, Eq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

#[derive(PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

pub struct Declaration {
    pub property: String,
    pub value: Value,
}

pub enum Value {
    Color(Color),
    Length(f32, Unit),
    Other(String),
}

pub enum Unit {
    // https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units#lengths
    Px,
    Em,
    Rem,
    Vh,
    Vw,
    Vmin,
    Vmax,
}

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules }
    }
}

impl Default for Stylesheet {
    // https://doc.rust-lang.org/std/default/trait.Default.html
    fn default() -> Self {
        Stylesheet { rules: Vec::new() }
    }
}

impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_res = String::new();
        for rule in &self.rules {
            if rule_res.len() > 0 {
                rule_res.push_str("\n\n");
            }

            rule_res.push_str(&format!("{:?}", rule));
        }

        write!(f, "{}", rule_res)
    }
}

impl Rule {
    pub fn new(selectors: Vec<Selector>, declarations: Vec<Declaration>) -> Rule {
        Rule {
            selectors,
            declarations,
        }
    }
}

impl Default for Rule {
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
        }
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sel_res = String::new();
        let mut dec_res = String::new();

        let tab = "    ";

        for selector in &self.selectors {
            if sel_res.len() > 0 {
                sel_res.push_str(", ");
            }

            sel_res.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            dec_res.push_str(tab);
            dec_res.push_str(&format!("{:?}", declaration));
            dec_res.push('\n');
        }

        write!(f, "{} {{\n{}}}", sel_res, dec_res)
    }
}

impl Selector {
    pub fn new(simple: Vec<SimpleSelector>, combinators: Vec<char>) -> Selector {
        Selector {
            simple,
            combinators,
        }
    }
}

impl Default for Selector {
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
        }
    }
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }

        write!(f, "{}", result)
    }
}

impl SimpleSelector {
    pub fn new(
        tag_name: Option<String>,
        id: Option<String>,
        classes: Vec<String>,
    ) -> SimpleSelector {
        SimpleSelector {
            tag_name,
            id,
            classes,
        }
    }
}

impl Default for SimpleSelector {
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
        }
    }
}

impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        match self.tag_name {
            Some(ref t) => result.push_str(t),
            None => {}
        }

        match self.id {
            Some(ref id) => {
                result.push('#');
                result.push_str(id);
            }
            None => {}
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }

        write!(f, "{}", result)
    }
}

impl Declaration {
    pub fn new(property: String, value: Value) -> Declaration {
        Declaration { property, value }
    }
}

impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Length(l, _) => write!(f, "{:?}", l),
            Value::Other(ref o) => write!(f, "{:?}", o),
        }
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(1.0, 1.0, 1.0, 1.0)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
    }
}
