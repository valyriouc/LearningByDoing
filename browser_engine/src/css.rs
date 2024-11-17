
struct Stylesheet {
    rules: Vec<Rule>
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
}

pub enum Selector {
    Simple(SimpleSelector)
}

pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        let Selector::Simple(ref simple) = *self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        (a, b, c)
    }
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>
}

struct Declaration {
    name: String,
    value: Value,
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

enum Unit {
    Px,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}
