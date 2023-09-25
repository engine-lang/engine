use crate::vm::{
    syntax_tree::AssignInstruction,
    syntax_tree::{
        Instruction,
        InstructionType,
        PrintInstruction,
        ConvertInstruction,
        OperationInstruction,
        InputInstruction,
        IfInstruction,
        ElseInstruction,
        GoToInstruction
    },
};
use crate::vm::tokens::TokenType;


fn split_line(
    line: String, current_line: u128, instruction_line: Option<u128>
) -> Result<(String, String), String>{

    let line = line.split_once(":");
    if line == None{
        return Err(format!(
            "Engine VM: Instruction Construction -> {}, instruction line: {}, line: {}.",
            "Invalid instruction",
            instruction_line.unwrap_or(current_line),
            current_line));
    }
    let line = line.unwrap();

    return Ok((String::from(line.0), String::from(line.1)));
}


fn construct_assign_instruction(
    line: String, current_line: u128, instruction_line: u128
) -> Result<AssignInstruction, String>{

    /* Retrieve Assign Type */
    let (assign_type, line) = split_line(line, current_line, Some(instruction_line))?;

    let assign_token_type = if assign_type == "bool" {
        TokenType::Bool} else if assign_type == "int" {
        TokenType::Int} else if assign_type == "double" {
        TokenType::Double} else if assign_type == "char" {
        TokenType::Char} else if assign_type == "string" {
        TokenType::String} else {TokenType::BadToken};

    /* Retrieve Variable Name */
    let (variable_name, variable_value) = split_line(line, current_line, Some(instruction_line))?;

    /* Construct Assign Instruction */
    return Ok(AssignInstruction{
        line: Some(instruction_line),
        assign_type: Some(assign_token_type),
        variable_name: Some(variable_name),
        value: Some(variable_value)
    });
}


fn construct_convert_instruction(
    line: String, current_line: u128, instruction_line: u128
) -> Result<ConvertInstruction, String>{

    /* Retrieve Convert Type */
    let (convert_type, line) = split_line(line, current_line, Some(instruction_line))?;

    let convert_token_type = if convert_type == "bool" {
        TokenType::Bool} else if convert_type == "int" {
        TokenType::Int} else if convert_type == "double" {
        TokenType::Double} else if convert_type == "char" {
        TokenType::Char} else if convert_type == "string" {
        TokenType::String} else {TokenType::BadToken};

    /* Retrieve Convert To Variable Name */
    let (convert_to_variable_name, convert_from_variable_name) = split_line(
        line, current_line, Some(instruction_line))?;

    return Ok(ConvertInstruction{
        line: Some(instruction_line),
        convertion_type: Some(convert_token_type),
        convert_to: Some(convert_to_variable_name),
        convert_from: Some(convert_from_variable_name),
    });
}


fn construct_input_instruction(
    line: String, instruction_line: u128
) -> Result<InputInstruction, String>{

    return Ok(InputInstruction{
        line: Some(instruction_line),
        variable_name: Some(line)
    });
}


fn construct_print_instruction(
    line: String, instruction_line: u128
) -> Result<PrintInstruction, String>{

    return Ok(PrintInstruction{
        line: Some(instruction_line),
        variable_name: Some(line)
    });
}


fn construct_operation_instruction(
    line: String, current_line: u128, instruction_line: u128
) -> Result<OperationInstruction, String>{

    /* Retrieve Operation Type */
    let (convert_type, line) = split_line(
        line, current_line, Some(instruction_line))?;

    let operation_token_type = if convert_type == "Plus" {
        TokenType::Plus} else if convert_type == "Minus" {
        TokenType::Minus} else if convert_type == "Mul" {
        TokenType::Mul} else if convert_type == "Div" {
        TokenType::Div} else if convert_type == "Mod" {
        TokenType::Mod} else if convert_type == "Or" {
        TokenType::Or} else if convert_type == "And" {
        TokenType::And} else if convert_type == "GreaterThan" {
        TokenType::GreaterThan} else if convert_type == "GreaterThanOrEqual" {
        TokenType::GreaterThanOrEqual} else if convert_type == "LessThan" {
        TokenType::LessThan} else if convert_type == "LessThanOrEqual" {
        TokenType::LessThanOrEqual} else {TokenType::BadToken};

    /* Retrieve Assign To Variable Name */
    let (assign_to_variable, line) = split_line(
        line, current_line, Some(instruction_line))?;

    /* Retrieve Left And Right Variables Names */
    let (left_variable, right_variable) = split_line(
        line, current_line, Some(instruction_line))?;

    return Ok(OperationInstruction{
        operation_type: Some(operation_token_type),
        assign_to_variable: Some(assign_to_variable),
        left_variable: Some(left_variable),
        right_variable: Some(right_variable),
        line: Some(instruction_line)
    });
}


