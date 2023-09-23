mod constants;
mod character;
mod file;
mod lexer;
mod tokens;
mod parser;
mod syntax_tree;
mod symantic_analyzer;
mod environments;
mod executes;

use std::panic;
use std::env;

use file::File;
use lexer::Lexer;
use parser::Parser;
use symantic_analyzer::Analyzer;
use crate::interpreter::executes::execute_statement;


pub fn interpret() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();

    panic::set_hook(Box::new(|panic_info| {
        if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {
            println!("{}", panic_message);
        } else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {
            println!("{}", panic_message);
        } else {
            println!("Engine Interpreter -> Interperter Error");
        }
    }));

    interpreter_mode(&args[1])?;

    return Ok(());
}


fn interpreter_mode(file_path: &String) -> Result<(), String>{
    let file = File::new(&file_path);
    let lexer = Lexer::new(file);
    if lexer.is_err(){
        panic!(
            "Engine Interpreter -> File Error: Failed in reading file character `{}`.",
            &file_path);
    }

    let mut parser = Parser::new(lexer.unwrap());
    if parser.is_err(){
        panic!(
            "Engine Interpreter -> Parser Error: Failed in reading file character `{}`.",
            &file_path);
    }

    let mut analyzer = Analyzer::new();

    loop {
        let result = parser.as_mut().unwrap().parse_statement(true)?;
        let node = result.1;
        if result.0{
            break;
        }

        if node.statement_type != None{
            execute_statement(&mut analyzer, &node)?;
        }
    }

    return Ok(());
}
