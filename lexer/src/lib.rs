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

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    cur_char: char,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_char == '\0' {
            return None;
        }

        self.skip_whitespace();

        let token = if let Some(single_operator) = SINGLE_OPS.get(&self.cur_char) {
            let peeked = self.read_char(true);

            if self.cur_char == '=' && peeked == '=' {
                self.read_char(false);
                Token::new(TokenType::Eq, "==")
            } else if self.cur_char == '!' && peeked == '=' {
                self.read_char(false);
                Token::new(TokenType::NotEq, "!=")
            } else if self.cur_char == '+' && peeked == '+' {
                self.read_char(false);
                Token::new(TokenType::Increment, "++")
            } else if self.cur_char == '-' && peeked == '-' {
                self.read_char(false);
                Token::new(TokenType::Decrement, "--")
            } else {
                single_operator.clone()
            }
        } else if self.cur_char == '\0' {
            Token::new(TokenType::Eof, "\0")
        } else if self.cur_char == '\'' || self.cur_char == '\"' {
            Token::new(TokenType::String, self.read_string())
        } else if self.cur_char.is_alphabetic() {
            let start = self.position;

            while self.cur_char.is_alphabetic() {
                self.read_char(false);
            }

            let literal = &self.input[start..self.position];
            Token::new(lookup_keyword(literal), literal)
        } else if self.cur_char.is_digit(10) {
            let start = self.position;
            while self.cur_char.is_digit(10) {
                self.read_char(false);
            }
            Token::new(TokenType::Int, &self.input[start..self.position])
        } else {
            Token::new(TokenType::Illegal, "?")
        };

        Some(token)
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
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

    fn read_string(&mut self) -> &'a str {
        let start = self.position + 1;

        loop {
            let read = self.read_char(false);

            if read == self.cur_char || read == '\0' {
                break;
            }
        }

        return &self.input[start..self.position];
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
