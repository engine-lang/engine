mod symantic_analyzer;
mod executes;

use std::panic;
use std::env;

use crate::constants::Mode;
use crate::file::File;
use crate::lexer::Lexer;
use crate::parser::{
    Parser,
    statement
};
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

    let file = File::new(&args[1], Mode::Interpreter);
    let lexer = Lexer::new(file, Mode::Interpreter);
    if lexer.is_err(){
        panic!(
            "Engine Interpreter -> File Error: Failed in reading file character `{}`.",
            &args[1]);
    }

    let mut parser = Parser::new(lexer.unwrap(), Mode::Interpreter)?;

    let mut analyzer = Analyzer::new();

    loop {
        let result = statement(&mut parser, true)?;
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