fn construct_if_instruction(
    line: String, current_line: u128, instruction_line: u128
) -> Result<IfInstruction, String>{

    /* Retrieve Variable And Go To */
    let (variable_name, goto_line) = split_line(
        line, current_line, Some(instruction_line))?;

    /* Parser GoTo  Line */
    let goto_line = goto_line.trim();
    let goto_line = goto_line.parse::<u128>();
    if goto_line.is_err(){
        return Err(format!(
            "Engine VM: Instruction Construction -> {}, instruction line: {}, line: {}.",
            "GoTo line part is not an integer", current_line - 1, current_line));
    }
    let goto_line = goto_line.unwrap();

    return Ok(IfInstruction {
        variable_name: Some(variable_name),
        condition_fail_goto_line: Some(goto_line),
        line: Some(instruction_line)
    });
}


fn construct_else_instruction(
    instruction_line: u128
) -> Result<ElseInstruction, String>{

    return Ok(ElseInstruction{
        line: Some(instruction_line),
    });
}


fn construct_goto_instruction(
    line: String, current_line: u128, instruction_line: u128
) -> Result<GoToInstruction, String>{

    /* Parser GoTo  Line */
    let goto_line = line.trim();
    let goto_line = goto_line.parse::<u128>();
    if goto_line.is_err(){
        return Err(format!(
            "Engine VM: Instruction Construction -> {}, instruction line: {}, line: {}.",
            "GoTo line part is not an integer", current_line - 1, current_line));
    }
    let goto_line = goto_line.unwrap();

    return Ok(GoToInstruction {
        goto_line: Some(goto_line),
        line: Some(instruction_line)
    });
}


pub fn construct_instruction(
    line: String, current_line: u128
) -> Result<(Instruction, u128), String>{

    /* Retrieve Line Count */
    let (line_counter, line) = split_line(line, current_line, None)?;

    /* Parser Line Counter */
    let line_counter = line_counter.parse::<u128>();
    if line_counter.is_err(){
        return Err(format!(
            "Engine VM: Instruction Construction -> {}, instruction line: {}, line: {}.",
            "Line part is not an integer", current_line - 1, current_line));
    }
    let line_counter = line_counter.unwrap();

    /* Retrieve Instruction */
    let (instruction_str, line) = split_line(line, current_line, Some(line_counter))?;

    if instruction_str == "Assign"{
        let assign_instruction_node = construct_assign_instruction(
            line, current_line, line_counter)?;

        return Ok((Instruction{
            instruction_type: Some(InstructionType::Assign),
            assign_instruction: Some(assign_instruction_node),
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: None,
            input_instruction: None,
            if_instruction: None,
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "Convert"{
        let convert_instruction_node = construct_convert_instruction(
            line, current_line, line_counter)?;

        return Ok((Instruction {
            instruction_type: Some(InstructionType::Convert),
            assign_instruction: None,
            convert_instruction: Some(convert_instruction_node),
            operation_instruction: None,
            print_instruction: None,
            input_instruction: None,
            if_instruction: None,
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "Input"{
        let input_instruction_node = construct_input_instruction(
            line, line_counter)?;

        return Ok((Instruction {
            instruction_type: Some(InstructionType::Input),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: None,
            input_instruction: Some(input_instruction_node),
            if_instruction: None,
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "Print"{
        let print_instruction_node = construct_print_instruction(
            line, line_counter)?;

        return Ok((Instruction {
            instruction_type: Some(InstructionType::Print),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: Some(print_instruction_node),
            input_instruction: None,
            if_instruction: None,
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "Operation"{
        let operation_instruction_node = construct_operation_instruction(
            line, current_line, line_counter)?;

        return Ok((Instruction{
            instruction_type: Some(InstructionType::Operation),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: Some(operation_instruction_node),
            print_instruction: None,
            input_instruction: None,
            if_instruction: None,
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "If"{
        let if_instruction_node = construct_if_instruction(
            line, current_line, line_counter)?;

        return Ok((Instruction {
            instruction_type: Some(InstructionType::If),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: None,
            input_instruction: None,
            if_instruction: Some(if_instruction_node),
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "Else"{
        let else_instruction_node = construct_else_instruction(line_counter)?;

        return Ok((Instruction {
            instruction_type: Some(InstructionType::Else),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: None,
            input_instruction: None,
            if_instruction: None,
            else_instruction: Some(else_instruction_node),
            goto_instruction: None
        }, line_counter));
    }
    else if instruction_str == "GoTo"{
        let goto_instruction_node = construct_goto_instruction(
            line, current_line, line_counter)?;

        return Ok((Instruction {
            instruction_type: Some(InstructionType::GoTo),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: None,
            input_instruction: None,
            if_instruction: None,
            else_instruction: None,
            goto_instruction: Some(goto_instruction_node)
        }, line_counter));
    }
    else if instruction_str == "End"{
        return Ok((Instruction {
            instruction_type: Some(InstructionType::End),
            assign_instruction: None,
            convert_instruction: None,
            operation_instruction: None,
            print_instruction: None,
            input_instruction: None,
            if_instruction: None,
            else_instruction: None,
            goto_instruction: None
        }, line_counter));
    }

    return Err(format!(
        "Engine VM: Instruction Construction -> {}, instruction line: {}, line: {}.",
        format!("Unknown instruction `{}`", instruction_str),
        line_counter, current_line));
}
