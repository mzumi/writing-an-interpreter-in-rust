use std::fmt;
use parser::Parser;
use token::TokenType;

#[derive(Debug, Clone)]
pub struct ParseError {
    kind: ParseErrorKind,
}

#[derive(Debug, Clone)]
pub enum ParseErrorKind {
    UnexpectedToken(Parser, TokenType),
}

impl ParseError {
    pub fn new(kind: ParseErrorKind) -> Self {
        ParseError { kind: kind }
    }

    pub fn error_description(&self) -> String {
        return match self.kind {
            ParseErrorKind::UnexpectedToken(ref p, ref t) => {
                format!("expected next token to be {}, got {} instead",
                        t.clone(),
                        p.clone().peek_token.token_type)
            }
        };
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error_description().fmt(f)
    }
}
