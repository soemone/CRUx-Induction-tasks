use std::fmt::Display;

use crate::utils::Span;

/// T: Tokenizer, P: Parser
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// An error that occurs when an octal, binary or hexadecimal number provided is incomplete - `0x` `0b` `0o`
    TNumberExpected { location: usize },

    /// Invalid octal numbers; Octal numbers with digits 8, or 9
    TInvalidOctal { span: Span },

    /// Invalid binary numbers; Binary numbers with digits 2 to 9
    TInvalidBinary { span: Span },

    /// An error that occurs when an invalid character is encountered
    TInvalidCharacter { location: usize },

    /// An incomplete, or non terminated string
    TIncompleteString { span: Span }, 

    /// End of file
    TEOF,

    /// Invalid expression / statement
    PInvalidStatement {
        message: String,
        span: Span,
    },

    /// Any parser error (Also includes one error generated during bytecode generation)
    PError {
        message: String,
        span: Span,
    },

    /// An internal, or unexpected error
    PInternalError {
        message: String,
        span: Span,
    },

    /// An empty file
    NoResult
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            Self::TNumberExpected { location } => format!("[TOKENIZER ERROR] [{location}]: Expected number!"),

            Self::TInvalidCharacter { location } => format!("[TOKENIZER ERROR] [{location}]: Found invalid character! Help: Remove this character"),

            Self::TInvalidBinary { span } => format!("[TOKENIZER ERROR] {span}: Invalid Binary number! Help: Binary numbers can only contain the digits 0 or 1"),

            Self::TInvalidOctal { span } => format!("[TOKENIZER ERROR] {span}: Invalid Octal number! Help: Octal numbers can only contain the digits 0 to 7"),

            Self::TIncompleteString { span } => format!("[TOKENIZER ERROR] {span}: Incomplete string! Help: Complete this string by inserting a `\"` at the end of it"),

            // Usually when the file is empty and an early EOF has been produced
            Self::NoResult => format!(""),

            Self::TEOF => format!("End of file reached. No new tokens can be generated"),

            Self::PError { message, span } | Self::PInvalidStatement { message, span } => {
                format!("[PARSE ERROR] {span}: {message}")
            },

            Self::PInternalError { message, span: _ } => format!("[INTERNAL PARSE ERROR]: {message}"),
        };
        write!(f, "{string}")
    }
}