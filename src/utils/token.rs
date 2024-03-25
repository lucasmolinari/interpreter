use::std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
}


#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
impl Token{
    pub fn new_token(token_type: TokenType, literal: String) -> Token {
        return Token {
            token_type: token_type,
            literal: literal,
        };
    }
}

pub struct Keywords {
    pub map: HashMap<&'static str, TokenType>
}
impl Keywords {
    pub fn check_ident (&self, keyword: &str) -> TokenType {
        match self.map.get(keyword) {
            Some(&ref k_word) => return k_word.clone(),
            None => return TokenType::IDENT.clone()
        }
    }

}

impl Default for Keywords {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert("fn", TokenType::FUNCTION);
        map.insert("let", TokenType::LET);

        Keywords { map }
    }
}