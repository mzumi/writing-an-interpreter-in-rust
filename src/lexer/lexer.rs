use token::*;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        let chars = self.input.chars().collect::<Vec<char>>();
        if chars.len() > self.read_position {
            self.ch = Some(chars[self.read_position]);
        } else {
            self.ch = None;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(c) = self.ch {
            let token = match c {
                '+' | '-' | ';' | '(' | ')' | ',' | '{' | '}' | '/' | '*' | '<' | '>' | '=' |
                '!' => {
                    let literal = if c == '=' || c == '!' {
                        self.get_equal_or_not_equal_string(&c).unwrap_or(c.to_string())
                    } else {
                        c.to_string()
                    };

                    let token_type = literal.parse::<TokenType>().unwrap_or(TokenType::EOF);
                    self.read_char();
                    Token::new(token_type, literal)
                }
                _ => {
                    if self.is_letter() {
                        let literal = self.read_identifier();
                        let token_type = TokenType::lookup_indent(&literal);

                        Token::new(token_type, literal)
                    } else if self.is_digit() {
                        Token::new(TokenType::Int, self.read_number())
                    } else {
                        Token::new(TokenType::Illegal, c.to_string())
                    }
                }
            };

            return token;
        }

        self.read_char();
        Token::new(TokenType::EOF, "".to_owned())
    }

    fn is_letter(&self) -> bool {
        if let Some(ch) = self.ch {
            return ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_';
        }
        false
    }

    fn is_digit(&self) -> bool {
        if let Some(ch) = self.ch {
            return '0' <= ch && ch <= '9';
        }
        false
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter() {
            self.read_char();
        }

        self.slice_input(position)
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit() {
            self.read_char();
        }

        self.slice_input(position)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.ch {
            if vec![' ', '\t', '\n', '\r'].contains(&ch) {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        let chars = self.input.chars().collect::<Vec<char>>();
        if self.read_position > chars.len() {
            None
        } else {
            Some(chars[self.read_position])
        }
    }

    fn get_equal_or_not_equal_string(&mut self, c: &char) -> Option<String> {
        if let Some(next_char) = self.peek_char() {
            if next_char == '=' {
                self.read_char();
                return Some(c.to_string() + self.ch.unwrap().to_string().as_str());
            }
        }
        None
    }

    fn slice_input(&self, position: usize) -> String {
        self.input
            .chars()
            .collect::<Vec<char>>()
            .get(position..self.position)
            .unwrap()
            .into_iter()
            .map(|c| c.clone())
            .collect()
    }
}

#[test]
fn test_next_token() {
    let input = r#"
    let five = 5;
    let ten = 10;

    let add = fn(x, y) {
        x + y;
    };

    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;

    if (5 < 10) {
        return true;
    } else {
        return false;
    }
    10 == 10;
    10 != 9;
    "#
        .to_owned();

    let tests = vec![("let", TokenType::Let),
                     ("five", TokenType::Ident),
                     ("=", TokenType::Assign),
                     ("5", TokenType::Int),
                     (";", TokenType::Semicolon),
                     ("let", TokenType::Let),
                     ("ten", TokenType::Ident),
                     ("=", TokenType::Assign),
                     ("10", TokenType::Int),
                     (";", TokenType::Semicolon),
                     ("let", TokenType::Let),
                     ("add", TokenType::Ident),
                     ("=", TokenType::Assign),
                     ("fn", TokenType::Function),
                     ("(", TokenType::LParen),
                     ("x", TokenType::Ident),
                     (",", TokenType::Comma),
                     ("y", TokenType::Ident),
                     (")", TokenType::RParen),
                     ("{", TokenType::LBrace),
                     ("x", TokenType::Ident),
                     ("+", TokenType::Plus),
                     ("y", TokenType::Ident),
                     (";", TokenType::Semicolon),
                     ("}", TokenType::RBrace),
                     (";", TokenType::Semicolon),
                     ("let", TokenType::Let),
                     ("result", TokenType::Ident),
                     ("=", TokenType::Assign),
                     ("add", TokenType::Ident),
                     ("(", TokenType::LParen),
                     ("five", TokenType::Ident),
                     (",", TokenType::Comma),
                     ("ten", TokenType::Ident),
                     (")", TokenType::RParen),
                     (";", TokenType::Semicolon),
                     ("!", TokenType::Bang),
                     ("-", TokenType::Minus),
                     ("/", TokenType::Slash),
                     ("*", TokenType::Asterisk),
                     ("5", TokenType::Int),
                     (";", TokenType::Semicolon),
                     ("5", TokenType::Int),
                     ("<", TokenType::LT),
                     ("10", TokenType::Int),
                     (">", TokenType::GT),
                     ("5", TokenType::Int),
                     (";", TokenType::Semicolon),
                     ("if", TokenType::If),
                     ("(", TokenType::LParen),
                     ("5", TokenType::Int),
                     ("<", TokenType::LT),
                     ("10", TokenType::Int),
                     (")", TokenType::RParen),
                     ("{", TokenType::LBrace),
                     ("return", TokenType::Return),
                     ("true", TokenType::True),
                     (";", TokenType::Semicolon),
                     ("}", TokenType::RBrace),
                     ("else", TokenType::Else),
                     ("{", TokenType::LBrace),
                     ("return", TokenType::Return),
                     ("false", TokenType::False),
                     (";", TokenType::Semicolon),
                     ("}", TokenType::RBrace),
                     ("10", TokenType::Int),
                     ("==", TokenType::EQ),
                     ("10", TokenType::Int),
                     (";", TokenType::Semicolon),
                     ("10", TokenType::Int),
                     ("!=", TokenType::NotEQ),
                     ("9", TokenType::Int),
                     (";", TokenType::Semicolon),
                     ("", TokenType::EOF)];

    let mut lexer = Lexer::new(input);
    for (literal, token_type) in tests {
        let token = lexer.next_token();
        assert_eq!(literal.to_owned(), token.literal);
        assert_eq!(token_type, token.token_type);
    }
}
