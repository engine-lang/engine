mod utils;
mod function_iteration;


use std::collections::{
    HashMap
};
pub use crate::compiler::code_generation::utils::CodeGenerator;
use crate::compiler::code_generation::utils::{
    generate_statements_node
};
use crate::environments::{
    Environment,
    EnvironmentScope,
};


pub fn generate(
    code_generator: &mut CodeGenerator
) -> Result<(), String>{
    let mut code_generator = code_generator;

    code_generator.file.writeln(String::from("#![allow(arithmetic_overflow)]"));
    code_generator.file.writeln(String::from("use std::io;"));
    code_generator.file.writeln(String::from("use std::panic;"));

    code_generator.environments_stack.push_back(Environment {
        scope: EnvironmentScope::Main,
        variables: HashMap::new(),
        internal_variables: HashMap::new(),
        stop_statements_execution: None,
        functions: HashMap::new(),
    });

    code_generator.file.writeln(String::from("fn main(){"));
    code_generator.file.writeln(String::from("use std::io::Write;"));
    code_generator.file.writeln(String::from("panic::set_hook(Box::new(|panic_info| {"));
    code_generator.file.writeln(String::from("if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {"));
    code_generator.file.writeln(String::from("println!(\"{}\", panic_message);"));
    code_generator.file.writeln(String::from("} else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {"));
    code_generator.file.writeln(String::from("println!(\"{}\", panic_message);"));
    code_generator.file.writeln(String::from("} else {"));
    code_generator.file.writeln(String::from("println!(\"Engine Compiler -> Interperter Error {}\", panic_info);"));
    code_generator.file.writeln(String::from("}"));
    code_generator.file.writeln(String::from("}));"));

    let mut tree = code_generator.syntax_tree.clone();

    generate_statements_node(&mut code_generator, &mut tree)?;

    code_generator.file.writeln(String::from("}"));
    code_generator.environments_stack.pop_back();

    return Ok(());
}
