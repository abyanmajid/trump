// lyin kamala
// liars all over the place
// you're a liar

use std::collections::HashMap;
use crate::token::{Token, TokenType};
use crate::lexer::Lexer;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum PrecedenceType {
    Lowest,      // Lowest precedence (e.g., for non-operators)
    Equals,      // == or !=
    LessGreater, // < or >
    Sum,         // + or -
    Product,     // * or /
    Power,       // ^
    Prefix,      // -x or !x
    Call,        // Function calls, e.g., add(1, 2)
    Index,       // Array indexing, e.g., arr[0]
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    prefix_parse_fns: HashMap<TokenType, fn(&mut Parser) -> Option<Expression>>,
    infix_parse_fns: HashMap<TokenType, fn(&mut Parser, Expression) -> Option<Expression>>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            errors: Vec::new(),
            current_token: None,
            peek_token: None,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser._next_token();
        parser._next_token();

        parser
    }

    fn _next_token(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = Some(self.lexer.next_token());
    }

    fn _peek_token_is(&self, token_type: &TokenType) -> bool {
        if let Some(ref peek_token) = self.peek_token {
            &peek_token.token_type == token_type
        } else {
            false
        }
    }

    fn _expect_peek(&mut self, token_type: TokenType) -> bool {
        if self._peek_token_is(&token_type) {
            self._next_token();
            true
        } else {
            self._peek_error(&token_type);
            false
        }
    }

    fn _peek_error(&mut self, token_type: &TokenType) {
        if let Some(ref peek_token) = self.peek_token {
            let error = format!(
                "Expected next token to be {:?}, got {:?} instead",
                token_type, peek_token.token_type
            );
            self.errors.push(error);
        }
    }
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Integer(i64),
    Float(f64),
    Infix {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}