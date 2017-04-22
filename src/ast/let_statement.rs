use token::Token;
use ast::{Node, Statement, Identifier};

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier, // value: Expression,
}

impl Statement for LetStatement {
    fn node(&self) -> String {
        "".to_owned()
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
