
pub trait Node {
    fn token_literal(&self) -> String;
    fn to_string(&self) -> String;
}

pub trait Statement: Node {
    fn node(&self) -> String;
}

pub trait Expression: Node {
    fn node(&self) -> String;
}

pub struct Program {
    pub statements: Vec<Box<Statement>>,
}

impl Program {
    pub fn new(statements: Vec<Box<Statement>>) -> Self {
        Program { statements: statements }
    }
    pub fn token_literal(&self) -> String {
        if self.statements.len() > 0 {
            for n in self.statements.iter() {
                return n.token_literal();
            }
        }
        "".to_owned()
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for statement in &self.statements {
            s += statement.to_string().as_ref();
        }

        s
    }
}

#[cfg(test)]
pub mod test {
    use ast::{Program, LetStatement, Identifier};
    use token::{Token, TokenType};

    #[test]
    fn test_to_stirng() {
        let token = Token::new(TokenType::Let, "let".to_owned());
        let identifier = Identifier::new(Token::new(TokenType::Ident, "myVar".to_owned()),
                                         "myVar".to_owned());
        let value = Identifier::new(Token::new(TokenType::Ident, "anotherVar".to_owned()),
                                    "anotherVar".to_owned());

        let let_statement = LetStatement::new(token, identifier, Some(Box::new(value)));
        let program = Program { statements: vec![Box::new(let_statement)] };

        assert_eq!(program.to_string(), "let myVar = anotherVar;");
    }

}
