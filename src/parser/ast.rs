//! The abstract syntax tree is built of enums within enums. For instance, a let statement, for example:
//! ```no_run
//! let three = 3;
//! ```
//! This would turn into the tree:
//! ```no_run
//! Let(three,Expr(3))
//! ```
//!
//! The complexity of these trees grow, especially when dealing with function calls.
use crate::token::token::Token;

// TODO: Implement fmt methods for the statements.

/// Statement enums effectively compose the structure of a line of code.
/// These could be considered the roots of an AST, typically with Expr being the children.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let { name: String, value: Expr },
    Define { func_name: String, func: Expr },
    Return { value: Expr },
    Expression(Expr),
}

/// Expressions typically consist of other expressions, and can be considered the children of statements. Any expression type that involves other expressions stores a smart pointer to them.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    String(String),
    Variable(String),
    Boolean(bool),
    Integer(i64),
    Prefix {
        prefix: Prefix,
        value: Box<Expr>,
    },
    Infix {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
    If {
        condition: Box<Expr>,
        consequence: Vec<Statement>,
        alternative: Vec<Statement>,
    },
    While {
        condition: Box<Expr>,
        instruction: Vec<Statement>,
    },
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Call {
        function: Box<Expr>, // Function name
        arguments: Vec<Expr>,
    },

    Builtin {
        function_name: String,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Prefix {
    Bang,
    Minus,
}

#[derive(PartialOrd, PartialEq, Debug, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Equals,
    LessGreater,
    Sum,
    Product,
    Prefix,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Equals,
    NotEquals,
}

pub fn is_builtin(func_name: &str) -> bool {
    match func_name {
        "len" | "quwuit" | "dprint" => true,
        _ => false,
    }
}

impl Token {
    /// The priority system determines whether an expression is evaluated as Infix or Prefix.
    pub fn priority(&self) -> Precedence {
        match self {
            Token::PLUS => Precedence::Sum,
            Token::MINUS => Precedence::Sum,
            Token::SLASH => Precedence::Product,
            Token::ASTERISK => Precedence::Product,
            Token::MOD => Precedence::Product,
            Token::LEQ => Precedence::LessGreater,
            Token::LE => Precedence::LessGreater,
            Token::GEQ => Precedence::LessGreater,
            Token::GR => Precedence::LessGreater,
            Token::EQ => Precedence::Equals,
            Token::NEQ => Precedence::Equals,
            _ => Precedence::Lowest,
        }
    }
}
