// lyin kamala
// liars all over the place
// you're a liar

use crate::ast::{
    Expression, ExpressionStatement, FloatLiteral, InfixExpression, IntegerLiteral, Node, Program,
    Statement,
};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::collections::HashMap;

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
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    errors: Vec<String>,
    current_token: Option<Token>,
    peek_token: Option<Token>,
    prefix_parse_fns: HashMap<TokenType, fn(&mut Parser<'a>) -> Option<Expression>>,
    infix_parse_fns: HashMap<TokenType, fn(&mut Parser<'a>, Expression) -> Option<Expression>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            errors: Vec::new(),
            current_token: None,
            peek_token: None,
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser._register_prefix(TokenType::Integer, Parser::_parse_int_literal);
        parser._register_prefix(TokenType::Float, Parser::_parse_float_literal);
        parser._register_prefix(TokenType::LeftParen, Parser::_parse_grouped_expression);

        parser._register_infix(TokenType::Plus, Parser::_parse_infix_expression);
        parser._register_infix(TokenType::Minus, Parser::_parse_infix_expression);
        parser._register_infix(TokenType::Slash, Parser::_parse_infix_expression);
        parser._register_infix(TokenType::Asterisk, Parser::_parse_infix_expression);
        parser._register_infix(TokenType::Pow, Parser::_parse_infix_expression);
        parser._register_infix(TokenType::Modulus, Parser::_parse_infix_expression);

        parser._next_token();
        parser._next_token();

        parser
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program::new();

        while let Some(ref current_token) = self.current_token {
            if current_token.token_type == TokenType::Eof {
                break;
            }

            if let Some(stmt) = self._parse_statement() {
                program.add_statement(Node::Statement(stmt));
            }

            self._next_token();
        }

        program
    }

    fn _parse_statement(&mut self) -> Option<Statement> {
        match self.current_token {
            Some(ref token) => match token.token_type {
                TokenType::Identifier
                | TokenType::Integer
                | TokenType::Float
                | TokenType::LeftParen => Some(Statement::ExpressionStatement(
                    self._parse_expression_statement(),
                )),
                _ => {
                    self.errors
                        .push(format!("Unexpected token: {:?}", token.token_type));
                    None
                }
            },
            None => None,
        }
    }

    fn _parse_expression_statement(&mut self) -> ExpressionStatement {
        let expr = self._parse_expression(PrecedenceType::Lowest);

        if self._peek_token_is(&TokenType::SemiColon) {
            self._next_token();
        }

        ExpressionStatement::new(Node::Expression(expr))
    }

    fn _parse_expression(&mut self, precedence: PrecedenceType) -> Expression {
        let prefix_fn = {
            let current_token_type = &self.current_token.as_ref().unwrap().token_type;
            self.prefix_parse_fns.get(current_token_type).cloned()
        };

        if prefix_fn.is_none() {
            self._no_prefix_parse_in_error(self.current_token.as_ref().unwrap().token_type);
            return Expression::IntegerLiteral(IntegerLiteral::new(0));
        }

        let mut left_expr = prefix_fn.unwrap()(self).unwrap();

        while !self._peek_token_is(&TokenType::SemiColon) && precedence < self._peek_precedence() {
            let infix_fn = {
                let peek_token_type = &self.peek_token.as_ref().unwrap().token_type;
                self.infix_parse_fns.get(peek_token_type).cloned()
            };

            if infix_fn.is_none() {
                return left_expr;
            }

            self._next_token();

            left_expr = infix_fn.unwrap()(self, left_expr).unwrap();
        }

        left_expr
    }

    fn _parse_int_literal(&mut self) -> Option<Expression> {
        let value = self
            .current_token
            .as_ref()
            .unwrap()
            .lexeme
            .parse::<i64>()
            .unwrap();
        Some(Expression::IntegerLiteral(IntegerLiteral::new(value)))
    }

    fn _parse_float_literal(&mut self) -> Option<Expression> {
        let value = self
            .current_token
            .as_ref()
            .unwrap()
            .lexeme
            .parse::<f64>()
            .unwrap();
        Some(Expression::FloatLiteral(FloatLiteral::new(value)))
    }

    fn _parse_grouped_expression(&mut self) -> Option<Expression> {
        self._next_token();
        let expr = self._parse_expression(PrecedenceType::Lowest);

        if !self._expect_peek(TokenType::RightParen) {
            return None;
        }

        Some(expr)
    }

    fn _parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.as_ref().unwrap().lexeme.clone();
        let precedence = self._current_precedence();
        self._next_token();

        let right = self._parse_expression(precedence);

        Some(Expression::InfixExpression(InfixExpression::new(
            Node::Expression(left),
            operator,
            Node::Expression(right),
        )))
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

    fn _no_prefix_parse_in_error(&mut self, token_type: TokenType) {
        let error = format!("No prefix parse function for {:?} found", token_type);
        self.errors.push(error);
    }

    fn _current_precedence(&self) -> PrecedenceType {
        match self.current_token {
            Some(ref token) => token.token_type.precedence(),
            None => PrecedenceType::Lowest,
        }
    }

    fn _peek_precedence(&self) -> PrecedenceType {
        match self.peek_token {
            Some(ref token) => token.token_type.precedence(),
            None => PrecedenceType::Lowest,
        }
    }

    fn _register_prefix(
        &mut self,
        token_type: TokenType,
        func: fn(&mut Parser<'a>) -> Option<Expression>,
    ) {
        self.prefix_parse_fns.insert(token_type, func);
    }

    fn _register_infix(
        &mut self,
        token_type: TokenType,
        func: fn(&mut Parser<'a>, Expression) -> Option<Expression>,
    ) {
        self.infix_parse_fns.insert(token_type, func);
    }
}
