pub enum TokenEnum {
    // Data types
    Integer,
    Float,

    // Arithmetic operators
    Add,
    Subtract,
    Times,
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
    pub token_type: TokenEnum,
    pub lexeme: String,
    pub line: u32,
    pub column: u32,
}

impl Token {
    pub fn new(token_type: TokenEnum, lexeme: String, line: u32, column: u32) -> Self {
        Self {
            token_type,
            lexeme,
            line,
            column,
        }
    }
}