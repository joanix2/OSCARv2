#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Ident(String),     // world, mineral, var, ...
    Number(i32),       // 123
    Float(f32),        // 1.23
    Symbol(String),    // <, >, :, etc.
    Eol,               // fin de ligne
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}
