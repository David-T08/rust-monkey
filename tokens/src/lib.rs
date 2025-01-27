#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Tokens {
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

		String
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: Tokens,
    pub literal: String
}

impl Token {
	pub fn from_string(token_type: Tokens, literal: String) -> Token {
		return Token { token_type, literal };
	}

	pub fn from_str(token_type: Tokens, literal: &'static str) -> Token {
			return Token { token_type, literal: literal.to_string()}
	}
}

pub fn lookup_keyword(ident: &str) -> Tokens {
	return match ident {
		"fn" => Tokens::Function,
		"let" => Tokens::Let,
		"true" => Tokens::True,
		"false" => Tokens::False,
		"if" => Tokens::If,
		"else" => Tokens::Else,
		"return" => Tokens::Return,
		"for" => Tokens::For,
		"const" => Tokens::Constant,
		_ => Tokens::Ident // Should never occur
	}
}