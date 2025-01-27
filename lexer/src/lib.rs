use std::collections::HashMap;
use tokens::{lookup_keyword, Token, Tokens};

lazy_static::lazy_static! {
    static ref SINGLE_OPS: HashMap<&'static str, Token> = {
        let mut ops = HashMap::new();
            ops.insert("=", Token::from_str(Tokens::Assign, "="));
            ops.insert("-", Token::from_str(Tokens::Minus, "-"));
            ops.insert("+", Token::from_str(Tokens::Plus, "+"));
            ops.insert("*", Token::from_str(Tokens::Mult, "*"));
            ops.insert("/", Token::from_str(Tokens::Div, "/"));
            ops.insert("!", Token::from_str(Tokens::Bang, "!"));
            ops.insert("<", Token::from_str(Tokens::LessThan, "<"));
            ops.insert(">", Token::from_str(Tokens::GreaterThan, ">"));
            ops.insert(";", Token::from_str(Tokens::Semicolon, ";"));
            ops.insert(",", Token::from_str(Tokens::Comma, ","));
            ops.insert("(", Token::from_str(Tokens::LParen, "("));
            ops.insert(")", Token::from_str(Tokens::RParen, ")"));
            ops.insert("{", Token::from_str(Tokens::LBrace, "{"));
            ops.insert("}", Token::from_str(Tokens::RBrace, "}"));
            ops.insert("[", Token::from_str(Tokens::LBracket, "["));
            ops.insert("]", Token::from_str(Tokens::RBracket, "]"));
            ops.insert(":", Token::from_str(Tokens::Colon, ":"));

        return ops;
    };
}

pub struct Lexer {
    input: String,

    position: usize,
    read_position: usize,

    cur_char: char,
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_char == '\0' {
            None // End of input
        } else {
            Some(self.next_token()) // Get the next token
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
        return new;
    }

    fn read_char(&mut self, peek: bool) -> char {
        // Check EOF
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
        self.read_position += self.cur_char.len_utf8(); // Move forward based on char length

        return self.cur_char;
    }

    fn read_iden_or_num(&mut self, tok_type: &str) -> String {
        let start = self.position;

        match tok_type {
            "num" => {
                while self.cur_char.is_digit(10) {
                    // Advance to next character
                    self.read_char(false);
                }
            }

            "iden" => {
                while self.cur_char.is_alphabetic() {
                    // Advance to next character
                    self.read_char(false);
                }
            }

            _ => panic!("Invalid token type {}, expected num or iden", tok_type),
        }

        return self.input[start..self.position].to_string();
    }

    fn read_string(&mut self) -> String {
        let mut escaped = false;
        let mut constructed = String::new();

        let initial = self.cur_char;

        loop {
            let read = self.read_char(false);

            if (!escaped && read == initial) || read == '\0' {
                break;
            }

            let peeked = self.read_char(true);
            if read == '\\' && (peeked == '\'' || peeked == '\"') {
                escaped = true
            } else {
                constructed.push(read);
                escaped = false;
            }
        }

        return constructed;
    }

    pub fn next_token(&mut self) -> Token {
        let token: Token;
        self.skip_whitespace();

        let single_operator = SINGLE_OPS.get(self.cur_char.to_string().as_str());
        if let Some(single_operator) = single_operator {
            let peeked = self.read_char(true);

            if self.cur_char == '=' && peeked == '=' {
                token = Token::from_str(Tokens::Eq, "==");
                self.read_char(false);
            } else if self.cur_char == '!' && peeked == '=' {
                token = Token::from_str(Tokens::NotEq, "!=");
                self.read_char(false);
            } else if self.cur_char == '+' && peeked == '+' {
                token = Token::from_str(Tokens::Increment, "++");
                self.read_char(false);
            } else if self.cur_char == '-' && peeked == '-' {
                token = Token::from_str(Tokens::Decrement, "--");
                self.read_char(false);
            } else {
                token = single_operator.clone();
            }
        } else if self.cur_char == '\0' {
            token = Token::from_str(Tokens::Eof, "\0");
        } else if self.cur_char == '\'' || self.cur_char == '\"' {
            token = Token::from_string(Tokens::String, self.read_string())
        } else {
            if self.cur_char.is_alphabetic() {
                let literal = self.read_iden_or_num("iden");

                return Token::from_string(lookup_keyword(&literal), literal);
            } else if self.cur_char.is_digit(10) {
                return Token::from_string(Tokens::Int, self.read_iden_or_num("num"));
            } else {
                token = Token::from_string(Tokens::Illegal, self.cur_char.to_string())
            }
        }

        self.read_char(false);
        return token;
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
        let mut lexer = Lexer::new(input.to_string());

        let expects = vec![
            Token::from_str(Tokens::Assign, "="),
            Token::from_str(Tokens::Plus, "+"),
            Token::from_str(Tokens::LParen, "("),
            Token::from_str(Tokens::RParen, ")"),
            Token::from_str(Tokens::LBrace, "{"),
            Token::from_str(Tokens::RBrace, "}"),
            Token::from_str(Tokens::Comma, ","),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Eof, "\0"),
        ];

