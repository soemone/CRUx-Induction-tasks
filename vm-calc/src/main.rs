mod tokens;
mod lexer;
mod parser;
mod bytecode;
mod instruction;
mod ast;
mod errors;
mod utils;
mod vm;
mod functions;
mod processchain;

// Tests
mod tests;

use std::{collections::HashMap, io::Write, time::Instant};

use instruction::Instruction;
use processchain::ProcessChain;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Run file
    #[arg(short = 'r', long="run")]
    runfile: Option<String>,

    /// Run file binary
    #[arg(short='b', long="run-binary")]
    runfilebin: Option<String>,

    /// Write file binary
    #[arg(short='w', long="write-binary", value_delimiter=' ', num_args=1..=2)]
    writefilebin: Option<Vec<String>>,

    /// Show parsed output
    #[arg(long="show-parsed", short = 'p')]
    showparse: Option<String>,

    /// Show instructions created from AST of parsed output
    #[arg(long="show-instructions", short = 'i')]
    showinstructions: Option<String>,
}

fn main() -> Result<(), ()> {
    run()?;
    Ok(())
}

fn run() -> Result<(), ()> {
    let args = Args::parse();

    match args.runfile {
        Some(value) => {
            ProcessChain::run_from_file(&value)?;
            return Ok(());
        },

        None => (),
    }

    let store = |value: &String| -> Result<(), ()> {
        let output = {
            let location = value.rfind(".");
            let mut res = value.as_str();
            res = match location {
                Some(idx) => &res[0..idx],
                None => res,
            };
            &format!("{}.bin", res)
        };
        ProcessChain::store_bytecode_from_file(&value, output)?;
        Ok(())
    };

    match args.runfilebin {
        Some(value) => {
            store(&value)?;
            ProcessChain::run_from_file(&value)?;
            return Ok(());
        },

        None => (),
    }


    match args.writefilebin {
        Some(values) => {
            let output = if values.len() == 2 {
                &values[1]
            } else {
                let location = values[0].rfind(".");
                let mut res = values[0].as_str();
                res = match location {
                    Some(idx) => &res[0..idx],
                    None => res,
                };
                &format!("{}.bin", res)
            };
            ProcessChain::store_bytecode_from_file(&values[0], output)?;
    
            // store(&value)?;
            return Ok(());
        },

        None => (),
    }

    match args.showparse {
        Some(path) => {
            ProcessChain::show_parsed_from_file(&path)?;
            return Ok(());
        }

        None => (),
    };


    match args.showinstructions {
        Some(path) => {
            ProcessChain::show_bytecode_from_file(&path)?;
            return Ok(());
        }

        None => (),
    };


    // let args: Vec<String> = std::env::args().collect();
    // match args[1].as_str() {
    //     "-rb" | "--run-binary" => ProcessChain::run_from_bytecode(&args[2])?,
    //     "-wb" | "--write-binary" => store()?,
    //     "-rf" | "--run-file" => ProcessChain::run_from_file(&args[2])?,
    //     "-rfs" | "--run-store" | "--run-and-store-binary" => {
    //         store()?;
    //         ProcessChain::run_from_file(&args[2])?;
    //     },
    //     "-t" | "--text" => ProcessChain::run_from_text(&args[2])?,
    //     "repl" => repl(),
    //     arg => println!("Invalid argument `{}` provided.", arg)
    // }
    Ok(())
}

