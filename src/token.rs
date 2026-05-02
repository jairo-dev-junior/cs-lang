#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Plus(),
    Minus(),
    Multiply(),
    Divide(),
    Number(String),
    Equal(),
}
