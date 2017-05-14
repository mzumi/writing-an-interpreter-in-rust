use token::Token;
use ast::{Node, Statement, Expression};

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<Box<Expression>>,
}

impl Statement for ExpressionStatement {
    fn node(&self) -> String {
        "".to_owned()
    }
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn to_string(&self) -> String {
        let mut s = String::new();
        if let Some(ref expression) = self.expression {
            s += expression.to_string().as_ref();
        }
        s
    }
}