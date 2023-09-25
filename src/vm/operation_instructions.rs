use crate::tokens::TokenType;
use crate::environments::{
    Environment,
    Variable,
    Value,
    ValueType
};

use crate::vm::syntax_tree::OperationInstruction;


pub fn execute_operation_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    if instruction.operation_type == Some(TokenType::Plus){
        execute_plus_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::Minus){
        execute_minus_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::Mul){
        execute_mul_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::Div){
        execute_div_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::Mod){
        execute_mod_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::And){
        execute_and_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::Or){
        execute_or_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::GreaterThan){
        execute_greater_than_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::GreaterThanOrEqual){
        execute_greater_than_or_equal_operation(
            current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::LessThan){
        execute_less_than_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::LessThanOrEqual){
        execute_less_than_or_equal_operation(
            current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::Equal){
        execute_equal_operation(current_line, environment, instruction)?;
    }
    else if instruction.operation_type == Some(TokenType::NotEqual){
        execute_not_equal_operation(current_line, environment, instruction)?;
    }

    return Ok(());
}


fn execute_plus_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::String) &&
        assign_to_variable.variable_type != Some(TokenType::Int) &&
        assign_to_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Plus` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    /* Concat String */
    if assign_to_variable.variable_type == Some(TokenType::String){
        if left_variable.variable_type != Some(TokenType::String) ||
            right_variable.variable_type != Some(TokenType::String)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Plus` to type `String` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::String),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::String),
                boolean: None,
                character: None,
                double: None,
                int: None,
                string: Some(left_value + &right_value),
                string_value: None,
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Int){
        if left_variable.variable_type != Some(TokenType::Int) ||
            right_variable.variable_type != Some(TokenType::Int)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Plus` to type `Int` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Int),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Integer),
                boolean: None,
                character: None,
                double: None,
                int: Some(left_value + right_value),
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Double){
        if left_variable.variable_type != Some(TokenType::Double) ||
            right_variable.variable_type != Some(TokenType::Double)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Plus` to type `Double` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Double),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Double),
                boolean: None,
                character: None,
                double: Some(left_value + right_value),
                int: None,
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }

    return Ok(());
}


fn execute_minus_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Int) &&
        assign_to_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Minus` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Int){
        if left_variable.variable_type != Some(TokenType::Int) ||
            right_variable.variable_type != Some(TokenType::Int)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Minus` to type `Int` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Int),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Integer),
                boolean: None,
                character: None,
                double: None,
                int: Some(left_value - right_value),
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Double){
        if left_variable.variable_type != Some(TokenType::Double) ||
            right_variable.variable_type != Some(TokenType::Double)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Minus` to type `Double` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Double),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Double),
                boolean: None,
                character: None,
                double: Some(left_value - right_value),
                int: None,
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }

    return Ok(());
}


fn execute_mul_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Int) &&
        assign_to_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Mul` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Int){
        if left_variable.variable_type != Some(TokenType::Int) ||
            right_variable.variable_type != Some(TokenType::Int)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Mul` to type `Int` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Int),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Integer),
                boolean: None,
                character: None,
                double: None,
                int: Some(left_value * right_value),
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Double){
        if left_variable.variable_type != Some(TokenType::Double) ||
            right_variable.variable_type != Some(TokenType::Double)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Mul` to type `Double` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Double),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Double),
                boolean: None,
                character: None,
                double: Some(left_value * right_value),
                int: None,
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }

    return Ok(());
}


