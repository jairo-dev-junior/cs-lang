use crate::token::Token;

const SYMBOLS: [char; 4] = ['+', '-', '*', '/'];

pub struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }

    fn next_token(&mut self) -> Result<char, ()> {
        if self.position >= self.input.len() {
            return Result::Err(());
        }
        let ch = self.input.chars().nth(self.position).ok_or(())?;
        self.position += 1;
        Result::Ok(ch)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut token = String::new();
        let mut number = String::new();

        while let Ok(ch) = self.next_token() {
            if SYMBOLS.contains(&ch) || ch.is_numeric() || ' ' == ch {
                if !token.is_empty() {
                    tokens.push(Token::Identifier(token.clone()));
                    token.clear();
                }
                if ch.is_numeric() {
                    number.push(ch);
                    continue;
                }
                if !number.is_empty() {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                }
                if ch == '+' {
                    tokens.push(Token::Plus(ch));
                }
                if ch == '*' {
                    tokens.push(Token::Multiply(ch));
                }
                if ch == '/' {
                    tokens.push(Token::Divide(ch));
                }
                if ch == '-' {
                    tokens.push(Token::Minus(ch));
                }
            } else {
                token.push(ch);

                if !number.is_empty() {
                    tokens.push(Token::Number(number.clone()));
                    number.clear();
                }
            }
        }

        if !token.is_empty() {
            tokens.push(Token::Identifier(token));
        }

        if !number.is_empty() {
            tokens.push(Token::Number(number));
        }

        tokens
    }
}
