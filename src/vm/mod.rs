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

use crate::syntax_tree::InstructionType;
use crate::file::File;
use instructions_construct::construct_instruction;
use instructions_executers::execute_instruction;


pub fn execute_byte_code() -> Result<(), String>{
    panic::set_hook(Box::new(|panic_info| {
        if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {
            println!("{}", panic_message);
        } else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {
            println!("{}", panic_message);
        } else {
            println!("EngineVM -> VM Error");
        }
    }));

    let args: Vec<String> = env::args().collect();

    let mut environment = Environment {
        scope: EnvironmentScope::Main,
        variables: HashMap::new()
    };

    let mut file = File::open_byte_file(&args[1]);

    let info = file.read_byte_code_line();
    if info.is_err(){
        panic!(
            "Engine VM: File Error -> {}, line: 1:1",
            info.unwrap_err());
    }

    let mut current_line: u128 = 1;

    let mut current_go_to_line: Option<u128> = None;

    let mut lines: HashMap<u128, u64> = HashMap::new();

    loop{
        /* Init Some Data */
        let stream_line = file.get_reader_stream_position();

        /* Read Instruction Line */
        let line = file.read_byte_code_line();
        if line.is_err(){
            panic!(
                "Engine VM: File Error -> {}, line: {}:1",
                line.unwrap_err(), current_line);
        }
        current_line += 1;

        if line.as_ref().unwrap().1 == 0{
            break;
        }

        /* Construct Instruction */
        let instruction = construct_instruction(line.unwrap().0, current_line);
        if instruction.is_err(){
            panic!("{}", instruction.unwrap_err());
        }
        let instruction = instruction.unwrap();

        let instruction_line = instruction.1;
        let instruction = instruction.0;

        if instruction.instruction_type == Some(InstructionType::End){
            break;
        }

        /* Insert Line into Saved Lines */
        lines.insert(instruction_line, stream_line.clone());

        /* Check If Line it is Go To Line */
        if current_go_to_line != None{
            if current_go_to_line != Some(instruction_line){
                continue;
            }
            current_go_to_line = None;
        }

        /* Execute Line */
        let result = execute_instruction(
            current_line, &mut environment, instruction);
        if result.is_err(){
            panic!("{}", result.unwrap_err());
        }
        let result = result.unwrap();

        /* Check Result if It is Go To Line */
        if !result.0{
            if result.1 < instruction_line{
                let stream_position = lines.get(&result.1);
                file.set_reader_stream_position(stream_position.as_deref().unwrap().clone());
            }
            current_go_to_line = Some(result.1);
        }
    }

    return Ok(());
}
