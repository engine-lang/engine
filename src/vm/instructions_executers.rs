use std::io::Write;

use crate::vm::{
    syntax_tree::{
        Instruction,
        InstructionType,
        PrintInstruction,
        InputInstruction,
        IfInstruction,
        GoToInstruction
    },
    tokens::TokenType,
    environments::{
        Environment,
        Variable,
        Value,
        ValueType
    }
};
use crate::vm::assign_instructions::execute_assign_instruction;
use crate::vm::convert_instructions::execute_convert_instruction;
use crate::vm::operation_instructions::execute_operation_instruction;


pub fn execute_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: Instruction
) -> Result<(bool, u128), String>{

    let mut environment = environment;

    if instruction.instruction_type == Some(InstructionType::Assign){
        execute_assign_instruction(
            current_line, &mut environment,
            instruction.assign_instruction.unwrap())?;
    }
    else if instruction.instruction_type == Some(InstructionType::Convert){
        execute_convert_instruction(
            current_line, &mut environment,
            instruction.convert_instruction.unwrap())?;
    }
    else if instruction.instruction_type == Some(InstructionType::Operation){
        execute_operation_instruction(
            current_line, &mut environment,
            instruction.operation_instruction.unwrap())?;
    }
    else if instruction.instruction_type == Some(InstructionType::Print){
        execute_print_instruction(
            current_line, &mut environment,
            instruction.print_instruction.unwrap())?;
    }
    else if instruction.instruction_type == Some(InstructionType::Input){
        execute_input_instruction(
            current_line, &mut environment,
            instruction.input_instruction.unwrap())?;
    }
    else if instruction.instruction_type == Some(InstructionType::If){
        return execute_if_instruction(
            current_line, &mut environment,
            instruction.if_instruction.unwrap());
    }
    else if instruction.instruction_type == Some(InstructionType::Else){}
    else if instruction.instruction_type == Some(InstructionType::GoTo){
        return execute_goto_instruction(
            instruction.goto_instruction.unwrap());
    }
    return Ok((true, 0));
}


fn execute_print_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: PrintInstruction
) -> Result<(), String>{

    let variable = environment.variables.get(
        instruction.variable_name.as_ref().unwrap());
    if variable == None{
        return Err(format!(
            "Engine VM: Print Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.variable_name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let value = variable.as_ref().unwrap().as_ref().unwrap().value.as_ref().unwrap();

    if value.value_type == Some(ValueType::Boolean){
        if value.boolean == Some(true){
            print!("True");
        }
        else{
            print!("False");
        }
    }
    else if value.value_type == Some(ValueType::Integer){
        print!("{}", value.int.unwrap());
    }
    else if value.value_type == Some(ValueType::Double){
        print!("{}", value.double.unwrap());
    }
    else if value.value_type == Some(ValueType::Character){
        print!("{}", value.character.unwrap());
    }
    else if value.value_type == Some(ValueType::String){
        print!("{}", value.string.as_ref().unwrap());
    }

    /* Flush Stdin */
    if std::io::stdout().flush().is_err(){
        return Err(format!(
            "Engine VM: Print Instruction -> {}, instruction line: {}, line: {}.",
            "Failed to print to console",
            instruction.line.unwrap(), current_line));
    }

    return Ok(());
}


fn execute_input_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: InputInstruction
) -> Result<(), String>{

    let variable = environment.variables.get(
        instruction.variable_name.as_ref().unwrap());
    if variable == None{
        return Err(format!(
            "Engine VM: Input Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.variable_name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).is_err(){
        return Err(format!(
            "Engine VM: Input Instruction -> {}, instruction line: {}, line: {}.",
            "Failed to read line",
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        variable_type: Some(TokenType::String),
        name: Some(instruction.variable_name.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::String),
            boolean: None,
            character: None,
            double: None,
            int: None,
            string: Some(String::from(line.trim_end())),
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.variable_name.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_if_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: IfInstruction
) -> Result<(bool, u128), String>{

    /* Retrieve Variable */
    let variable = environment.variables.get(
        instruction.variable_name.as_ref().unwrap());
    if variable == None{
        return Err(format!(
            "Engine VM: Input Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.variable_name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let value = variable.as_ref().unwrap().as_ref().unwrap().value.as_ref().unwrap();
    if value.value_type != Some(ValueType::Boolean){
        return Err(format!(
            "Engine VM: Input Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` value must be of type `bool`",
                instruction.variable_name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    return Ok((
        value.boolean.as_ref().unwrap().clone(),
        instruction.condition_fail_goto_line.as_ref().unwrap().clone()));
}


fn execute_goto_instruction(
    instruction: GoToInstruction
) -> Result<(bool, u128), String>{

    return Ok((
        false, instruction.goto_line.as_ref().unwrap().clone()));
}
