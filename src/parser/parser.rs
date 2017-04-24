use lexer::Lexer;
use token::{Token, TokenType};
use ast::{Program, Statement, LetStatement, Identifier, ReturnStatement};
use parser::{ParseError, ParseErrorKind};

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut l = lexer.clone();
        let current_token = l.next_token();
        let peek_token = l.next_token();

        Parser {
            lexer: l,
            current_token: current_token,
            peek_token: peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parser_program(&mut self) -> Result<Program, ParseError> {
        let statements: Vec<Box<Statement>> = vec![];
        let mut program = Program::new(statements);

        loop {
            if self.current_token_is(TokenType::EOF) {
                break;
            }

            let s = self.parse_statement()?;
            program.statements.push(s);

            self.next_token();
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> Result<Box<Statement>, ParseError> {
        return match self.current_token.token_type {
            TokenType::Let => {
                let let_statement = self.parse_let_statement()?;
                let s: Result<Box<Statement>, _> = Ok(Box::new(let_statement));
                return s;
            }
            TokenType::Return => {
                let return_statement = self.parse_return_statement()?;
                let s: Result<Box<Statement>, _> = Ok(Box::new(return_statement));
                return s;
            }
            _ => {
                Err(ParseError::new(ParseErrorKind::UnexpectedToken(self.clone(),
                                                                    self.current_token
                                                                        .token_type
                                                                        .clone())))
            }
        };
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, ParseError> {
        let token = self.current_token.clone();

        self.expect_peek(TokenType::Ident)?;

        let name = Identifier::new(self.current_token.clone(),
                                   self.current_token.literal.clone());

        let statement = LetStatement {
            token: token,
            name: name,
        };

        self.expect_peek(TokenType::Assign)?;

        loop {
            if self.current_token_is(TokenType::Semicolon) {
                break;
            }
            self.next_token();
        }

        Ok(statement)
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, ParseError> {
        let statement = ReturnStatement { token: self.current_token.clone() };

        self.next_token();

        loop {
            if self.current_token_is(TokenType::Semicolon) {
                break;
            }
            self.next_token();
        }

        Ok(statement)
    }

    fn current_token_is(&self, token_type: TokenType) -> bool {
        self.current_token.token_type == token_type
    }

    fn peek_token_is(&self, token_type: TokenType) -> bool {
        self.peek_token.token_type == token_type
    }

    fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParseError> {
        if self.peek_token_is(token_type.clone()) {
            self.next_token();
            return Ok(());
        }
        Err(ParseError::new(ParseErrorKind::UnexpectedToken(self.clone(), token_type.clone())))
    }
}

#[test]
fn test_let_statement() {
    let input = r#"
    let x = 5;
    let y = 10;
    let foobar = 838383;
    "#
        .to_owned();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    match parser.parser_program() {
        Ok(p) => {
            assert_eq!(3, p.statements.len());
            for s in p.statements {
                assert_eq!("let".to_owned(), s.token_literal());
            }
        }
        Err(err) => panic!("{}", err.error_description()),
    };
}

#[test]
fn test_invalid_let_statement() {
    let input = r#"
    let x = 5;
    let = 10;
    let foobar = 838383;
    "#
        .to_owned();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parser_program();
    assert!(program.is_err());
    assert_eq!("expected next token to be Ident, got Assign instead",
               program.err().unwrap().error_description());
}

#[test]
fn test_return_statement() {
    let input = r#"
    return 5;
    return 10;
    return add(15);
    "#
        .to_owned();

    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parser_program().unwrap();
    assert_eq!(3, program.statements.len());
    for s in program.statements {
        assert_eq!("return".to_owned(), s.token_literal());
    }
}
