use std::{collections::HashMap, io::Write};

use crate::{ast::Operator, functions::get_function, instruction::{Function, Instruction, Value}};

pub enum VMError {
    InvalidBytecode,
    ErrString(String)
}

pub struct VM<'a> {
    instructions: Vec<Instruction<'a>>,
    stack: Vec<Value>,
    pc: usize,
    pub(crate) outputs: Vec<Value>,
    symbols: HashMap<&'a str, Value>,
}

impl<'a> VM<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>) -> Self {
        Self {
            pc: 0,
            stack: vec![],
            outputs: vec![],
            symbols: HashMap::new(),
            instructions,
        }
    }

    pub fn new_with_symbols(instructions: Vec<Instruction<'a>>, symbols: HashMap<&'a str, Value>) -> Self {
        Self {
            pc: 0,
            stack: vec![],
            outputs: vec![],
            symbols,
            instructions,
        }
    }

    pub fn print_output(&self) {
        if self.outputs.len() > 0 {
            println!("Results: {}", self.outputs.iter().map(|value| format!("{value}")).collect::<Vec<_>>().join(", "));
        } else {
            println!("No results for this expression");
        }
    }

    pub fn execute_all(&mut self) {
        // Don't run code that is empty or invalid
        if self.instructions.len() == 0 || self.instructions[0] == Instruction::CompileError {
            return;
        }

        while self.pc < self.instructions.len() {
            match self.execute_next() {
                Ok(_) => (),
                Err(error) => {
                    // Stop the vm since a runtime error has occured.
                    self.pc = self.instructions.len();
                    match error {
                        VMError::InvalidBytecode => println!("[RUNTIME ERROR]: The bytecode provided to the VM appears to be invalid, or containing a bug that causes the program to unexpectedly crash"),
                        VMError::ErrString(string) => println!("[RUNTIME ERROR]: {string}"),
                    }
                }
            };
        }
    }

    pub fn execute_next(&mut self) -> Result<(), VMError> {
        self.pc += 1;
        match &self.instructions[self.pc - 1] {
            Instruction::Load { value } => self.stack.push(value.clone()),

            Instruction::Binary { operator } => {
                let rhs = match self.stack.pop() {
                    Some(value) => value,
                    None => return Err(VMError::InvalidBytecode),
                };

                let lhs = match self.stack.pop() {
                    Some(value) => value,
                    None => return Err(VMError::InvalidBytecode),
                };

                match (lhs, rhs) {
                    (Value::Number(a), Value::Number(b)) => {
                        let res = match operator {
                            Operator::Plus => a + b,
                            Operator::Minus => a - b,
                            Operator::Multiply => a * b,
                            Operator::Modulo => {
                                if b == 0.0 {
                                    return Err(VMError::ErrString(format!("Cannot take the modulus of a number by 0!")));
                                }
                                a % b
                            },
                            Operator::Divide => {
                                if b == 0.0 {
                                    return Err(VMError::ErrString(format!("Cannot divide a number by zero!")));
                                }
                                a / b
                            },
                            Operator::Exponent => a.powf(b),
                            Operator::BitAnd => (a as usize & b as usize) as f64,
                            Operator::BitOr => (a as usize | b as usize) as f64,
                            Operator::BitXor => (a as usize ^ b as usize) as f64,
                            Operator::BitLeftShift => ((a as usize) << (b as usize)) as f64,
                            Operator::BitRightShift => ((a as usize) >> (b as usize)) as f64,
                            _ => unimplemented!()
                        };
                        self.stack.push(Value::Number(res));
                    }

                    (Value::String(a), Value::String(b)) => {
                        let res = match operator {
                            Operator::Plus => {
                                let mut base = a;
                                base.push_str(&b);
                                base
                            },
                            _ => return Err(VMError::ErrString(format!("Cannot perform binary operation `{operator}` on strings!")))
                        };
                        self.stack.push(Value::String(res));
                    }

                    (a, b) => {
                        return Err(
                            VMError::ErrString(
                                format!(
                                    "Cannot perform binary operation `{operator}` on {} types: lhs `{}` and rhs `{}`!",
                                    if a.type_of() != b.type_of() { "mismatched" } else { "incompatible" },
                                    a.type_of(), b.type_of()
                                )
                            )
                        );
                    }
                }
            },

            Instruction::Unary { operator } => {
                let rhs = match self.stack.pop() {
                    Some(Value::Number(number)) => number,
                    Some(_) => return Err(VMError::ErrString(format!("Cannot perform unary operations on non numerical values"))),
                    None => return Err(VMError::InvalidBytecode),
                };

                let result = match operator {
                    Operator::Plus => rhs,
                    Operator::Minus => -rhs,
                    _ => return Err(VMError::ErrString(format!("Unable to perform unary operation {operator} on a number!"))),
                };
                self.stack.push(Value::Number(result))
            }

            Instruction::Output => {
                let res = match self.stack.pop() {
                    Some(res) => res,
                    None => return Err(VMError::InvalidBytecode), 
                };
                self.outputs.push(res)
            },

            Instruction::LoadSymbolName { name } => {
                self.symbols.insert(name, Value::Null);
                self.stack.push(Value::Null);
            },

            Instruction::LoadSymbol { name } => {
                let value = match self.stack.pop() {
                    Some(res) => res,
                    None => return Err(VMError::InvalidBytecode), 
                };
                self.stack.push(value.clone());
                self.symbols.insert(name, value);
            },

            Instruction::CallSymbol { name } => {
                match self.symbols.get(name) {
                    Some(value) => self.stack.push(value.clone()),
        
                    None => return Err(VMError::ErrString(format!("The variable `{name}` does not exist!"))),
                }
            },

            Instruction::ReloadSymbol { name } => {
                match self.symbols.get_mut(name) {
                    Some(value) => {
                        let new_value = match self.stack.pop() {
                            Some(res) => res,
                            None => return Err(VMError::InvalidBytecode), 
                        };        
                        *value = new_value;
                        self.stack.push(value.clone());
                    },
                    None => {
                        self.stack.push(Value::Null);
                        return Err(VMError::ErrString(format!("Cannot assign a value to variable {name} because it does not exist!")))
                    },
                }
                
            },

            Instruction::ReloadSymbolOp { name } => {
                let operator = match &self.instructions[self.pc] {
                    Instruction::OData { operator } => operator,
                    _ => return Err(VMError::InvalidBytecode),
                };

                self.pc += 1;

                match self.symbols.get_mut(name) {
                    Some(value) => {
                        let new_value = match self.stack.pop() {
                            Some(res) => res,
                            None => return Err(VMError::InvalidBytecode), 
                        };

                        match (new_value, value) {
                            (Value::Number(a), Value::Number(b)) => {
                                match operator {
                                    Operator::PlusEqual => *b += a,
                                    Operator::MinusEqual => *b -= a,
                                    Operator::DivideEqual => {
                                        if a == 0.0 {
                                            return Err(VMError::ErrString(format!("Cannot divide a number by 0!")));
                                        }
                                        *b /= a
                                    },
                                    Operator::MultiplyEqual => *b *= a,
                                    Operator::ModuloEqual => {
                                        if a == 0.0 {
                                            return Err(VMError::ErrString(format!("Cannot take the modulus of a number by 0!")));
                                        }
                                        *b %= a
                                    },
                                    Operator::ExponentEqual => *b = f64::powf(*b, a),
                                    Operator::BitOrEqual => *b = (*b as usize | a as usize) as f64,
                                    Operator::BitAndEqual => *b = (*b as usize & a as usize) as f64,
                                    Operator::BitXorEqual => *b = (*b as usize ^ a as usize) as f64,
                                    Operator::BitLeftShiftEqual => *b = ((*b as usize) << a as usize) as f64,
                                    Operator::BitRightShiftEqual => *b = (*b as usize >> a as usize) as f64,
                                    
                                    // No other operation can ever reach here
                                    _ => unimplemented!()
                                };

                                self.stack.push(Value::Number(*b));
                            }

                            (Value::String(a), Value::String(b)) => {
                                match operator {
                                    Operator::PlusEqual => b.push_str(a.as_str()),
        
                                    _ => return Err(VMError::ErrString(format!("Cannot perform operation `{operator}` on strings!"))),
                                };

                                self.stack.push(Value::String(b.clone()));
                            }

                            (new_value, value) => {
                                return Err(
                                            VMError::ErrString(
                                                format!(
                                                    "Cannot perform operation `{operator}` on {} types: lhs `{}` and rhs `{}`!",
                                                    if value.type_of() != new_value.type_of() { "mismatched" } else { "incompatible" },
                                                    value.type_of(), new_value.type_of()
                                                )
                                            )
                                        );
                            }
                        }

                    },
                    None => return Err(VMError::ErrString(format!("Cannot find variable {name} to change its value!"))),
                }
            },

            // Really slow?
            Instruction::FunctionCall { name, len } => {
                let mut arguments = vec![];
                match get_function(name.unwrap_or("")) {
                    Ok((length, function)) => {
                        for _ in 0..length {
                            let arg = match self.stack.pop() {
                                Some(value) => {
                                    match value {
                                        Value::Number(num) => num,
                                        _ => return Err(VMError::ErrString(format!("Functions that do not deal with values other than numbers are not yet supported!"))),
                                    }
                                },
                                None => return Err(VMError::ErrString(format!("Failed to get arguments to function {} (Likely an internal error)!", name.unwrap()))),
                            };
                            arguments.push(arg);
                        }
                        self.stack.push(Value::Number(function(arguments.as_slice())))        
                    },

                    // Look for function in function symbols
                    Err(..) => {
                        let function = match self.symbols.get(name.unwrap_or("")) {
                            Some(Value::Function(function)) => Some(function.clone()),
                            None => {
                                // This can have improved performance, probably
                                let last = self.stack.last().cloned();
                                if let Some(Value::Function(function)) = last {
                                    self.stack.pop();
                                    Some(function)
                                } else {
                                    return if let None = name {
                                        Err(VMError::ErrString(format!("Cannot call a non function!")))
                                    } else {
                                        Err(VMError::ErrString(format!("The function `{}` does not exist!", name.unwrap_or("<DIRECT_CALL>"))))
                                    };
                                }
                            }
                            _ => return Err(VMError::ErrString(format!("The function `{}` does not exist!", name.unwrap_or("<DIRECT_CALL>")))),
                        };
                        if let Some(function) = function {
                            let fn_args_address = function.instructions.start - function.arguments;
                            let fn_body_address = function.instructions.start;
                            let fn_body_end = function.instructions.end;

                            let args_len = fn_body_address - fn_args_address;

                            // This is a partial function call
                            if *len + function.is_partial.len() < args_len {
                                match self.symbols.get(name.unwrap_or("")) {
                                    // A partial function call behind a variable reference
                                    // Ex: let a = func(5); a(10):
                                    Some(Value::Function(function)) => {
                                        let mut function = function.clone();
                                        for _ in 0..*len {
                                            function.is_partial.push(self.stack.pop().unwrap());
                                        }
                                        self.stack.push(Value::Function(function));
                                    }

                                    _ => {
                                        // A direct partial function call
                                        // Ex: let a = func(5)(10); a:
                                        if let None = name {
                                            let mut function = function.clone();
                                            for _ in 0..*len {
                                                function.is_partial.push(self.stack.pop().unwrap());
                                            }
                                            self.stack.push(Value::Function(function));    
                                        } else {
                                            return Err(VMError::ErrString(format!("The partially called function `{}` does not exist!", name.unwrap())))
                                        }
                                    },
                                };                
                            } 
                            // Too many arguments provided to function
                            else if  *len + function.is_partial.len() > args_len {
                                return Err(VMError::ErrString(format!("The function `{}` was provided too many arguments! Expected: {args_len}, got: {} argument(s)", name.unwrap_or("<PARTIAL_FUNCTION>"), *len + function.is_partial.len())))
                            }
                            else {
                                let orig_pc = self.pc;
                                let orig_symbols = self.symbols.clone();
                                self.pc = fn_args_address;
                                let mut start = 0;
                                let mut predefined_args = function.is_partial.clone();
                                let offset = function.is_partial.len();
    
                                for i in (0..args_len).rev() {
                                    let arg = &self.instructions[self.pc + i];
                                    match arg {
                                        Instruction::ArgumentName { name } => {
                                            if args_len - start > offset {
                                                let value = match self.stack.pop() {
                                                    Some(value) => value,
                                                    None => return Err(VMError::InvalidBytecode),
                                                };
                                                self.symbols.insert(name, value);
                                                start += 1; 
                                            } else {
                                                self.symbols.insert(name, predefined_args.pop().unwrap());
                                            }
                                        }
    
                                        _ => return Err(VMError::InvalidBytecode),
                                    }
                                }
                                
                                self.pc = fn_body_address;
                                
                                for _ in 0..(fn_body_end - fn_body_address) {
                                    self.execute_next()?;
                                    // A stack overflow used to occur when a function declaration was used during initializaion of another 
                                    // function like `let a _ = (let b _ = 1); a()();`, This would cause a stack overflow due to the function 
                                    // escaping it's bounds and calling `a` again
                                    if self.pc >= fn_body_end { break; }
                                }
    
                                self.pc = orig_pc;
                                self.symbols = orig_symbols;    
                            }
                        }
                    } 
                };
            }

            Instruction::PartialCall { name, len } => {
                match self.symbols.get(name) {
                    Some(Value::Function(function)) => {
                        let mut function = function.clone();
                        for _ in 0..*len {
                            function.is_partial.push(self.stack.pop().unwrap());
                        }
                        self.stack.push(Value::Function(function));
                    }

                    _ => (),
                };
            }

            Instruction::Null => self.stack.push(Value::Null),

            Instruction::Delete { name } => {
                // Remove every symbol related to the name
                self.symbols.remove(name);

                // This is checked when parsing
                // if let Ok(..) = get_function(name) {
                //     return Err(VMError::ErrString(format!("Cannot delete builtin function `{name}`")));
                // }
                self.stack.push(Value::Null);
            }

            Instruction::Array { len } => {
                let mut array = Vec::with_capacity(*len);
                array.extend(self.stack.drain((self.stack.len() - *len)..));
                self.stack.push(Value::Array(array));
            }

            Instruction::FunctionDecl { name } => {
                let args = match self.instructions[self.pc] {
                    Instruction::UData { number } => number,
                    _ => return Err(VMError::InvalidBytecode),
                };
                self.pc += 1;

                let end = match self.instructions[self.pc] {
                    Instruction::UData { number } => number,
                    _ => return Err(VMError::InvalidBytecode),
                };
                self.pc += 1;
                let fn_body_address = self.pc + args;
                let fn_body_end = self.pc + end;
                self.pc += end;
                self.symbols.insert(name, Value::Function(Function::new(args, fn_body_address..fn_body_end)));
                self.stack.push(Value::Function(Function::new(args, fn_body_address..fn_body_end)));
            }

            Instruction::Print { depth } => {
                let end = self.stack.len();
                let drained = self.stack.drain((end - depth)..(end));
                std::io::stdout().flush().ok();
                for value in drained {
                    print!("{value} ");
                }
                println!();
                self.stack.push(Value::Null);
            }

            Instruction::TypeOf => {
                let string = self.stack.pop().unwrap().type_of().to_owned();
                self.stack.push(Value::String(string));
            },

            Instruction::Index => {
                let expression = self.stack.pop().unwrap();
                let to_index = self.stack.pop().unwrap();
                match (to_index, expression) {
                    (Value::Array(array), Value::Number(number)) => {
                        let index = number as usize;
                        if array.len() >= index + 1 {
                            self.stack.push(array[index].clone());
                        } else {
                            return Err(VMError::ErrString(format!("Indexing out of bounds of the array!")));
                        }
                    }

                    (a, b) => return Err(VMError::ErrString(format!("Unable to index type {} by type {}", a.type_of(), b.type_of()))),
                };
            },
            
            // Change array values
            Instruction::ReloadIndex { name, depth, operator } => {

                let value = 
                    match self.stack.pop() {
                        Some(value) => value,
                        None => return Err(VMError::InvalidBytecode),
                    };

                if let Some(item) = self.symbols.get_mut(name) {
                    let mut item = item;
                    for _ in 0..*depth {
                        let index = match self.stack.pop() {
                            Some(Value::Number(value)) => value as usize,
                            Some(a) => return Err(VMError::ErrString(format!("Cannot index an array by type {}!", a.type_of()))),
                            None => return Err(VMError::InvalidBytecode),
                        };
                        if let Value::Array(inside_item) = item {
                            match inside_item.get_mut(index) {
                                Some(inside_item) => item = inside_item,
                                None =>
                                    return Err(VMError::ErrString(format!("Indexing out of bounds of array `{name}`!")))
                            }
                        } else {
                            return Err(VMError::ErrString(format!("Indexing out of bounds of array `{name}`!")))
                        }
                    }

                    match (&mut item, value) {
                        (Value::Number(a), Value::Number(b)) => {
                            match operator {
                                Operator::Equal => *a = b,
                                Operator::PlusEqual => *a += b,
                                Operator::MinusEqual => *a -= b,
                                Operator::MultiplyEqual => *a *= b,
                                Operator::DivideEqual => {
                                    if b == 0.0 {
                                        return Err(VMError::ErrString(format!("Cannot divide a number by 0!")));
                                    }
                                    *a /= b
                                },
                                Operator::ModuloEqual => {
                                    if b == 0.0 {
                                        return Err(VMError::ErrString(format!("Cannot take the modulus of a number by 0!")));
                                    }
                                    *a %= b
                                },
                                Operator::ExponentEqual => *a = a.powf(b),
                                Operator::BitOrEqual => *a = (*a as usize | (b as usize)) as f64,
                                Operator::BitAndEqual => *a = (*a as usize & (b as usize)) as f64,
                                Operator::BitXorEqual => *a = (*a as usize ^ (b as usize)) as f64,
                                Operator::BitLeftShiftEqual => *a = ((*a as usize) << (b as usize)) as f64,
                                Operator::BitRightShiftEqual => *a = ((*a as usize) >> (b as usize)) as f64,

                                _ => unimplemented!()
                            }
                        },

                        (Value::String(a), Value::String(b)) => {
                            match operator {
                                Operator::Equal => *a = b,
                                Operator::PlusEqual => *a = format!("{a}{b}"),
                                _ => return Err(VMError::ErrString(format!("Cannot perform operation `{operator}` on strings!")))
                            }
                        }

                        (a, b) => {
                            match operator {
                                Operator::Equal => **a = b,
                                _ => return Err(VMError::ErrString(format!("Cannot perform operation {operator} on incompatible types {} and {}!", a.type_of(), b.type_of())))
                            }
                        }
                    }
                    self.stack.push(item.clone());
                } else {
                    return Err(VMError::ErrString(format!("The variable `{name}` of type `{}` does not exist!", "{Array}")))
                }
                
            }

            instruction => 
                return 
                    Err(
                        VMError::ErrString(
                            format!(
                                "Unexpected instruction {instruction:?} found at {}. This is most probably an error produced by a bug in the bytecode.", 
                                self.pc - 1
                            )
                        )
                    ),
        };
        Ok(())
    }

    pub fn get_symbols(self) -> HashMap<&'a str, Value> {
        self.symbols
    }
}