use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    // Identifier + literals
    Ident,
    Int,

    // Operaters
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    EQ,
    NotEQ,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl TokenType {
    pub fn lookup_indent(indent: &String) -> Self {
        let keywords = [TokenType::Function,
                        TokenType::Let,
                        TokenType::If,
                        TokenType::Return,
                        TokenType::True,
                        TokenType::False,
                        TokenType::Else];

        let token_type = indent.parse::<TokenType>();
        if let Some(t) = token_type.ok() {
            if keywords.contains(&t) {
                return t;
            }
        }

        TokenType::Ident
    }
}

impl FromStr for TokenType {
    type Err = ParseTokenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "=" => Ok(TokenType::Assign),
            "+" => Ok(TokenType::Plus),
            "-" => Ok(TokenType::Minus),
            "!" => Ok(TokenType::Bang),
            "*" => Ok(TokenType::Asterisk),
            "/" => Ok(TokenType::Slash),
            "<" => Ok(TokenType::LT),
            ">" => Ok(TokenType::GT),
            "," => Ok(TokenType::Comma),
            ";" => Ok(TokenType::Semicolon),
            "(" => Ok(TokenType::LParen),
            ")" => Ok(TokenType::RParen),
            "{" => Ok(TokenType::LBrace),
            "}" => Ok(TokenType::RBrace),
            "==" => Ok(TokenType::EQ),
            "!=" => Ok(TokenType::NotEQ),
            "fn" => Ok(TokenType::Function),
            "let" => Ok(TokenType::Let),
            "if" => Ok(TokenType::If),
            "return" => Ok(TokenType::Return),
            "true" => Ok(TokenType::True),
            "false" => Ok(TokenType::False),
            "else" => Ok(TokenType::Else),
            _ => Err(ParseTokenError),
        }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct ParseTokenError;

#[test]
fn parse_test() {
    let tests = vec![("=", TokenType::Assign),
                     ("+", TokenType::Plus),
                     (",", TokenType::Comma),
                     (";", TokenType::Semicolon),
                     ("(", TokenType::LParen),
                     (")", TokenType::RParen),
                     ("{", TokenType::LBrace),
                     ("}", TokenType::RBrace),
                     ("fn", TokenType::Function),
                     ("let", TokenType::Let),
                     ("if", TokenType::If)];

    for (s, e) in tests {
        assert_eq!(s.parse::<TokenType>().unwrap(), e);
    }

}

#[test]
fn fail_parse() {
    assert!("TEST".parse::<TokenType>().is_err());
}
