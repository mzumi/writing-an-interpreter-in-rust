use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    // Identifier + literals
    IDENT,
    INT,

    // Operaters
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,

    LT,
    GT,

    EQ,
    NOT_EQ,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

impl TokenType {
    pub fn lookup_indent(indent: &String) -> Self {
        let keywords = [TokenType::FUNCTION,
                        TokenType::LET,
                        TokenType::IF,
                        TokenType::RETURN,
                        TokenType::TRUE,
                        TokenType::FALSE,
                        TokenType::ELSE];

        let token_type = indent.parse::<TokenType>();
        if let Some(t) = token_type.ok() {
            if keywords.contains(&t) {
                return t;
            }
        }

        TokenType::IDENT
    }
}

impl FromStr for TokenType {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(TokenType::ASSIGN),
            "+" => Ok(TokenType::PLUS),
            "-" => Ok(TokenType::MINUS),
            "!" => Ok(TokenType::BANG),
            "*" => Ok(TokenType::ASTERISK),
            "/" => Ok(TokenType::SLASH),
            "<" => Ok(TokenType::LT),
            ">" => Ok(TokenType::GT),
            "," => Ok(TokenType::COMMA),
            ";" => Ok(TokenType::SEMICOLON),
            "(" => Ok(TokenType::LPAREN),
            ")" => Ok(TokenType::RPAREN),
            "{" => Ok(TokenType::LBRACE),
            "}" => Ok(TokenType::RBRACE),
            "==" => Ok(TokenType::EQ),
            "!=" => Ok(TokenType::NOT_EQ),
            "fn" => Ok(TokenType::FUNCTION),
            "let" => Ok(TokenType::LET),
            "if" => Ok(TokenType::IF),
            "return" => Ok(TokenType::RETURN),
            "true" => Ok(TokenType::TRUE),
            "false" => Ok(TokenType::FALSE),
            "else" => Ok(TokenType::ELSE),
            _ => Err(ParseTokenError),
        }
    }
}

#[derive(Debug)]
pub struct ParseTokenError;

#[test]
fn parse_test() {
    let tests = vec![("=", TokenType::ASSIGN),
                     ("+", TokenType::PLUS),
                     (",", TokenType::COMMA),
                     (";", TokenType::SEMICOLON),
                     ("(", TokenType::LPAREN),
                     (")", TokenType::RPAREN),
                     ("{", TokenType::LBRACE),
                     ("}", TokenType::RBRACE),
                     ("fn", TokenType::FUNCTION),
                     ("let", TokenType::LET),
                     ("if", TokenType::IF)];

    for (s, e) in tests {
        assert_eq!(s.parse::<TokenType>().unwrap(), e);
    }

}

#[test]
fn fail_parse() {
    assert!("TEST".parse::<TokenType>().is_err());
}
