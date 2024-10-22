#[cfg(test)]

// These are just tests. Nothing to see here, that is if physical laws are still the same. I hope they are, at least.
// If not, this place is gonna need renovation.
mod tests {
    use std::{ops::Range, rc::Rc};
    use crate::{ast::{Tree, AST}, errors::Error, lexer::Lexer, parser::Parser, utils::Span};

    fn generate_tree(input: &str) -> Vec<Result<Rc<Tree<'_>>, Error>> {
        let lexer = Lexer::new(input).unwrap();
        let mut parser = Parser::new(lexer);
        parser.generate_expressions()
    }

    fn ok_tree(ast: AST, range: Range<usize>) -> Result<Rc<Tree>, Error> {
        Ok(Rc::new(Tree::new(ast, Span::from_range(range))))
    }

    fn generate_and_test(input: &str, tests: &[&str]) {
        let tree = generate_tree(input);
        
        for i in 0..tests.len() {
            assert_eq!(format!("{}", tree[i].clone().unwrap()), tests[i].to_owned())
        }   
    }

    fn expect_error<T, A>(value: &Result<T, A>) {
        assert!(matches!(value, Err(..)));
    }

    #[test] 
    fn number() {
        let tree = generate_tree("6;0.5;.9;");
        assert_eq!(tree[0], ok_tree(AST::Number { value: 6.0 }, 0..1));
        assert_eq!(tree[1], ok_tree(AST::Number { value: 0.5 }, 2..5));
        assert_eq!(tree[2], ok_tree(AST::Number { value: 0.9 }, 6..8));
    }

    #[test] 
    fn hex() {
        let tree = generate_tree("0x0;0xf;_ 0x;");
        assert_eq!(tree[0], ok_tree(AST::Number { value: 0.0 }, 0..3));
        assert_eq!(tree[1], ok_tree(AST::Number { value: 15.0 }, 4..7));
        // An error arises before the third expression is produced (identifier `_`), since the tokenizer 
        // generates the next token within the parser as the previous expression is parsed
        // This tokenizer error bubbles up to the parser, preventing the next expression 
        // from being returned, in favour of the error
        expect_error(&tree[2]);
    }

    #[test] 
    fn octal() {
        let tree = generate_tree("0o0;0o5;_ 0o9;");
        assert_eq!(tree[0], ok_tree(AST::Number { value: 0.0 }, 0..3));
        assert_eq!(tree[1], ok_tree(AST::Number { value: 5.0 }, 4..7));
        // Same reason as above
        expect_error(&tree[2]);
    }

    #[test]
    fn operators() {
        let tests = [
            "(-1)",
            "(+1)",
            "(1 + 1)",
            "(1 - 1)",
            "(1 * 1)",
            "(1 / 1)",
            "(1 ** 1)",
            "(1 & 1)",
            "(1 | 1)",
            "(1 >> 1)",
            "(1 << 1)",
            "(1 ^ 1)",
        ];
        generate_and_test("-1;+1;1+1;1-1;1*1;1/1;1**1;1&1;1|1;1>>1;1<<1;1^1;", &tests)
    }

    #[test]
    fn variables() {
        let tests = [
            "(let a)",
            "(let a = 5)",
            "(let a = {Null})",
            "*(let a = \"str\")*",
        ];
        generate_and_test("let a;let a = 5;let a = Null;let a = \"str\":", &tests)
    }

    #[test]
    fn variable_operators() {
        let tests = [
            "(let a)",
            "(a = 5)",
            "(a += 7)",
            "(a -= 7)",
            "(a *= 7)",
            "(a /= 7)",
            "(a **= 7)",
            "(a &= 7)",
            "(a |= 7)",
            "(a ^= 7)",
            "(a <<= 7)",
            "(a >>= 7)",
        ];
        generate_and_test("let a;a=5;a+=7;a-=7;a*=7;a/=7;a**=7;a&=7;a|=7;a^=7;a<<=7;a>>=7;", &tests)
    }

    
    #[test]
    fn functions() {
        let tests = [
            "(let a a b c = ((a + b) + c))",
            "a(1, 2, 3)",
            "<PRINT>(1, 2, 3)",
        ];
        generate_and_test("let a a b c=a+b+c;a(1,2,3);print(1,2,3);", &tests)
    }
        
    #[test]
    fn delete() {
        let tests = [
            "(let a)",
            "(delete a)",
        ];
        generate_and_test("let a;delete a;", &tests)
    }
}