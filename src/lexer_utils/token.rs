use ::std::collections::HashMap;
#[derive(Debug, Clone, PartialEq, Copy, Hash, Eq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    EQ,
    NOTEQ,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    LT,
    GT,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Token {
    pub fn new_token(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type: token_type,
            literal: literal,
        }
    }
}
#[derive(Debug)]
pub struct Keywords {
    pub map: HashMap<&'static str, TokenType>,
}
impl Keywords {
    pub fn new() -> Keywords {
        let map = HashMap::from([
            ("fn", TokenType::FUNCTION),
            ("let", TokenType::LET),
            ("true", TokenType::TRUE),
            ("false", TokenType::FALSE),
            ("if", TokenType::IF),
            ("else", TokenType::ELSE),
            ("return", TokenType::RETURN),
        ]
        );
        
        Keywords { map }
    }

    pub fn check_ident(&self, keyword: &str) -> TokenType {
        match self.map.get(keyword) {
            Some(k_word) => k_word.clone(),
            None => TokenType::IDENT.clone(),
        }
    }
}
