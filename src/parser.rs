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
        let parser = Parser {
            lexer,
            errors: Vec::new(),
            current_token: None,
            peek_token: None,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser
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