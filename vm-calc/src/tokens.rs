use std::fmt::{Debug, Display};

use crate::utils::Span;

#[derive(Debug, PartialEq, Clone)]
pub enum NumberType {
    /// 0b1001
    Binary,
    /// 1, 1.001, 0.0000001, .1111, 1.
    Real,
    /// 0o1113
    Octal,
    /// 0xFFF
    Hex,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // Basic elements
    /// Numbers of any type
    Number {
        number_type: NumberType,
    },
    /// Variables or functions
    Identifier,

    /// A string: "abc", "\n " etc.
    String,

    // Delimiters
    /// Semicolon to seperate statements
    Semicolon,
    /// This is the alternative to a semicolon, that allows that expression to be shown in the output 
    Colon,
    /// Comma, to seperate expressions
    Comma,
    /// (
    OpeningBracket,
    /// (
    ClosingBracket,

    /// [
    OpenArray,
    /// ]
    CloseArray,

    // Operators
    /// *
    Multiply,
    /// *=
    MultiplyEqual,

    /// %
    Modulo,
    /// %=
    ModuloEqual,
    
    /// /
    Divide,
    /// /=
    DivideEqual,    

    /// +
    Add,
    /// +=
    AddEqual,
    
    /// -
    Subtract,
    /// -=
    SubtractEqual,

    /// **
    Exponent,
    /// **=
    ExponentEqual,

    /// ^
    BitXor,
    /// ^=
    BitXorEqual,

    /// &
    BitAnd,
    /// &=
    BitAndEqual,

    /// |
    BitOr,
    /// |=
    BitOrEqual,

    /// <<
    BitLeftShift,
    /// <<=
    BitLeftShiftEqual,

    /// >>
    BitRightShift,
    /// >>=
    BitRightShiftEqual,

    /// =
    Equal,

    /// Null token for the parser
    /// Could have used an Option, but too lazy
    Null,

    /// End Of File
    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::Number { number_type } => {
                match number_type {
                    NumberType::Binary => "Binary Number",
                    NumberType::Hex => "Hexadecimal Number",
                    NumberType::Octal => "Octal Number",
                    NumberType::Real => "Real Number",
                }
            },
            Self::Identifier => "Identifier",
            Self::String => "String",
            Self::Semicolon => ";",
            Self::Colon => ":",
            Self::Comma => ",",
            Self::OpeningBracket => "(",
            Self::ClosingBracket => ")",
            Self::OpenArray => "[",
            Self::CloseArray => "]",
            Self::Multiply => "*",
            Self::MultiplyEqual => "*=",
            Self::Modulo => "%",
            Self::ModuloEqual => "%=",
            Self::Divide => "/",
            Self::DivideEqual => "/=",
            Self::Add => "+",
            Self::AddEqual => "+=",
            Self::Subtract => "-",
            Self::SubtractEqual => "-=",
            Self::Exponent => "**",
            Self::ExponentEqual => "**=",
            Self::BitXor => "^",
            Self::BitXorEqual => "^=",
            Self::BitAnd => "&",
            Self::BitAndEqual => "&=",
            Self::BitOr => "|",
            Self::BitOrEqual => "|=",
            Self::BitLeftShift => "<<",
            Self::BitLeftShiftEqual => "<<=",
            Self::BitRightShift => ">>",
            Self::BitRightShiftEqual => ">>=",
            Self::Equal => "=",
            Self::EOF => "End Of File",
            Self::Null => "Null token. A bug has occured if this has been presented to the output.",
        };
        write!(f, "{string}")
    }
}


#[derive(PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }

    pub fn null() -> Self {
        Self {
            token_type: TokenType::Null,
            span: Span::null(),
        }
    }

    pub fn eof(end: usize) -> Self {
        Self {
            token_type: TokenType::EOF,
            span: Span::new(end, end),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} type @ {}", self.token_type, self.span)
    }
}