fn execute_div_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Double){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Div` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    if left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Div` to type `Double` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();
    let right_value = right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Double),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Double),
            boolean: None,
            character: None,
            double: Some(left_value / right_value),
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_mod_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Int) &&
        assign_to_variable.variable_type == Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Mod` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Int){
        if left_variable.variable_type != Some(TokenType::Int) ||
            right_variable.variable_type != Some(TokenType::Int)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Mod` to type `Int` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Int),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Integer),
                boolean: None,
                character: None,
                double: None,
                int: Some(left_value % right_value),
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }
    else if assign_to_variable.variable_type == Some(TokenType::Double){
        if left_variable.variable_type != Some(TokenType::Double) ||
            right_variable.variable_type != Some(TokenType::Double)
        {
            return Err(format!(
                "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
                format!(
                    "Can't do operation `Mod` to type `Double` and `{:?}` and `{:?}`",
                    left_variable.variable_type.as_ref().unwrap(),
                    right_variable.variable_type.as_ref().unwrap()),
                instruction.line.unwrap(), current_line));
        }

        let left_value = left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();
        let right_value = right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone();

        let variable = Variable{
            is_reasigned: false,
            variable_type: Some(TokenType::Int),
            name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
            value: Some(Value{
                value_type: Some(ValueType::Integer),
                boolean: None,
                character: None,
                double: Some(left_value % right_value),
                int: None,
                string: None,
                string_value: None
            })
        };

        environment.variables.insert(
            instruction.assign_to_variable.as_ref().unwrap().clone(),
            Some(variable));
    }

    return Ok(());
}


fn execute_and_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `And` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Bool) ||
        right_variable.variable_type != Some(TokenType::Bool)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `And` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = left_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone();
    let right_value = right_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone();

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value && right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_or_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Or` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Bool) ||
        right_variable.variable_type != Some(TokenType::Bool)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Or` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = left_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone();
    let right_value = right_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone();

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value || right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_greater_than_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `GreaterThan` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Int) &&
        left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Int) &&
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `GreaterThan` to type Bool and types `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = if left_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let right_value = if right_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value > right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_greater_than_or_equal_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `GreaterThanOrEqual` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Int) &&
        left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Int) &&
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `GreaterThanOrEqual` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = if left_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let right_value = if right_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value >= right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_less_than_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type != Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `LessThan` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Int) &&
        left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Int) &&
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `LessThan` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = if left_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let right_value = if right_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value < right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_less_than_or_equal_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type == Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `LessThanOrEqual` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Int) &&
        left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Int) &&
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `LessThanOrEqual` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = if left_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let right_value = if right_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value <= right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_equal_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type == Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Equal` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    if left_variable.variable_type != Some(TokenType::Int) &&
        left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Int) &&
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `Equal` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = if left_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let right_value = if right_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value == right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_not_equal_operation(
    current_line: u128,
    environment: &mut Environment,
    instruction: OperationInstruction
) -> Result<(), String>{

    /* Retrieve Assign To Variable */
    let assign_to_variable = environment.variables.get(
        instruction.assign_to_variable.as_ref().unwrap());
    if assign_to_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.assign_to_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let assign_to_variable = assign_to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Left Variable */
    let left_variable = environment.variables.get(
        instruction.left_variable.as_ref().unwrap());
    if left_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.left_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let left_variable = left_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve Right Variable */
    let right_variable = environment.variables.get(
        instruction.right_variable.as_ref().unwrap());
    if right_variable == None{
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.right_variable.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let right_variable = right_variable.as_ref().unwrap().as_ref().unwrap();

    if assign_to_variable.variable_type == Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `NotEqual` to type `{:?}` and `{:?}` and `{:?}`",
                assign_to_variable.variable_type.as_ref().unwrap(),
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    if left_variable.variable_type != Some(TokenType::Int) &&
        left_variable.variable_type != Some(TokenType::Double) ||
        right_variable.variable_type != Some(TokenType::Int) &&
        right_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Operation Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't do operation `NotEqual` to type `Bool` and `{:?}` and `{:?}`",
                left_variable.variable_type.as_ref().unwrap(),
                right_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let left_value = if left_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            left_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {left_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let right_value = if right_variable.value.as_ref().unwrap().value_type == Some(ValueType::Integer){
            right_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
        } else {right_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()};

    let variable = Variable{
        is_reasigned: false,
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.assign_to_variable.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(left_value != right_value),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.assign_to_variable.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}
