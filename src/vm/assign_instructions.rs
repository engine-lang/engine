use crate::tokens::TokenType;
use crate::environments::{
    Environment,
    Variable,
    Value,
    ValueType
};

use crate::vm::syntax_tree::AssignInstruction;


pub fn execute_assign_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: AssignInstruction
) -> Result<(), String>{

    /* Assert Assign Type Is Correct */
    if instruction.assign_type == Some(TokenType::Bool){
        execute_assign_bool_instruction(current_line, environment, instruction)?;
    }
    else if instruction.assign_type == Some(TokenType::Int){
        execute_assign_int_instruction(current_line, environment, instruction)?;
    }
    else if instruction.assign_type == Some(TokenType::Double){
        execute_assign_double_instruction(current_line, environment, instruction)?;
    }
    else if instruction.assign_type == Some(TokenType::Char){
        execute_assign_char_instruction(environment, instruction)?;
    }
    else if instruction.assign_type == Some(TokenType::String){
        execute_assign_string_instruction(current_line, environment, instruction)?;
    }
    else if instruction.assign_type == Some(TokenType::BadToken){
        return Err(format!(
            "Engine VM: Assign Instruction -> {} `{:?}`, instruction line: {}, line: {}.",
            "Unknown Assign type",
            instruction.assign_type.as_ref().unwrap(),
            instruction.line.unwrap(), current_line));
    }

    return Ok(());
}


fn execute_assign_bool_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: AssignInstruction
) -> Result<(), String>{

    if instruction.value != Some(String::from("True")) && instruction.value != Some(String::from("False")){
        return Err(format!(
            "Engine VM: Assign Instruction -> {} `{:?}`, instruction line: {}, line: {}.",
            "Invalid Bool value",
            instruction.value.as_ref().unwrap(),
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.variable_name.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(if instruction.value == Some(String::from("True")) {true} else {false}),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.variable_name.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_assign_int_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: AssignInstruction
) -> Result<(), String>{

    let value = instruction.value.as_ref().unwrap().parse::<i64>();
    if value.is_err(){
        return Err(format!(
            "Engine VM: Assign Instruction -> {} `{:?}`, instruction line: {}, line: {}.",
            "Invalid Int value",
            instruction.value.as_ref().unwrap(),
            instruction.line.unwrap(), current_line));
    }
    let value = value.unwrap();

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Int),
        name: Some(instruction.variable_name.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Integer),
            boolean: None,
            character: None,
            double: None,
            int: Some(value),
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.variable_name.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_assign_double_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: AssignInstruction
) -> Result<(), String>{

    let value = instruction.value.as_ref().unwrap().parse::<f64>();
    if value.is_err(){
        return Err(format!(
            "Engine VM: Assign Instruction -> {} `{:?}`, instruction line: {}, line: {}.",
            "Invalid Double value",
            instruction.value.as_ref().unwrap(),
            instruction.line.unwrap(), current_line));
    }
    let value = value.unwrap();

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Double),
        name: Some(instruction.variable_name.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Double),
            boolean: None,
            character: None,
            double: Some(value),
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.variable_name.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_assign_char_instruction(
    environment: &mut Environment,
    instruction: AssignInstruction
) -> Result<(), String>{

    let value = instruction.value.as_ref().unwrap().clone();
    let value = if value.len() < 3 {'\0'} else {
        if value == "'\\n'"{
            '\n'
        }
        else if value == "'\\t'"{
            '\t'
        }
        else if value == "'\''"{
            '\''
        }
        else if value == "'\\\\'"{
            '\\'
        }
        else{
            value.chars().nth(1).unwrap()
        }
    };

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Char),
        name: Some(instruction.variable_name.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Character),
            boolean: None,
            character: Some(value),
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.variable_name.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_assign_string_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: AssignInstruction
) -> Result<(), String>{

    let value = instruction.value.unwrap();
    if value.len() < 2{
        return Err(format!(
            "Engine VM: Assign Instruction -> {} `{:?}`, instruction line: {}, line: {}.",
            "Invalid String",
            value,
            instruction.line.unwrap(), current_line));
    }
    let value = &value[1..value.len()-1];

    let mut new_value = String::new();
    let mut special = false;

    for (i, ch) in value.chars().enumerate(){
        if special{
            special = false;
            continue;
        }

        else if ch == '\\' && i < value.len() - 1{
            if value.chars().nth(i + 1) == Some('n') {
                new_value.push('\n');
                special = true;
                continue;
            }
            else if value.chars().nth(i + 1) == Some('t'){
                new_value.push('\t');
                special = true;
                continue;
            }
            else if value.chars().nth(i + 1) == Some('\\'){
                new_value.push('\\');
                special = true;
                continue;
            }
            else if value.chars().nth(i + 1) == Some('"'){
                new_value.push('"');
                special = true;
                continue;
            }
        }
        new_value.push(ch);
    }

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::String),
        name: Some(instruction.variable_name.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::String),
            boolean: None,
            character: None,
            double: None,
            int: None,
            string: Some(String::from(new_value)),
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.variable_name.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}
