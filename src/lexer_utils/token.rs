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

#[derive(Debug, Clone)]
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
    pub fn check_ident(&self, keyword: &str) -> TokenType {
        match self.map.get(keyword) {
            Some(k_word) => k_word.clone(),
            None => TokenType::IDENT.clone(),
        }
    }
}
impl Default for Keywords {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("fn", TokenType::FUNCTION);
        map.insert("let", TokenType::LET);
        map.insert("true", TokenType::TRUE);
        map.insert("false", TokenType::FALSE);
        map.insert("if", TokenType::IF);
        map.insert("else", TokenType::ELSE);
        map.insert("return", TokenType::RETURN);

        Keywords { map }
    }
}
