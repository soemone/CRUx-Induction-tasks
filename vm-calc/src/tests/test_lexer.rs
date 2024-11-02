#[cfg(test)]

// These are just tests. Nothing to see here, that is if physical laws are still the same. I hope they are, at least.
// If not, this place is gonna need renovation.
mod tests {
    use crate::{tokens::{NumberType, Token, TokenType}, errors::Error, lexer, utils::Span};
    
    #[test]
    fn test_numbers() -> Result<(), ()> {
        let mut lexer = lexer::Lexer::new("1 2.4 08 5. 0o 0b 0x 0b110 0o01234567 0x0123456789ABCDEFabcdef .1")?;
        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(0, 1)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(2, 5)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(6, 8)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(9, 11)))
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TNumberExpected { location: 14 }),
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TNumberExpected { location: 17 }),
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TNumberExpected { location: 20 }),
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Binary }, Span::new(21, 26)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Octal }, Span::new(27, 37)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Hex }, Span::new(38, 62)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Number { number_type: NumberType::Real }, Span::new(63, 65)))
        );

        Ok(())
    }

    #[test]
    fn test_identifiers() -> Result<(), ()>{
        let mut lexer = lexer::Lexer::new("a ab abc a~b")?;
        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Identifier, Span::new(0, 1)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Identifier, Span::new(2, 4)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Identifier, Span::new(5, 8)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::Identifier, Span::new(9, 12)))
        );
        Ok(())
    }

    #[test]
    fn test_strings() -> Result<(), ()>{
        let mut lexer = lexer::Lexer::new("\"Hello\n\" \"ABC\\\"DEF\"")?;
        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::String, Span::new(0, 8)))
        );

        assert_eq!(
            lexer.next(), 
            Ok(Token::new(TokenType::String, Span::new(9, 19)))
        );
        Ok(())
    }


    #[test]
    fn test_operators_and_delimiters() -> Result<(), ()>{
        let mut lexer = lexer::Lexer::new("* *= ** **= + += - -= = [ ( ) ] ; / /= << <<= >> >>= & &= | |= ^ ^=")?;
        use TokenType::*;
        let list = 
            [
                (Multiply, 1), 
                (MultiplyEqual, 2),
                (Exponent, 2),
                (ExponentEqual, 3),
                (Add, 1),
                (AddEqual, 2),
                (Subtract, 1),
                (SubtractEqual, 2),
                (Equal, 1),
                (OpenArray, 1),
                (OpeningBracket, 1),
                (ClosingBracket, 1),
                (CloseArray, 1),
                (Semicolon, 1),
                (Divide, 1),
                (DivideEqual, 2),
                (BitLeftShift, 2),
                (BitLeftShiftEqual, 3),
                (BitRightShift, 2),
                (BitRightShiftEqual, 3),
                (BitAnd, 1),
                (BitAndEqual, 2),
                (BitOr, 1),
                (BitOrEqual, 2),
                (BitXor, 1),
                (BitXorEqual, 2),
            ];
        let mut idx = 0;
        for (item, len) in list {
            assert_eq!(
                lexer.next(), 
                Ok(Token::new(item, Span::new(idx, idx + len)))
            );
            idx += len + 1;
        }
        Ok(())
    }

    #[test]
    fn test_invalid() -> Result<(), ()>{
        let mut lexer = lexer::Lexer::new("<\\!>")?;
        assert_eq!(
            lexer.next(), 
            Err(Error::TInvalidCharacter { location: 0 })
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TInvalidCharacter { location: 1 })
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TInvalidCharacter { location: 2 })
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TInvalidCharacter { location: 3 })
        );

        assert_eq!(
            lexer.next(), 
            Err(Error::TEOF)
        );
        Ok(())
    }

}