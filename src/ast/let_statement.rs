use token::Token;
use ast::{Node, Statement, Identifier, Expression};

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<Box<Expression>>,
}

impl LetStatement {
    pub fn new(token: Token, name: Identifier, value: Option<Box<Expression>>) -> Self {
        LetStatement {
            token: token,
            name: name,
            value: value,
        }
    }
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

    fn to_string(&self) -> String {
        let mut s = String::new();
        s += format!("{} {} = ", self.token_literal(), self.name.to_string()).as_str();
        if let Some(ref value) = self.value {
            s += value.to_string().as_ref();
        }
        s += ";";
        s
    }
}