/* fn repl() {
    // Introduction
    println!("Running repl...");
    println!("Type `.quit` | `.q` to exit the repl");
    println!("Type `.show variables` | `.show var` to show the variables in the session");
    println!("Type `.time` | `.timer` to time the execution of the code");
    println!("Type `.load <filepath>` to load and execute code (timer does not apply to this)");
    println!("Type `.load bytecode <filepath>` | `.load b <filepath>` to load and execute bytecode (timer does not apply to this)");

    let mut symbols = HashMap::new();
    let mut fn_symbols = HashMap::new();
    let mut pfn_symbols = HashMap::new();
    let mut p_symbols = HashMap::new();

    let mut fn_bytecode = vec![];
    let mut functions = HashMap::new();
    
    let mut time = false;
    loop {
        print!(">> ");
        std::io::stdout().flush().expect("Failed to flush the buffer");
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed to read from the command line");
        if [".quit", ".q", ".exit", ".quit()", ".q()", ".stop", ".stop()"].contains(&buffer.trim()) {
            break;
        }
        let comment: Option<usize> = buffer.find("//");
        match comment {
            Some(location) => buffer = buffer[0..location].to_string(),
            None => (),
        };
        buffer = buffer.trim().to_string();
        if buffer.is_empty() {
            println!("No expression was provided. Did you mean to exit? Type in `.quit` to exit");
            continue;
        }

        // Load and execute binary
        if buffer.starts_with(".load b ") || buffer.starts_with(".load bytecode ") || buffer.starts_with(".load binary ") {
            let mut split = buffer.split(" ");
            split.next();
            split.next();
            match split.next() {
                Some(filename) => {
                    println!("loading binary file and executing: ");
                    ProcessChain::run_from_bytecode(filename).ok();
                },
                None => println!("Expected file path to load file!"),
            };
            continue;
        } else if buffer.starts_with(".load ") {
            let mut split = buffer.split(" ");
            split.next();
            match split.next() {
                Some(filename) => {
                    println!("loading file and executing: ");
                    ProcessChain::run_from_file(filename).ok();
                },
                None => println!("Expected file path to load file!"),
            };
            continue;
        } else if [".show functions", ".show fns", ".disp fns", ".display functions"].contains(&buffer.as_str()) {
            println!("Functions in this session: ");
            println!("BUILTIN FUNCTIONS: ");
            for (function, (args, _)) in functions::FUNCTIONS {
                let repeated = "*, ".repeat(args);
                println!("{function}({})", if args > 0 { &repeated[..(args * 3 - 2)] } else { "" });
            }
            println!("USER FUNCTIONS: ");
            for (key, (args, shadow)) in &pfn_symbols {
                let repeated = "*, ".repeat(*args);
                println!("{key}({}){}", 
                                    if *args > 0 { &repeated[..(*args * 3 - 2)] } else { "" }, 
                                    if *shadow { " [SHADOWED - UNREACHABLE]" } else { "" }
                        );
            }
            if pfn_symbols.is_empty() {
                println!("None");
            }
            continue;
        } else if [".show variables", ".show var", ".disp var", ".display variables"].contains(&buffer.as_str()) {
            println!("Variables in this session: ");
            for (key, value) in &symbols {
                if let Some(true) = p_symbols.get(key) {
                    println!("{key} = {value} [SHADOWED - UNREACHABLE]");
                } else {
                    println!("{key} = {value}");
                }
            }
            if symbols.is_empty() {
                println!("None");
            }
            continue;
        } else if [".time", ".timer"].contains(&buffer.as_str()) {
            time = !time;
            println!("The timer is now {}", if time { "on" } else { "off" });
            continue;
        }
        
        let source = Box::leak(Box::new(buffer));
        
        if time { println!("Begin compilation"); }
        let instant = Instant::now();
        
        // Crazy workaround things...

        let lexer = lexer::Lexer::new(source).expect("Failed to initialize the lexer!");
        let parser = parser::Parser::new_fn_symbols(lexer, pfn_symbols, p_symbols);
        let mut bytecode_gen = bytecode::Bytecode::new(parser);
        let (instructions, new_fn_bytecode) = bytecode_gen.generate_fn_bytecode(fn_bytecode.clone());

        (pfn_symbols, p_symbols) = bytecode_gen.get_symbols();

        let mut i = 0;
        while i < new_fn_bytecode.len() {
            let instr = new_fn_bytecode[i].clone();
            match instr {
                Instruction::FunctionDecl { name } => {
                    if !functions.contains_key(name) {
                        fn_bytecode.push(instr);
                        functions.insert(name, fn_bytecode.len() - 1);
                    } else {
                        // Delete old function to replace with new
                        let start = *functions.get(name).unwrap();
                        if let Some(Instruction::FunctionDecl { .. }) = &fn_bytecode.get(start) {
                            let end = match fn_bytecode.get(start + 2) {
                                Some(Instruction::UData { number }) => number,
                                _ => panic!("Unknown error trying to run the repl!"),
                            };
                            fn_bytecode.drain(start..=(start + end + 2));
                        }
                        fn_bytecode.push(instr);
                    }
                }

                Instruction::Output => (),

                _ => { fn_bytecode.push(instr); }
            }
            i += 1;
        }

        if time { println!("Finished compilation in {:?}", instant.elapsed()); }
        
        let mut vm = vm::VM::new_with_symbols(instructions, symbols, fn_symbols);
        
        if time { println!("Begin run"); }
        let instant = Instant::now();
        
        vm.execute_all();

        vm.print_output();

        (symbols, fn_symbols) = vm.get_symbols();
        
        if time { println!("Finished run in {:?}", instant.elapsed()); }
    }
    println!("Finished repl");
}
*/