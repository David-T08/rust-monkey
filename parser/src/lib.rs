use ast as AST;
use lexer::Lexer;
use std::{any::Any, mem::replace};
use tokens::{Token, TokenType};

use std::collections::HashMap;

enum Constants {
    LOWEST,
    EQUALS,
    LESSGREATER,
    SUM,
    PRODUCT,
    PREFIX,
    CALL,
    INDEX,
}

type PrefixParseFn = fn(&mut Parser) -> AST::Expression;
type InfixParseFn = fn(&mut Parser, expr: AST::Expression) -> AST::Expression;

fn get_precedence(token: &TokenType) -> Constants {
    return match token {
        TokenType::Eq => Constants::EQUALS,
        TokenType::NotEq => Constants::EQUALS,

        TokenType::GreaterThan => Constants::LESSGREATER,
        TokenType::LessThan => Constants::LESSGREATER,

        TokenType::Plus => Constants::SUM,
        TokenType::Minus => Constants::SUM,

        TokenType::Div => Constants::PRODUCT,
        TokenType::Mult => Constants::PRODUCT,

        TokenType::LParen => Constants::CALL,
        TokenType::LBracket => Constants::INDEX,

        _ => Constants::LOWEST,
    };
}

pub struct Parser {
    lexer: Lexer,
    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,

    infix_parse_fns: HashMap<TokenType, InfixParseFn>,
    prefix_parse_fns: HashMap<TokenType, PrefixParseFn>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Parser {
        let cur_token = lexer.next_token();
        let peek_token = lexer.next_token();

        let parser = Parser {
            lexer,
            errors: Vec::new(),

            cur_token,
            peek_token,

            infix_parse_fns: HashMap::new(),
            prefix_parse_fns: HashMap::new(),
        };

        return parser;
    }

    // Logging
    fn peek_err(&mut self, expected_type: &TokenType) {
        self.errors.push(format!(
            "Expected next token to be {}, got {}",
            expected_type, self.peek_token.token_type
        ));
    }

    fn no_prefix_parse_fn_err(&mut self, expected_type: &TokenType) {
        self.errors.push(format!(
            "No prefix parse function for {} found",
            expected_type
        ));
    }

    // Utility
    fn expect_peek(&mut self, token: &Token) -> bool {
        if self.peek_token.token_type == token.token_type {
            self.next_token();
            return true;
        } else {
            self.peek_err(&token.token_type);
            return false;
        }
    }

    fn peek_precedence(&self) -> Constants {
        return get_precedence(&self.peek_token.token_type);
    }

    fn cur_precedence(&self) -> Constants {
        return get_precedence(&self.cur_token.token_type);
    }

    fn next_token(&mut self) {
        self.cur_token = replace(&mut self.peek_token, self.lexer.next_token());
    }

		// Registering
    fn register_prefix(&mut self, token_type: TokenType, fn_: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_type, fn_);
    }

		fn register_infix(&mut self, token_type: TokenType, fn_: InfixParseFn) {
				self.infix_parse_fns.insert(token_type, fn_);
		}

    // Parsers
    fn parse_boolean(&mut self) -> AST::BooleanLiteral {
        return AST::BooleanLiteral::new(
            self.cur_token.clone(),
            self.cur_token.token_type == TokenType::True,
        );
    }

		fn parse_string_literal(&mut self) -> AST::StringLiteral {
				let clone = self.cur_token.clone();
				return AST::StringLiteral::new(
						clone,
						&clone.literal
				)
		}
}
