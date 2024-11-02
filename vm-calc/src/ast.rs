use std::{fmt::Display, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::{tokens::TokenType, utils::Span};

// This is pretty redundant, and could probably be phased out for just using `TokenType`
/// An operator
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    // Arithmetic operators
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Divide,
    DivideEqual,
    Multiply,
    MultiplyEqual,
    Modulo,
    ModuloEqual,
    Exponent,
    ExponentEqual,

    // Bitwise operators
    BitAnd,
    BitOr,
    BitXor,
    BitLeftShift,
    BitRightShift,
    BitAndEqual,
    BitOrEqual,
    BitXorEqual,
    BitLeftShiftEqual,
    BitRightShiftEqual,

    Equal,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Self::Equal => "=",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Multiply => "*",
            Self::Modulo => "%",
            Self::Divide => "/",
            Self::Exponent => "**",
            Self::BitAnd => "&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::BitRightShift => ">>",
            Self::BitLeftShift => "<<",
            Self::PlusEqual => "+=",
            Self::MinusEqual => "-=",
            Self::DivideEqual => "/=",
            Self::MultiplyEqual => "*=",
            Self::ModuloEqual => "%=",
            Self::ExponentEqual => "**=",
            Self::BitAndEqual => "&=",
            Self::BitOrEqual => "|=",
            Self::BitXorEqual => "^=",
            Self::BitLeftShiftEqual => "<<=",
            Self::BitRightShiftEqual => ">>=",
        };
        write!(f, "{res}")
    }
}

impl From<TokenType> for Operator {    
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::Equal => Self::Equal,
            TokenType::Add => Self::Plus,
            TokenType::AddEqual => Self::Plus,
            TokenType::Subtract => Self::Minus,
            TokenType::SubtractEqual => Self::MinusEqual,
            TokenType::Divide => Self::Divide,
            TokenType::DivideEqual => Self::DivideEqual,
            TokenType::Multiply => Self::Multiply,
            TokenType::MultiplyEqual => Self::MultiplyEqual,
            TokenType::Modulo => Self::Modulo,
            TokenType::ModuloEqual => Self::ModuloEqual,
            TokenType::Exponent => Self::Exponent,
            TokenType::ExponentEqual => Self::ExponentEqual,
            TokenType::BitAnd => Self::BitAnd,
            TokenType::BitXor => Self::BitXor,
            TokenType::BitOr => Self::BitOr,
            TokenType::BitLeftShift => Self::BitLeftShift,
            TokenType::BitRightShift => Self::BitRightShift,

            _ => panic!("A bug has occured when trying to convert `{value:?}` to `Operator`"),
        }
    } 
}

#[derive(Debug, PartialEq)]
pub struct Tree<'a> {
    pub(crate) ast: AST<'a>,
    pub(crate) span: Span,
}

impl<'a> Tree<'a> {
    pub fn new(ast: AST<'a>, span: Span) -> Self {
        Self {
            ast, span,
        }
    }
}

impl Display for Tree<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ast)
    }
}

