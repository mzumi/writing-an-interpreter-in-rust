use token::Token;
use ast::{Node, Expression};

#[derive(Debug)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn new(token: Token, value: String) -> Self {
        Identifier {
            token: token,
            value: value,
        }
    }
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl Expression for Identifier {
    fn node(&self) -> String {
        "".to_owned()
    }
}
