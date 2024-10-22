use crate::{bytecode::Bytecode, lexer::Lexer, parser::Parser, vm::VM};

pub struct ProcessChain { }

impl ProcessChain {
    pub fn store_bytecode_from_file(path_in: &str, path_out: &str) -> Result<(), ()> {
        let source = &match std::fs::read_to_string(path_in) {
            Ok(result) => result,
            Err(error) => {
                println!("An error has occured while reading file from path {path_in}: {error}");
                return Err(());
            }
        };
        Self::store_bytecode_from_text(source, path_out)?;
        Ok(())
    }

    pub fn store_bytecode_from_text(source: &str, path: &str) -> Result<(), ()> {
        let lexer = Lexer::new(source)?;
        let parser = Parser::new(lexer);
        let mut bytecode_gen = Bytecode::new(parser);
        let instructions = bytecode_gen.generate_bytecode();
        let bytecode = match bincode::serialize(&instructions) {
            Ok(result) => result,
            Err(error) => panic!("An error occured while trying to read from path {path}: {error}"),
        };
        match std::fs::write(path, bytecode) {
            Ok(()) => println!("Wrote bytecode to path: {path}"),
            Err(error) => panic!("An error occured while trying to read from path {path}: {error}"),
        };
        Ok(())
    }

    pub fn run_from_text(source: &str) -> Result<(), ()> {
        let lexer = Lexer::new(source)?;
        let parser = Parser::new(lexer);
        let mut bytecode_gen = Bytecode::new(parser);
        let instructions = bytecode_gen.generate_bytecode();
        let mut vm = VM::new(instructions);
        vm.execute_all();
        vm.print_output();
        Ok(())
    }

    pub fn run_from_bytecode(path: &str) -> Result<(), ()>{
        let bytecode = match std::fs::read(path) {
            Ok(result) => result,
            Err(error) => {
                println!("An error occured while trying to read from path {path}: {error}");
                return Err(());
            },
        };

        let instructions = match bincode::deserialize(&bytecode) {
            Ok(result) => result,
            Err(error) => {
                println!("An error occured while trying to read from path {path}: {error}");
                return Err(());
            },
        };
        let mut vm = VM::new(instructions);
        vm.execute_all();
        vm.print_output();
        Ok(())
    }

    pub fn run_from_file(path: &str) -> Result<(), ()> {
        let source = &match std::fs::read_to_string(path) {
            Ok(result) => result,
            Err(error) => {
                println!("An error has occured while reading file from path {path}: {error}");
                return Err(());
            }
        };
        let lexer = Lexer::new(source)?;
        let parser = Parser::new(lexer);
        let mut bytecode_gen = Bytecode::new(parser);
        let instructions = bytecode_gen.generate_bytecode();
        let mut vm = VM::new(instructions);
        vm.execute_all();
        vm.print_output();
        Ok(())
    }

    pub fn show_parsed_from_file(path: &str) -> Result<(), ()> {
        let source = &match std::fs::read_to_string(path) {
            Ok(result) => result,
            Err(error) => {
                println!("An error has occured while reading file from path {path}: {error}");
                return Err(());
            }
        };
        let lexer = Lexer::new(source)?;
        let mut parser = Parser::new(lexer);
        loop {
            if parser.eof { break; }
            match parser.next_expression() {
                Ok(tree) => println!("{tree}"),
                Err(error) => println!("{error}"),
            }
        }
        Ok(())
    }

    pub fn show_bytecode_from_file(path: &str) -> Result<(), ()> {
        let source = &match std::fs::read_to_string(path) {
            Ok(result) => result,
            Err(error) => {
                println!("An error has occured while reading file from path {path}: {error}");
                return Err(());
            }
        };
        let lexer = Lexer::new(source)?;
        let parser = Parser::new(lexer);
        let mut bytecode_gen = Bytecode::new(parser);
        let instructions = bytecode_gen.generate_bytecode();
        for instruction in instructions {
            println!("{instruction:?}");
        }
        Ok(())
    }
}