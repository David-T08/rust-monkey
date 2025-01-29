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
    pub position: usize,
    pub read_position: usize,
    cur_char: char,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
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
        return new
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
        return self.cur_char
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_can_do_basic_symbols() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);
        let expects = vec![
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, "\0"),
        ];
        for expected in expects {
            let tok = lexer.next().unwrap();
												println!("{tok}, {}", lexer.position);
						
            // assert_eq!(tok, expected);
        }
    }
    #[test]
    fn it_can_do_a_basic_program() {
        let input = r#"
            let five = 5;
            let ten = 10;
            let add = fn(x, y) {
                x + y;
            };
            let result = add(five, ten);
        "#;
        let expects = vec![
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "five"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "ten"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "add"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Function, "fn"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Ident, "y"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Plus, "+"),
            Token::new(TokenType::Ident, "y"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Ident, "result"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Ident, "add"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Ident, "five"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Ident, "ten"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Eof, "\0"),
        ];
        let mut lexer = Lexer::new(input);
        for expected in expects {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected);
        }
    }
    #[test]
    fn it_can_do_operators() {
        let input = r#"
            !-/*5;
            5 < 10 > 5;
        "#;
        let expects = vec![
            Token::new(TokenType::Bang, "!"),
            Token::new(TokenType::Minus, "-"),
            Token::new(TokenType::Div, "/"),
            Token::new(TokenType::Mult, "*"),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::LessThan, "<"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::GreaterThan, ">"),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Semicolon, ";"),
        ];
        let mut lexer = Lexer::new(input);
        for expected in expects {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected);
        }
    }
    #[test]
    fn it_can_do_a_basic_if_statement() {
        let input = r#"
            if (5 < 10) {
                return true;
            } else {
                return false;
            }
        "#;
        let expects = vec![
            Token::new(TokenType::If, "if"),
            Token::new(TokenType::LParen, "("),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::LessThan, "<"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::RParen, ")"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::True, "true"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Else, "else"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Return, "return"),
            Token::new(TokenType::False, "false"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::RBrace, "}"),
        ];
        let mut lexer = Lexer::new(input);
        for expected in expects {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected);
        }
    }
    #[test]
    fn it_can_do_pre_increment_decrement_operators() {
        let input = "x++; --x; x--; ++x";
        let expects = vec![
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Increment, "++"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Decrement, "--"),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Decrement, "--"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Increment, "++"),
            Token::new(TokenType::Ident, "x"),
        ];
        let mut lexer = Lexer::new(input);
        for expected in expects {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected);
        }
    }
    #[test]
    fn it_can_fully_lex() {
        let input = r#"
            10 == 10;
            10 != 9;
            "foobar"
            " foo  bar"
            "I have nested quotes! \"omggg\""
            [1, 2];
            {"foo": "bar", "test": 1}
            {1: 2};
            let const x = 5
        "#;
        let expects = vec![
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Eq, "=="),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Int, "10"),
            Token::new(TokenType::NotEq, "!="),
            Token::new(TokenType::Int, "9"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::String, "foobar"),
            Token::new(TokenType::String, " foo  bar"),
            Token::new(TokenType::String, r#"I have nested quotes! "omggg""#),
            Token::new(TokenType::LBracket, "["),
            Token::new(TokenType::Int, "1"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::Int, "2"),
            Token::new(TokenType::RBracket, "]"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::String, "foo"),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::String, "bar"),
            Token::new(TokenType::Comma, ","),
            Token::new(TokenType::String, "test"),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::Int, "1"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::LBrace, "{"),
            Token::new(TokenType::Int, "1"),
            Token::new(TokenType::Colon, ":"),
            Token::new(TokenType::Int, "2"),
            Token::new(TokenType::RBrace, "}"),
            Token::new(TokenType::Semicolon, ";"),
            Token::new(TokenType::Let, "let"),
            Token::new(TokenType::Constant, "const"),
            Token::new(TokenType::Ident, "x"),
            Token::new(TokenType::Assign, "="),
            Token::new(TokenType::Int, "5"),
            Token::new(TokenType::Eof, "\0"),
        ];
        let mut lexer = Lexer::new(input);
        for expected in expects {
            let token = lexer.next().unwrap();
            assert_eq!(token, expected);
        }
    }
}