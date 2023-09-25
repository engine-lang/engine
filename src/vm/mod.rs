mod file;
mod syntax_tree;
mod instructions_construct;
mod assign_instructions;
mod convert_instructions;
mod operation_instructions;
mod instructions_executers;

use std::collections::HashMap;
use std::panic;
use std::env;

use crate::environments::{
    Environment,
    EnvironmentScope
};

use syntax_tree::InstructionType;
use file::File;
use instructions_construct::construct_instruction;
use instructions_executers::execute_instruction;


pub fn execute_byte_code() -> Result<(), String>{
    let args: Vec<String> = env::args().collect();
    panic::set_hook(Box::new(|panic_info| {
        if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {
            println!("{}", panic_message);
        } else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {
            println!("{}", panic_message);
        } else {
            println!("EngineVM -> VM Error");
        }
    }));

    let mut environment = Environment {
        scope: EnvironmentScope::Main,
        variables: HashMap::new()
    };

    let mut file = File::open(&args[1]);

    let info = file.read_line();
    if info.is_err(){
        panic!(
            "Engine VM: File Error -> {}, line: 1:1",
            info.unwrap_err());
    }

    let mut current_line: u128 = 1;

    let mut goto_lines_stack: Vec<u128> = Vec::new();

    loop{
        let line = file.read_line();
        if line.is_err(){
            panic!(
                "Engine VM: File Error -> {}, line: {}:1",
                line.unwrap_err(), current_line);
        }
        current_line += 1;

        if line.as_ref().unwrap().1 == 0{
            break;
        }

        let instruction = construct_instruction(line.unwrap().0, current_line);
        if instruction.is_err(){
            panic!("{}", instruction.unwrap_err());
        }
        let instruction = instruction.unwrap();

        let instruction_line = instruction.1;
        let instruction = instruction.0;

        if instruction.instruction_type == Some(InstructionType::End) ||
            instruction.instruction_type == None
        {
            break;
        }

        if goto_lines_stack.len() > 0{
            if &&instruction_line != goto_lines_stack.last().as_ref().unwrap(){
                continue;
            }
            goto_lines_stack.pop();
        }

        let result = execute_instruction(
            current_line, &mut environment, instruction);
        if result.is_err(){
            panic!("{}", result.unwrap_err());
        }
        let result = result.unwrap();

        if !result.0{
            goto_lines_stack.push(result.1);
        }
    }

    return Ok(());
}
