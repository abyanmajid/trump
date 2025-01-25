use crate::parser::PrecedenceType;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum TokenType {
    // Variables
    Identifier,

    // Data types
    Integer,
    Float,

    // Arithmetic operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Pow,
    Modulus,

    // Symbols
    SemiColon,
    LeftParen,
    RightParen,

    // Errors
    Eof,
    Illegal,
}

impl TokenType {
    pub fn precedence(&self) -> PrecedenceType {
        match self {
            TokenType::Plus | TokenType::Minus => PrecedenceType::Sum,
            TokenType::Asterisk | TokenType::Slash | TokenType::Modulus => PrecedenceType::Product,
            TokenType::Pow => PrecedenceType::Power,
            _ => PrecedenceType::Lowest,
        }
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,  // The string literal encoding of the current token
    pub line: usize,     // The line number where the token was found
    pub position: usize, // The position (index) in the line where the token was found
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, position: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            position,
        }
    }
}
