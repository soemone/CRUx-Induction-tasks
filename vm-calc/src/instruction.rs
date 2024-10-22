use std::{ops::Range, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::ast::Operator;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Symbol<'a> {
    Variable(&'a str),
    Function(&'a str),
}

// There most definitely is a better, more efficient way to represent the bytecode, but I cannot think of it
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Number(f64),
    String(String),
    Function(Function),
    PartialFunction(Function, Vec<Value>),
    Array(Vec<Value>),
    Null,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match &self {
            Value::Number(number) => format!("{number}"),
            Value::String(string) => { format!("{}", string) },
            Value::Function(..) => { format!("<FUNCTION>(...)") },
            Value::PartialFunction(..) => { format!("<PARTIAL>(...)") },
            Value::Array(values) => { 
                let mut value_str = String::new();
                for value in values {
                    if value_str.is_empty() {
                        value_str = format!("{value}");
                    } else {
                        value_str = format!("{value_str}, {value}");
                    }
                }
                format!("<Array> [{}]", value_str)
            },
            // WHY?
            Value::Null => format!("{}NULL{}", "{", "}"),
        };
        write!(f, "{res}")
    }
}


impl Value {
    pub fn type_of(&self) -> &str {
        match self {
            Value::Null => "{Null}",
            Value::Number(..) => "{Number}",
            Value::Function(..) => "{Function}",
            Value::PartialFunction(..) => "{PartialFunction}",
            Value::String(..) => "{String}",
            Value::Array(..) => "{Array}",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[repr(align(1))]
pub enum Instruction<'a> {
    /// Load a value into the stack
    Load {
        value: Value,
    },

    /// Perform a binary operation
    Binary {
        operator: Operator,
    },

    /// Perform a unary operation
    Unary {
        operator: Operator,
    },

    /// Create a variable and initialize it with a null value
    LoadSymbolName {
        name: &'a str,
    },

    /// Create a variable and initialize it with a given value
    LoadSymbol {
        name: &'a str,
    },

    /// Change the value of a variable
    ReloadSymbol {
        name: &'a str,
    },

    /// Change the value of a variable
    ReloadSymbolOp {
        name: &'a str,
    },

    /// Invoke the value of a variable
    CallSymbol {
        name: &'a str,
    },

    /// Invoke a function
    FunctionCall {
        name: Option<&'a str>,
        len: usize,
    },

    /// Invoke a function
    PartialCall {
        name: &'a str,
        len: usize,
    },

    FunctionDecl {
        name: &'a str,
    },

    ArgumentName {
        name: &'a str,
    },

    Delete {
        name: &'a str,
    },

    Print {
        depth: usize
    },

    UData { number: usize },

    OData { operator: Operator },

    /// An array
    Array { len: usize, },

    Index,

    /// A null value
    Null,

    /// A keyword to check types
    TypeOf,

    /// Present the result of the previous expression to the terminal
    Output,

    /// A flag to not run the VM when a compiler error has occured
    CompileError,

    /// An illegal instruction
    Illegal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Function {
    pub(crate) arguments: usize,
    pub(crate) instructions: Range<usize>,
    pub(crate) is_partial: Vec<Value>,
}

impl Function {
    pub fn new(arguments: usize, instructions: Range<usize>) -> Self {
        Self { arguments, instructions, is_partial: vec![] }
    }
}