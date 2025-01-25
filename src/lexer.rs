use crate::token::{Token, TokenType};

pub struct Lexer {
    pub source: String,        // The source code
    pub position: usize,       // The current position (0 initially)
    pub read_position: usize,  // The next position in the input
    pub line_no: usize,        // The current line number (1 initially)
    pub current_char: Option<char>, // The current character being processed
}

impl Lexer {
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer {
            source,
            position: 0,
            read_position: 0,
            line_no: 1,
            current_char: None,
        };

        lexer._read_char();
        lexer
    }

    fn _read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.current_char = None;
        } else {
            self.current_char = self.source.chars().nth(self.read_position);
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn _skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() {
                self._read_char();
            } else {
                break;
            }
        }
    }

    fn _new_token(&self, token_type: TokenType, lexeme: String) -> Token {
        Token::new(token_type, lexeme, self.line_no, self.position)
    }

    fn _read_number(&mut self) -> String {
        let start_position = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' {
                self._read_char();
            } else {
                break;
            }
        }
        self.source[start_position..self.position].to_string()
    }

    fn _read_identifier(&mut self) -> String {
        let start_position = self.position;
        while let Some(ch) = self.current_char {
            if ch.is_alphabetic() || ch == '_' {
                self._read_char();
            } else {
                break;
            }
        }
        self.source[start_position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self._skip_whitespace();

        let token = match self.current_char {
            Some('+') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Add, lexeme)
            }
            Some('-') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Subtract, lexeme)
            }
            Some('*') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Multiply, lexeme)
            }
            Some('/') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Divide, lexeme)
            }
            Some('^') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Power, lexeme)
            }
            Some('%') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Modulus, lexeme)
            }
            Some('(') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::LeftParen, lexeme)
            }
            Some(')') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::RightParen, lexeme)
            }
            Some(';') => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::SemiColon, lexeme)
            }
            Some(ch) if ch.is_ascii_digit() || ch == '.' => {
                let lexeme = self._read_number();
                if lexeme.contains('.') {
                    self._new_token(TokenType::Float, lexeme)
                } else {
                    self._new_token(TokenType::Integer, lexeme)
                }
            }
            Some(ch) if ch.is_alphabetic() || ch == '_' => {
                let lexeme = self._read_identifier();
                self._new_token(TokenType::Identifier, lexeme)
            }
            None => self._new_token(TokenType::Eof, "".to_string()),
            Some(_) => {
                let lexeme = self.current_char.unwrap().to_string();
                self._read_char();
                self._new_token(TokenType::Illegal, lexeme)
            }
        };

        token
    }
}