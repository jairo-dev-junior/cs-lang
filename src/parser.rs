use std::ops::Index;

use crate::ast::Ast;
use crate::token::{self, Token};

struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    fn parse(&self) -> Result<Ast, ()> {
        let mut result: Result<Ast, ()> = Result::Err(());
        for (index, token) in self.tokens.iter().enumerate() {
            if let Ok(ast) = self.parse_spray(token, index) {
                result = Ok(ast);
            }
        }

        return result;
    }

    fn parse_spray(&self, token: &Token, index: usize) -> Result<Ast, ()> {
        if token == &Token::Identifier("spray".to_string()) {
            return match self.tokens.get(index + 1) {
                Some(Token::Identifier(value)) => Ok(Ast::Spray(value.clone())),
                Some(Token::Number(value)) => Ok(Ast::Spray(value.clone())),
                _ => Err(()),
            };
        }

        Err(())
    }
}
