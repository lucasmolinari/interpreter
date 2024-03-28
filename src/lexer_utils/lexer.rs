use crate::lexer_utils::token::{Token, TokenType, Keywords};

#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub keywords: Keywords,
    pub position: usize,      // current position
    pub read_position: usize, // next position
    pub ch: char,             // current char under analysis
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            keywords: Keywords::default(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap()
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_space();
        let tok = match self.ch {
            '(' => Token::new_token(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new_token(TokenType::RPAREN, self.ch.to_string()),
            '{' => Token::new_token(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new_token(TokenType::RBRACE, self.ch.to_string()),
            ',' => Token::new_token(TokenType::COMMA, self.ch.to_string()),
            '+' => Token::new_token(TokenType::PLUS, self.ch.to_string()),
            '-' => Token::new_token(TokenType::MINUS, self.ch.to_string()),
            '/' => Token::new_token(TokenType::SLASH, self.ch.to_string()),
            '*' => Token::new_token(TokenType::ASTERISK, self.ch.to_string()),
            '<' => Token::new_token(TokenType::LT, self.ch.to_string()),
            '>' => Token::new_token(TokenType::GT, self.ch.to_string()),
            ';' => Token::new_token(TokenType::SEMICOLON, self.ch.to_string()),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new_token(TokenType::NOTEQ, "!=".to_string())
                } else {
                    Token::new_token(TokenType::BANG, self.ch.to_string())
                }
            }
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::new_token(TokenType::EQ, "==".to_string())
                } else {
                    Token::new_token(TokenType::ASSIGN, self.ch.to_string())
                }
            }
            '\0' => Token::new_token(TokenType::EOF, self.ch.to_string()),
            _ => {
                if self.ch.is_alphabetic() {
                    let keyword = self.read_identifier();
                    let tok_type = self.keywords.check_ident(&keyword);
                    return Token::new_token(tok_type, keyword);
                } else if self.ch.is_alphanumeric() {
                    let int = self.read_int();
                    return Token::new_token(TokenType::INT, int);
                } else {
                    Token::new_token(TokenType::ILLEGAL, self.ch.to_string())
                }
            }
        };
        self.read_char();
        return tok;
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.ch.is_numeric() {
            self.read_char();
        }
        return self.input[position..self.position].to_string();
    }

    fn is_letter(&mut self) -> bool {
        return self.ch.is_alphabetic() || self.ch == '_';
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.chars().nth(self.read_position).unwrap();
        }
    }

    fn skip_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}
