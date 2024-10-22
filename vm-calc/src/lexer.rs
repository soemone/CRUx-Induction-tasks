use std::str::Chars;
use crate::{errors::Error, tokens::{NumberType, Token, TokenType}, utils::Span};

pub struct Lexer<'a> {
    pub(crate) source: &'a str,
    chars: Chars<'a>,
    current: Option<char>,
    position: usize,
}

impl Lexer<'_> {

    pub fn new(source: &'_ str) -> Result<Lexer, ()> {
        let chars = source.chars();
        Ok(Lexer {
            source,
            chars,
            current: None,
            position: 0,
        })
    }

    pub fn next(&mut self) -> Result<Token, Error>{
        let start = self.position;

        macro_rules! token {
            (TokenType::$token_type: ident) => {{
                self.increment();
                Ok(Token::new(TokenType::$token_type, Span::new(start, self.position)))
            }};
        }

        macro_rules! variable_token {
            ($number_of_increments: expr, TokenType::$token_type: ident) => {{
                for _ in 0..$number_of_increments {
                    self.increment();
                }
                Ok(Token::new(TokenType::$token_type, Span::new(start, self.position)))
            }};
        }

        let next =
            match self.chars.clone().next() {
                Some(character) => character,
                None => {
                    return Err(Error::TEOF)
                },
            };

        match next {
            // Skip whitespace
            _ if Self::check_whitespace(next) => {
                self.take_while(Self::check_whitespace);
                return self.next();
            },

            // Numbers
            _ if Self::check_number(next) => {
                if next == '0' {
                    self.increment();
                    let mut cloned_iter = self.chars.clone();
                    match cloned_iter.next() {
                        // Octal
                        Some('o') => {
                            self.increment();
                            let mut has_89 = false;
                            self.take_while(|c| {
                                if matches!(c, '8'..='9') {
                                    has_89 = true;
                                }
                                Self::check_number(c)
                            });
                            let span = Span::new(start, self.position);
                            if self.position - start == 2 {
                                return Err(Error::TNumberExpected { location: self.position });
                            } else if has_89 {
                                return Err(Error::TInvalidOctal { span });
                            }
                            Ok(Token::new(TokenType::Number { number_type: NumberType::Octal }, span))
                        },

                        // Binary
                        Some('b') => {
                            self.increment();
                            let mut has_29 = false;
                            self.take_while(|c| {
                                if matches!(c, '2'..='9') {
                                    has_29 = true;
                                }
                                Self::check_number(c)
                            });                          
                            let span = Span::new(start, self.position);  
                            if self.position - start == 2 {
                                return Err(Error::TNumberExpected { location: self.position });
                            } else if has_29 {
                                return Err(Error::TInvalidBinary { span });
                            }
                            Ok(Token::new(TokenType::Number { number_type: NumberType::Binary }, span))
                        }

                        // Hex
                        Some('x') => {
                            self.increment();
                            self.take_while(Self::check_hex);
                            if self.position - start == 2 {
                                return Err(Error::TNumberExpected { location: self.position });
                            }
                            Ok(Token::new(TokenType::Number { number_type: NumberType::Hex }, Span::new(start, self.position)))
                        }

                        // A number that starts with zero. Why? Why not.
                        Some(character) if Self::check_number(character) => {
                            // Parse number
                            self.take_while(Self::check_number);
                            let mut cloned_iter = self.chars.clone();
                            if let Some('.') = cloned_iter.next() {
                                self.increment();
                            }
                            self.take_while(Self::check_number);
                            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(start, self.position)))
                        }

                        // Floating point expression
                        Some('.') => {
                            self.increment();
                            match cloned_iter.next() {
                                Some(character) if Self::check_number(character) => {
                                    self.take_while(Self::check_number);
                                }
                                _ => ()
                            }
                            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(start, self.position)))
                        }
                        _ => {
                            // Just a zero
                            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(start, self.position)))
                        }
                    }
                } else {
                    // Take the rest of the numbers
                    self.take_while(Self::check_number);
                    let mut cloned_iter = self.chars.clone();
                    if let Some('.') = cloned_iter.next() {
                        self.increment();
                        self.take_while(Self::check_number);
                    }
                    Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(start, self.position)))
                }
            },

            // Leading decimal real numbers ie. `.15`, `.11111`
            _ if '.' == next => {
                self.increment();
                let mut cloned_iter = self.chars.clone();
                match cloned_iter.next() { 
                    Some(character) if Self::check_number(character) => {
                        self.take_while(Self::check_number);
                        Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(start, self.position)))
                    },

                    _ => Err(Error::TInvalidCharacter { location: self.position - 1 })
                }
            }

            // Register identifiers
            _ if Self::check_ident_start(next) => {
                self.take_while(Self::check_ident_continue);
                Ok(Token::new(TokenType::Identifier, Span::new(start, self.position)))
            },

            // Delimiters 
            '(' => token!(TokenType::OpeningBracket),

            ')' => token!(TokenType::ClosingBracket),

            '[' => token!(TokenType::OpenArray),
            
            ']' => token!(TokenType::CloseArray),

            ';' => token!(TokenType::Semicolon),
            
            ':' => token!(TokenType::Colon),

            ',' => token!(TokenType::Comma),

            // Operators
            '=' => token!(TokenType::Equal),

            '+' => {
                self.increment();
                match self.peek() {
                    Ok('=') => variable_token!(1, TokenType::AddEqual),
                    _ => variable_token!(0, TokenType::Add)
                }
            },

            '-' => {
                self.increment();
                match self.peek() {
                    Ok('=') => variable_token!(1, TokenType::SubtractEqual),
                    _ => variable_token!(0, TokenType::Subtract)
                }
            },

            '/' => {
                self.increment();
                match self.peek() {
                    Ok('=') => token!(TokenType::DivideEqual),
                    // Get rid of comments
                    Ok('/') => {
                        self.take_while(|character| character != '\n');
                        return self.next();
                    }
                    _ => variable_token!(0, TokenType::Divide)
                }
            },

            '*' => {
                self.increment();
                match self.peek() {
                    Ok('=') => token!(TokenType::MultiplyEqual),
                    Ok('*') => {
                        self.increment();
                        match self.peek() {
                            Ok('=') => token!(TokenType::ExponentEqual),
                            _ => variable_token!(0, TokenType::Exponent)
                        }
                    }
                    _ => variable_token!(0, TokenType::Multiply)
                }
            },

            '%' => {
                self.increment();
                match self.peek() {
                    Ok('=') => token!(TokenType::ModuloEqual),
                    _ => variable_token!(0, TokenType::Modulo)
                }
            }

            '<' => {
                self.increment();
                match self.peek() {
                    Ok('<') => {
                        self.increment();
                        match self.peek() {
                            Ok('=') => token!(TokenType::BitLeftShiftEqual),
                            _ => variable_token!(0, TokenType::BitLeftShift)
                        }
                    }
                    _ => Err(Error::TInvalidCharacter { location: self.position - 1 })
                }
            },

            '>' => {
                self.increment();
                match self.peek() {
                    Ok('>') => {
                        self.increment();
                        match self.peek() {
                            Ok('=') => token!(TokenType::BitRightShiftEqual),
                            _ => variable_token!(0, TokenType::BitRightShift)
                        }
                    }
                    _ => Err(Error::TInvalidCharacter { location: self.position - 1 })
                }
            },

            '&' => {
                self.increment();
                match self.peek() {
                    Ok('=') => variable_token!(1, TokenType::BitAndEqual),
                    _ => variable_token!(0, TokenType::BitAnd)
                }
            },

            '|' => {
                self.increment();
                match self.peek() {
                    Ok('=') => variable_token!(1, TokenType::BitOrEqual),
                    _ => variable_token!(0, TokenType::BitOr)
                }
            },

            '^' => {
                self.increment();
                match self.peek() {
                    Ok('=') => variable_token!(1, TokenType::BitXorEqual),
                    _ => variable_token!(0, TokenType::BitXor)
                }
            },

            '"' => {
                self.increment();
                let mut skip_next = false;
                self.take_while(|c| {

                    if skip_next && c == '"'{
                        skip_next = false; 
                        return true;
                    } else if skip_next {
                        skip_next = false; 
                    }

                    if c == '\\' {
                        skip_next = true;
                    }
                    c != '"'
                });
                self.increment();
                let span = Span::new(start, self.position);
                // Check if terminated
                if !matches!(self.current, Some('"')) {
                    return Err(Error::TIncompleteString { span });
                }
                Ok(Token::new(TokenType::String, span))
            }

            // Invalid characters
            _ => {
                let location = self.position;
                self.increment();
                Err(Error::TInvalidCharacter { location })
            },
        }
    }

    // utils

    fn take_while<T>(&mut self, mut predicate: T) 
        where T: FnMut(char) -> bool {
            let mut cloned_iter = self.chars.clone();
            loop {
                match cloned_iter.next() {
                    Some(character) if predicate(character) => {
                        self.chars.next();
                        self.current = Some(character);
                        self.position += character.len_utf8();
                    },
                    _ => break,
                };
            }
    }

    fn increment(&mut self) {
        match self.chars.next() {
            Some(character) => {
                self.position += character.len_utf8();
                self.current = Some(character);
            },

            None => self.current = None,
        };
    }

    fn peek(&self) -> Result<char, ()> {
        let mut cloned_iter = self.chars.clone();
        cloned_iter.next().ok_or(())
    }

    fn check_ident_start(character: char) -> bool {
        character.is_alphabetic() || matches!(character,  '_' | '~' | '#' | '$' | '@' | '`')
    }

    fn check_ident_continue(character: char) -> bool {
        character.is_alphanumeric() || matches!(character, '_' | '~' | '#' | '$' | '@' | '`')
    }

    /// just a wrapper for now
    fn check_whitespace(character: char) -> bool {
        char::is_whitespace(character)
    }

    fn check_number(character: char) -> bool {
        matches!(character, '0'..='9')
    }

    fn check_hex(character: char) -> bool {
        matches!(character, '0'..='9' | 'A'..='F' | 'a'..='f')
    }
}