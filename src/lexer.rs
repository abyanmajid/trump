use crate::token::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub source: &'a str,            // Reference to the source code
    pub position: usize,            // The current position (0 initially)
    pub read_position: usize,       // The next position in the input
    pub line_no: usize,             // The current line number (1 initially)
    pub current_char: Option<char>, // The current character being processed
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
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

    pub fn next_token(&mut self) -> Token {
        self._skip_whitespace();

        let token = match self.current_char {
            Some('+') => self._create_single_char_token(TokenType::Plus),
            Some('-') => self._create_single_char_token(TokenType::Minus),
            Some('*') => self._create_single_char_token(TokenType::Asterisk),
            Some('/') => self._create_single_char_token(TokenType::Slash),
            Some('^') => self._create_single_char_token(TokenType::Pow),
            Some('%') => self._create_single_char_token(TokenType::Modulus),
            Some('(') => self._create_single_char_token(TokenType::LeftParen),
            Some(')') => self._create_single_char_token(TokenType::RightParen),
            Some(';') => self._create_single_char_token(TokenType::SemiColon),
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

    fn _create_single_char_token(&mut self, token_type: TokenType) -> Token {
        let lexeme = self.current_char.unwrap().to_string();
        self._read_char();
        self._new_token(token_type, lexeme)
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
}