        for expected in expects {
            let tok = lexer.next_token();
            assert_eq!(tok, expected);
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
            Token::from_str(Tokens::Let, "let"),
            Token::from_str(Tokens::Ident, "five"),
            Token::from_str(Tokens::Assign, "="),
            Token::from_str(Tokens::Int, "5"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Let, "let"),
            Token::from_str(Tokens::Ident, "ten"),
            Token::from_str(Tokens::Assign, "="),
            Token::from_str(Tokens::Int, "10"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Let, "let"),
            Token::from_str(Tokens::Ident, "add"),
            Token::from_str(Tokens::Assign, "="),
            Token::from_str(Tokens::Function, "fn"),
            Token::from_str(Tokens::LParen, "("),
            Token::from_str(Tokens::Ident, "x"),
            Token::from_str(Tokens::Comma, ","),
            Token::from_str(Tokens::Ident, "y"),
            Token::from_str(Tokens::RParen, ")"),
            Token::from_str(Tokens::LBrace, "{"),
            Token::from_str(Tokens::Ident, "x"),
            Token::from_str(Tokens::Plus, "+"),
            Token::from_str(Tokens::Ident, "y"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::RBrace, "}"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Let, "let"),
            Token::from_str(Tokens::Ident, "result"),
            Token::from_str(Tokens::Assign, "="),
            Token::from_str(Tokens::Ident, "add"),
            Token::from_str(Tokens::LParen, "("),
            Token::from_str(Tokens::Ident, "five"),
            Token::from_str(Tokens::Comma, ","),
            Token::from_str(Tokens::Ident, "ten"),
            Token::from_str(Tokens::RParen, ")"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Eof, "\0"),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected in expects {
            let token = lexer.next_token();
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
            Token::from_str(Tokens::Bang, "!"),
            Token::from_str(Tokens::Minus, "-"),
            Token::from_str(Tokens::Div, "/"),
            Token::from_str(Tokens::Mult, "*"),
            Token::from_str(Tokens::Int, "5"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Int, "5"),
            Token::from_str(Tokens::LessThan, "<"),
            Token::from_str(Tokens::Int, "10"),
            Token::from_str(Tokens::GreaterThan, ">"),
            Token::from_str(Tokens::Int, "5"),
            Token::from_str(Tokens::Semicolon, ";"),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected in expects {
            let token = lexer.next_token();
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
            Token::from_str(Tokens::If, "if"),
            Token::from_str(Tokens::LParen, "("),
            Token::from_str(Tokens::Int, "5"),
            Token::from_str(Tokens::LessThan, "<"),
            Token::from_str(Tokens::Int, "10"),
            Token::from_str(Tokens::RParen, ")"),
            Token::from_str(Tokens::LBrace, "{"),
            Token::from_str(Tokens::Return, "return"),
            Token::from_str(Tokens::True, "true"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::RBrace, "}"),
            Token::from_str(Tokens::Else, "else"),
            Token::from_str(Tokens::LBrace, "{"),
            Token::from_str(Tokens::Return, "return"),
            Token::from_str(Tokens::False, "false"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::RBrace, "}"),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected in expects {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }

    #[test]
    fn it_can_do_pre_increment_decrement_operators() {
        let input = "x++; --x; x--; ++x";

        let expects = vec![
            Token::from_str(Tokens::Ident, "x"),
            Token::from_str(Tokens::Increment, "++"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Decrement, "--"),
            Token::from_str(Tokens::Ident, "x"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Ident, "x"),
            Token::from_str(Tokens::Decrement, "--"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Increment, "++"),
            Token::from_str(Tokens::Ident, "x"),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected in expects {
            let token = lexer.next_token();
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
            Token::from_str(Tokens::Int, "10"),
            Token::from_str(Tokens::Eq, "=="),
            Token::from_str(Tokens::Int, "10"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Int, "10"),
            Token::from_str(Tokens::NotEq, "!="),
            Token::from_str(Tokens::Int, "9"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::String, "foobar"),
            Token::from_str(Tokens::String, " foo  bar"),
            Token::from_str(Tokens::String, r#"I have nested quotes! "omggg""#),
            Token::from_str(Tokens::LBracket, "["),
            Token::from_str(Tokens::Int, "1"),
            Token::from_str(Tokens::Comma, ","),
            Token::from_str(Tokens::Int, "2"),
            Token::from_str(Tokens::RBracket, "]"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::LBrace, "{"),
            Token::from_str(Tokens::String, "foo"),
            Token::from_str(Tokens::Colon, ":"),
            Token::from_str(Tokens::String, "bar"),
            Token::from_str(Tokens::Comma, ","),
            Token::from_str(Tokens::String, "test"),
            Token::from_str(Tokens::Colon, ":"),
            Token::from_str(Tokens::Int, "1"),
            Token::from_str(Tokens::RBrace, "}"),
            Token::from_str(Tokens::LBrace, "{"),
            Token::from_str(Tokens::Int, "1"),
            Token::from_str(Tokens::Colon, ":"),
            Token::from_str(Tokens::Int, "2"),
            Token::from_str(Tokens::RBrace, "}"),
            Token::from_str(Tokens::Semicolon, ";"),
            Token::from_str(Tokens::Let, "let"),
            Token::from_str(Tokens::Constant, "const"),
            Token::from_str(Tokens::Ident, "x"),
            Token::from_str(Tokens::Assign, "="),
            Token::from_str(Tokens::Int, "5"),
            Token::from_str(Tokens::Eof, "\0"),
        ];

        let mut lexer = Lexer::new(input.to_string());
        for expected in expects {
            let token = lexer.next_token();
            assert_eq!(token, expected);
        }
    }
}