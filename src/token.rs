#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    Plus(char),
    Minus(char),
    Multiply(char),
    Divide(char),
    Number(String),
}
