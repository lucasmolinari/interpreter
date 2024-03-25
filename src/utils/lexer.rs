use super::token::Keywords;
use super::token::Token;
use super::token::TokenType;
#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pub position: usize,      // current position
    pub read_position: usize, // next position
    pub ch: char,             // current char under analysis
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
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
        // TODO: Find a way to initialize 'keywords' only once
        let keywords = Keywords::default();
        self.skip_space();
        let tok = match self.ch {
            '=' => Token::new_token(TokenType::ASSIGN, self.ch.to_string()),
            '(' => Token::new_token(TokenType::LPAREN, self.ch.to_string()),
            ')' => Token::new_token(TokenType::RPAREN, self.ch.to_string()),
            '{' => Token::new_token(TokenType::LBRACE, self.ch.to_string()),
            '}' => Token::new_token(TokenType::RBRACE, self.ch.to_string()),
            ',' => Token::new_token(TokenType::COMMA, self.ch.to_string()),
            '+' => Token::new_token(TokenType::PLUS, self.ch.to_string()),
            ';' => Token::new_token(TokenType::SEMICOLON, self.ch.to_string()),
            '\0' => Token::new_token(TokenType::EOF, self.ch.to_string()),
            _ => {
                if self.ch.is_alphabetic() {
                    let keyword = self.read_identifier();
                    let tok_type = keywords.check_ident(&keyword);
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

    fn skip_space(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}