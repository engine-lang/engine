mod parser;
mod symantic_analyzer;
mod code_generator;
mod byte_code_generator;


use std::panic;
use std::env;

use crate::constants::Mode;

use crate::file::File;
use crate::lexer::Lexer;
use parser::Parser;
use symantic_analyzer::{Analyzer, analyze};
use code_generator::CodeGenerator;
use byte_code_generator::ByteCodeGenerator;


pub fn compile(generate_byte_code: bool) -> Result<(), String>{
    panic::set_hook(Box::new(|panic_info| {
        if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {
            println!("{}", panic_message);
        } else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {
            println!("{}", panic_message);
        } else {
            println!("Engine Compiler -> Interperter Error {}", panic_info);
        }
    }));

    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        panic!("Engine Compiler -> Compiler Error: Must provide file path.");
    }

    let file_path = args[1].clone();
    let mut file_path_splitted: Vec<&str> = file_path.split("/").collect();
    let last_str = file_path_splitted.pop().unwrap();
    let parent_path = file_path_splitted.join("/");

    let mut ext_arr: Vec<&str> = last_str.split(".").collect();
    if ext_arr.pop().unwrap() != "en"{
        panic!("Engine Compiler -> File Error: File does not have engine extension part.")
    }

    let file_name_without_ext = ext_arr.join(".");

    let current_mode = if generate_byte_code {Mode::ByteCodeGenerator} else {Mode::Compiler};

    let file = File::new(
        &args[1],
        current_mode.clone()
    );
    let lexer = Lexer::new(file, current_mode);
    if lexer.is_err(){
        panic!("{}", lexer.unwrap_err());
    }

    let mut parser = Parser::new(lexer.unwrap())?;

    let syntax_tree = parser.parse()?;

    let mut analyzer = Analyzer::new();
    analyze(&mut analyzer, syntax_tree.clone())?;

    if generate_byte_code{
        let mut byte_code_generator = ByteCodeGenerator::new(
            syntax_tree.clone(),
            parent_path,
            file_name_without_ext);
        if byte_code_generator.is_err(){
            panic!("{}", byte_code_generator.unwrap_err());
        }

        byte_code_generator.as_mut().unwrap().generate()?;
    }
    else{
        let mut code_generator = CodeGenerator::new(
            syntax_tree.clone(),
            parent_path,
            file_name_without_ext);
        if code_generator.is_err(){
            panic!("{}", code_generator.unwrap_err());
        }

        code_generator.as_mut().unwrap().generate()?;
        code_generator.as_mut().unwrap().execute()?;
        code_generator.as_mut().unwrap().clean()?;
    }

    return Ok(());
}
