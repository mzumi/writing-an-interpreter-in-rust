
pub trait Node {
    fn token_literal(&self) -> String;
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
}
