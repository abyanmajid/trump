pub enum TokenType {
    // Variables
    Identifier,

    // Data types
    Integer,
    Float,

    // Arithmetic operators
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulus,

    // Symbols
    SemiColon,
    LeftParen,
    RightParen,

    // Errors
    Eof,
    Illegal,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String, // The string literal encoding of the current token
    pub line: usize, // The line number where the token was found
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