use std::collections::HashMap;
use tokens::{lookup_keyword, Token, TokenType};

lazy_static::lazy_static! {
    static ref SINGLE_OPS: HashMap<char, Token<'static>> = {
        let mut ops = HashMap::new();
        ops.insert('=', Token::new(TokenType::Assign, "="));
        ops.insert('-', Token::new(TokenType::Minus, "-"));
        ops.insert('+', Token::new(TokenType::Plus, "+"));
        ops.insert('*', Token::new(TokenType::Mult, "*"));
        ops.insert('/', Token::new(TokenType::Div, "/"));
        ops.insert('!', Token::new(TokenType::Bang, "!"));
        ops.insert('<', Token::new(TokenType::LessThan, "<"));
        ops.insert('>', Token::new(TokenType::GreaterThan, ">"));
        ops.insert(';', Token::new(TokenType::Semicolon, ";"));
        ops.insert(',', Token::new(TokenType::Comma, ","));
        ops.insert('(', Token::new(TokenType::LParen, "("));
        ops.insert(')', Token::new(TokenType::RParen, ")"));
        ops.insert('{', Token::new(TokenType::LBrace, "{"));
        ops.insert('}', Token::new(TokenType::RBrace, "}"));
        ops.insert('[', Token::new(TokenType::LBracket, "["));
        ops.insert(']', Token::new(TokenType::RBracket, "]"));
        ops.insert(':', Token::new(TokenType::Colon, ":"));
        return ops
    };
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    cur_char: char,
}

impl Iterator for Lexer {
    type Item = Token<'a>; // Error ere

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_char == '\0' {
            None
        } else {
            Some(self.next_token())
        }
    }
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut new = Lexer {
            input,
            position: 0,
            read_position: 0,
            cur_char: '\0',
        };

        new.read_char(false);
        new
    }

    fn read_char(&mut self, peek: bool) -> char {
        if self.read_position >= self.input.len() {
            if peek {
                return '\0';
            }
            self.cur_char = '\0';
        } else {
            let read = self.input[self.read_position..].chars().next().unwrap();
            if peek {
                return read;
            }
            self.cur_char = read;
        }

        self.position = self.read_position;
        self.read_position += self.cur_char.len_utf8();
        self.cur_char
    }

    fn read_iden_or_num(&mut self, tok_type: &str) -> &str {
				let start = self.position;

        match tok_type {
            "num" => {
                while self.cur_char.is_digit(10) {
                    self.read_char(false);
                }
            }
            "iden" => {
                while self.cur_char.is_alphabetic() {
                    self.read_char(false);
                }
            }
            _ => panic!("Invalid token type {}, expected num or iden", tok_type),
        }

        return &self.input[start..self.position];
    }

    fn read_string(&mut self) -> &str {
        let start = self.position + 1;

        loop {
            let read = self.read_char(false);

            if read == self.cur_char || read == '\0' {
                break;
            }
        }

        return &self.input[start..self.position];
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if let Some(single_operator) = SINGLE_OPS.get(&self.cur_char) {
            let peeked = self.read_char(true);

            if self.cur_char == '=' && peeked == '=' {
                self.read_char(false);
                return Token::new(TokenType::Eq, "==");
            } else if self.cur_char == '!' && peeked == '=' {
                self.read_char(false);
                return Token::new(TokenType::NotEq, "!=");
            } else if self.cur_char == '+' && peeked == '+' {
                self.read_char(false);
                return Token::new(TokenType::Increment, "++");
            } else if self.cur_char == '-' && peeked == '-' {
                self.read_char(false);
                return Token::new(TokenType::Decrement, "--");
            } else {
                return single_operator.clone();
            }
        } else if self.cur_char == '\0' {
            return Token::new(TokenType::Eof, "\0");
        } else if self.cur_char == '\'' || self.cur_char == '\"' {
            return Token::new(TokenType::String, self.read_string());
        } else if self.cur_char.is_alphabetic() {
            let literal = self.read_iden_or_num("iden");
            return Token::new(lookup_keyword(literal), literal);
        } else if self.cur_char.is_digit(10) {
            let literal = self.read_iden_or_num("num");
            return Token::new(TokenType::Int, literal);
        } else {
            return Token::new(TokenType::Illegal, "?")//&self.cur_char.to_string()[..]); // Error here
        }
    }

    fn skip_whitespace(&mut self) {
        while self.cur_char == ' '
            || self.cur_char == '\t'
            || self.cur_char == '\n'
            || self.cur_char == '\r'
        {
            self.read_char(false);
        }
    }
}