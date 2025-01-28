use std::{collections::HashMap, fmt}; // for Display trait; TODO
use tokens::Token;

pub enum ASTNodeType {
    Statement,
    Expression,
    Identifier,
    PostIncrementIdentifier,
    LetStatement,
    ReturnStatement,
    ExpressionStatement,
    PrefixExpression,
    InfixExpression,
    IntegerLiteral,
    BooleanLiteral,
    IfExpression,
    BlockStatement,
    FunctionLiteral,
    CallExpression,
    StringLiteral,
    AssignmentStatement,
    ArrayLiteral,
    IndexExpression,
    ArrayIndexExpression,
    HashLiteral,
}

pub trait ASTNode {
    fn kind(&self) -> ASTNodeType;
    fn token_literal(&self) -> &str;
}

pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl ASTNode for Identifier {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::Identifier;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub struct Expression {}
impl ASTNode for Expression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::Expression;
    }

    fn token_literal(&self) -> &str {
        unimplemented!();
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Expression")
    }
}

pub struct Statement {}
impl ASTNode for Statement {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::Statement;
    }

    fn token_literal(&self) -> &str {
        unimplemented!();
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Statement")
    }
}

pub struct PostIncrementIdentifier {
    pub token: Token,
    pub value: String,
    pub operator: Token,
}

impl ASTNode for PostIncrementIdentifier {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::PostIncrementIdentifier;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for PostIncrementIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value, self.operator.literal)
    }
}

pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
    pub constant: bool,
}

impl ASTNode for LetStatement {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::LetStatement;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} = {};",
            if self.constant { "let const" } else { "let" },
            self.name,
            self.value
        )
    }
}

pub struct ReturnStatement {
    pub token: Token,
    pub return_value: Expression,
}

impl ASTNode for ReturnStatement {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::ReturnStatement;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {};", self.token_literal(), self.return_value)
    }
}

pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Expression,
}

impl ASTNode for ExpressionStatement {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::ExpressionStatement;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for ExpressionStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expression)
    }
}

pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Expression,
}

impl ASTNode for PrefixExpression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::PrefixExpression;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for PrefixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator, self.right)
    }
}

pub struct InfixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Expression,
    pub left: Expression,
}

impl ASTNode for InfixExpression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::InfixExpression;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for InfixExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl ASTNode for IntegerLiteral {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::IntegerLiteral;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

pub struct BooleanLiteral {
    pub token: Token,
    pub value: bool,
}

impl ASTNode for BooleanLiteral {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::BooleanLiteral;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for BooleanLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

pub struct IfExpression {
    pub token: Token,
    pub condition: Expression,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

impl ASTNode for IfExpression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::IfExpression;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for IfExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Start with the "if" condition
        write!(f, "if {} {{ {}", self.condition, self.consequence)?;

        // Add the alternative block if it exists
        if let Some(alternative) = &self.alternative {
            write!(f, " }} else {{ {}", alternative)?;
        }

        // Close the block
        write!(f, " }}")
    }
}

pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl ASTNode for BlockStatement {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::BlockStatement;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for BlockStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();

        for stmt in &self.statements {
            out.push_str(&stmt.to_string());
        }

        write!(f, "{}", out)
    }
}

pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl ASTNode for FunctionLiteral {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::FunctionLiteral;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut params = String::new();

        for param in &self.parameters {
            params.push_str(&param.to_string());
        }

        write!(f, "{}({}) {{{}}}", self.token.literal, params, self.body)
    }
}

pub struct CallExpression {
    pub token: Token,
    pub function: Expression,
    pub arguments: Option<Vec<Expression>>,
}

impl ASTNode for CallExpression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::CallExpression;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for CallExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut args = String::new();

        if let Some(arguments) = &self.arguments {
            for arg in arguments {
                args.push_str(&arg.to_string());
            }
        }

        write!(f, "{}({})", self.function, args)
    }
}

pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl ASTNode for StringLiteral {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::StringLiteral;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for StringLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.literal)
    }
}

pub struct AssignmentStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Expression,
}

impl ASTNode for AssignmentStatement {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::AssignmentStatement;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for AssignmentStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name, self.value)
    }
}

pub struct ArrayLiteral {
    pub token: Token,
    pub elements: Vec<Expression>,
}

impl ASTNode for ArrayLiteral {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::ArrayLiteral;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for ArrayLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut elements = String::new();

        for elm in &self.elements {
            elements.push_str(&elm.to_string());
        }

        write!(f, "[{}]", elements)
    }
}

pub struct IndexExpression {
    pub token: Token,
    pub left: Expression,
    pub index: Expression,
}

impl ASTNode for IndexExpression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::IndexExpression;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}[{}])", self.left, self.index)
    }
}

pub struct ArrayIndexExpression {
    pub token: Token,
    pub array: Expression,
    pub index: Expression,
    pub value: Expression,
}

impl ASTNode for ArrayIndexExpression {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::ArrayIndexExpression;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for ArrayIndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}[{}] = {})", self.array, self.index, self.value)
    }
}

pub struct HashLiteral {
    pub token: Token,
    pub pairs: HashMap<Expression, Expression>,
}

impl ASTNode for HashLiteral {
    fn kind(&self) -> ASTNodeType {
        return ASTNodeType::HashLiteral;
    }

    fn token_literal(&self) -> &str {
        return &self.token.literal;
    }
}

impl fmt::Display for HashLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", "undone")
    }
}
