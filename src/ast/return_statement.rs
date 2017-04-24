use token::Token;
use ast::{Node, Statement};

pub struct ReturnStatement {
    pub token: Token, // pub return_value: Expression,
}

impl Statement for ReturnStatement {
    fn node(&self) -> String {
        "".to_owned()
    }
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}
