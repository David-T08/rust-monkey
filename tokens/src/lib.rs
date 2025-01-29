use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    Ident,
    Int,

    Assign,
    Plus,
    Minus,
    Mult,
    Div,
    Bang,

    LessThan,
    GreaterThan,

    Comma,
    Semicolon,
    Colon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    LBracket,
    RBracket,

    For,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Constant,

    Eq,
    NotEq,

    Increment,
    Decrement,

    String,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "EOF",
            TokenType::Ident => "Identifier",
            TokenType::Int => "Integer",
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Mult => "*",
            TokenType::Div => "/",
            TokenType::Bang => "!",
            TokenType::LessThan => "<",
            TokenType::GreaterThan => ">",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Colon => ":",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",
            TokenType::For => "for",
            TokenType::Function => "function",
            TokenType::Let => "let",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Return => "return",
            TokenType::Constant => "constant",
            TokenType::Eq => "==",
            TokenType::NotEq => "!=",
            TokenType::Increment => "++",
            TokenType::Decrement => "--",
            TokenType::String => "string",
        };

        write!(f, "{}", token_str)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub literal: &'a str,
}

impl Token<'_> {
    pub fn new<'a>(token_type: TokenType, literal: &'a str) -> Token<'a> {
        return Token {
            token_type,
            literal: literal,
        };
    }
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.literal)
    }
}

pub fn lookup_keyword(ident: &str) -> TokenType {
    return match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        "for" => TokenType::For,
        "const" => TokenType::Constant,
        _ => TokenType::Ident, // Should never occur
    };
}