#[derive(Debug, PartialEq)]
pub enum AST<'a> {
    /// Binary operations applied to numbers, and strings where applicable
    BinaryOp { 
        lhs: Rc<Tree<'a>>,
        rhs: Rc<Tree<'a>>,
        op: Operator,
    },

    /// The operator + / - applied to a number
    UnaryOp { 
        rhs: Rc<Tree<'a>>,
        op: Operator,
    },

    /// The most fundamental type, a number used for all operations
    Number {
        value: f64,
    },

    /// An identifier for variables / functions etc.
    Identifier {
        name: &'a str,
    },

    /// Declaring and assigning a value to a variable in the same expression
    DeclareAssign {
        identifier: &'a str,
        identifier_span: Span,
        value: Rc<Tree<'a>>,
    },

    /// Declaring a variable
    Declare {
        identifier: &'a str,
        identifier_span: Span,
    },

    // These two can probably be combined, like how AssignIndex works
    /// Just a normal assignment for a variable
    Assign {
        identifier: &'a str,
        identifier_span: Span,
        value: Rc<Tree<'a>>,
    },

    /// Assign + operator for a variable
    AssignOp {
        identifier: &'a str,
        identifier_span: Span,
        value: Rc<Tree<'a>>,
        operator: Operator,
    },

    /// Assign data to an array at the given index or indices for a multi-dimensional array
    AssignIndex {
        identifier: &'a str,
        value: Rc<Tree<'a>>,
        operator: Operator,
        indicies: Vec<Rc<Tree<'a>>>,
    },

    /// Tells the VM to push the result to the output
    Output {
        value: Rc<Tree<'a>>,
    },

    /// Calls a function
    FunctionCall {
        name: Rc<Tree<'a>>,
        expressions: Vec<Rc<Tree<'a>>>,
    },

    /// Declare a function
    FunctionDecl {
        name: &'a str,
        arguments: Vec<&'a str>,
        body: Rc<Tree<'a>>,
    },

    /// Partially calls a function
    PartialCall {
        name: &'a str,
        expressions: Vec<Rc<Tree<'a>>>,
    },

    /// Used to delete variables or functions
    Delete {
        name: &'a str,
    },

    /// Invokes the print function
    Print {
        expressions: Vec<Rc<Tree<'a>>>,
    },

    /// A string
    String {
        contents: String,
    },

    /// Used for direct function calls to a variable
    Name {
        value: &'a str,
    },

    /// Get the type of a specific value or a function
    TypeOf {
        expression: Rc<Tree<'a>>,
    },

    /// Create an array
    Array {
        expressions: Vec<Rc<Tree<'a>>>,
    },

    /// Index an array to get the value
    Index {
        to_index: Rc<Tree<'a>>,
        expression: Rc<Tree<'a>>,
    },

    /// A null value
    Null,
}

impl Display for AST<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "{}Null{}", "{", "}"),
            Self::TypeOf { expression } => write!(f, "<TypeOf> ({expression})"),
            Self::Index { to_index, expression } => write!(f, "({to_index}[{expression}])"),
            
            Self::BinaryOp { lhs, rhs, op } => write!(f, "({lhs} {op} {rhs})"),
            Self::UnaryOp { rhs, op } => write!(f, "({op}{rhs})"),
            
            Self::Number { value } => write!(f, "{value}"),
            Self::Output { value } => write!(f, "*{value}*"),
            
            Self::Identifier { name } => write!(f, "{name}"),
            Self::String { contents } => write!(f, "\"{contents}\""),
            Self::Delete { name } => write!(f, "(delete {name})"),

            Self::Assign { identifier, value, .. } => write!(f, "({identifier} = {value})"),
            Self::AssignOp { operator, identifier, value, .. } => write!(f, "({identifier} {operator} {value})"),
            
            Self::AssignIndex { identifier, value, indicies, operator } => {
                let mut arguments = String::new();
                for expr in indicies {
                    arguments = format!("{arguments}[{expr}]");
                }
                write!(f, "({identifier}{arguments} {operator} {value})")
            }
            
            Self::DeclareAssign { identifier, value, .. } => write!(f, "(let {identifier} = {value})"),
            Self::Declare { identifier, identifier_span: _ } =>  write!(f, "(let {identifier})"),
            
            Self::FunctionDecl { name, arguments, body } => write!(f, "(let {name} {} = {body})", arguments.join(" ")),
            Self::FunctionCall { name, expressions } => {
                let mut arguments = String::new();
                for expr in expressions {
                    if arguments.is_empty() {
                        arguments = format!("{expr}");
                    } else {
                        arguments = format!("{arguments}, {expr}");
                    }
                }
                write!(f, "{name}({})", arguments)
            }

            Self::PartialCall { name, expressions } => {
                let mut arguments = String::new();
                for expr in expressions {
                    if arguments.is_empty() {
                        arguments = format!("{expr}");
                    } else {
                        arguments = format!("{arguments}, {expr}");
                    }
                }
                write!(f, "(<PARTIAL> {name}({}))", arguments)
            }


            Self::Print { expressions } => {
                let mut arguments = String::new();
                for expr in expressions {
                    if arguments.is_empty() {
                        arguments = format!("{expr}");
                    } else {
                        arguments = format!("{arguments}, {expr}");
                    }
                }
                write!(f, "<PRINT>({})", arguments)
            },

            Self::Name { value } => {
                write!(f, "{value}")
            },

            Self::Array { expressions } => {
                let mut expression_str = String::new();
                for expr in expressions {
                    if expression_str.is_empty() {
                        expression_str = format!("{expr}");
                    } else {
                        expression_str = format!("{expression_str}, {expr}");
                    }
                }

                write!(f, "[{}]", expression_str)
            }
        }
    }
}