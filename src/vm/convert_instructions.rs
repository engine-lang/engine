use crate::vm::{
    syntax_tree::ConvertInstruction,
    tokens::TokenType,
    environments::{
        Environment,
        Variable,
        Value,
        ValueType
    }
};


pub fn execute_convert_instruction(
    current_line: u128,
    environment: &mut Environment,
    instruction: ConvertInstruction
) -> Result<(), String>{

    let mut environment = environment;

    if instruction.convertion_type == Some(TokenType::Bool){
        execute_convert_to_bool(current_line, &mut environment, instruction)?;
    }
    else if instruction.convertion_type == Some(TokenType::Int){
        execute_convert_to_int(current_line, environment, instruction)?;
    }
    else if instruction.convertion_type == Some(TokenType::Double){
        execute_convert_to_double(current_line, environment, instruction)?;
    }
    else if instruction.convertion_type == Some(TokenType::Char){
        execute_convert_to_char(current_line, environment, instruction)?;
    }
    else if instruction.convertion_type == Some(TokenType::String){
        execute_convert_to_string(current_line, environment, instruction)?;
    }
    else if instruction.convertion_type == Some(TokenType::BadToken){
        return Err(format!(
            "Engine VM: Convert Instruction -> {} `{:?}`, instruction line: {}, line: {}.",
            "Unknown Conversion type",
            instruction.convertion_type.as_ref().unwrap(),
            instruction.line.unwrap(), current_line));
    }

    return Ok(());
}


fn execute_convert_to_bool(
    current_line: u128,
    environment: &mut Environment,
    instruction: ConvertInstruction
) -> Result<(), String>{

    /* Retrieve From Variable */
    let from_variable = environment.variables.get(
        instruction.convert_from.as_ref().unwrap());
    if from_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_from.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let from_variable = from_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve To Variable */
    let to_variable = environment.variables.get(
        instruction.convert_to.as_ref().unwrap());
    if to_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_to.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let to_variable = to_variable.as_ref().unwrap().as_ref().unwrap();

    if to_variable.variable_type != Some(TokenType::Bool){
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't convert type `{:?}` to `Bool`",
                to_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        variable_type: Some(TokenType::Bool),
        name: Some(instruction.convert_to.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Boolean),
            boolean: Some(
                if from_variable.variable_type == Some(TokenType::Bool){
                    from_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone()
                }
                else if from_variable.variable_type == Some(TokenType::String){
                    let temp_value = from_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone();
                    let temp_value = temp_value.parse::<bool>();
                    if temp_value.is_err(){
                        return Err(format!(
                            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
                            "Can't convert type `String` to type `Bool`",
                            instruction.line.unwrap(), current_line));
                    }
                    temp_value.unwrap()
                }
                else {
                    return Err(format!(
                        "Engine VM: Convert Instruction -> Can't convert `{}` to `Bool`, instruction line: {}, line: {}.",
                        if from_variable.variable_type == Some(TokenType::Int){
                            from_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone().to_string()
                        }
                        else if from_variable.variable_type == Some(TokenType::Double){
                            from_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone().to_string()
                        }
                        else if from_variable.variable_type == Some(TokenType::Char){
                            from_variable.value.as_ref().unwrap().character.as_ref().unwrap().clone().to_string()
                        }
                        else{
                            String::from("")
                        },
                        instruction.line.unwrap(), current_line));
                }
            ),
            character: None,
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.convert_to.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_convert_to_int(
    current_line: u128,
    environment: &mut Environment,
    instruction: ConvertInstruction
) -> Result<(), String>{

    /* Retrieve From Variable */
    let from_variable = environment.variables.get(
        instruction.convert_from.as_ref().unwrap());
    if from_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_from.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let from_variable = from_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve To Variable */
    let to_variable = environment.variables.get(
        instruction.convert_to.as_ref().unwrap());
    if to_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_to.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let to_variable = to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Check Values */
    if to_variable.variable_type != Some(TokenType::Int) &&
        to_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is not of type `Int` or `Double`",
                to_variable.name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        variable_type: Some(TokenType::Int),
        name: Some(instruction.convert_to.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Integer),
            boolean: None,
            character: None,
            double: None,
            int: Some(
                if from_variable.variable_type == Some(TokenType::Int){
                    from_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone()
                }
                else if from_variable.variable_type == Some(TokenType::Double) {
                    from_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone() as i64
                }
                else if from_variable.variable_type == Some(TokenType::String){
                    let temp_value = from_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone();
                    let temp_value = temp_value.parse::<i64>();
                    if temp_value.is_err(){
                        return Err(format!(
                            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
                            "Can't convert type `String` to type `Int`",
                            instruction.line.unwrap(), current_line));
                    }
                    temp_value.unwrap()
                }
                else {
                    return Err(format!(
                        "Engine VM: Convert Instruction -> Can't convert `{}` to `Int`, instruction line: {}, line: {}.",
                        if from_variable.variable_type == Some(TokenType::Bool){
                            from_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone().to_string()
                        }
                        else if from_variable.variable_type == Some(TokenType::Char){
                            from_variable.value.as_ref().unwrap().character.as_ref().unwrap().clone().to_string()
                        }
                        else{
                            String::from("")
                        },
                        instruction.line.unwrap(), current_line));
                }
            ),
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.convert_to.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_convert_to_double(
    current_line: u128,
    environment: &mut Environment,
    instruction: ConvertInstruction
) -> Result<(), String>{

    /* Retrieve From Variable */
    let from_variable = environment.variables.get(
        instruction.convert_from.as_ref().unwrap());
    if from_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_from.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let from_variable = from_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve To Variable */
    let to_variable = environment.variables.get(
        instruction.convert_to.as_ref().unwrap());
    if to_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_to.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let to_variable = to_variable.as_ref().unwrap().as_ref().unwrap();

    /* Check Values */
    if to_variable.variable_type != Some(TokenType::Int) &&
        to_variable.variable_type != Some(TokenType::Double)
    {
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is not of type `Int` or `Double`",
                to_variable.name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        variable_type: Some(TokenType::Double),
        name: Some(instruction.convert_to.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Double),
            boolean: None,
            character: None,
            double: Some(
                if from_variable.variable_type == Some(TokenType::Int){
                    from_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone() as f64
                }
                else if from_variable.variable_type == Some(TokenType::Double){
                    from_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone()
                }
                else if from_variable.variable_type == Some(TokenType::String){
                    let temp_value = from_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone();
                    let temp_value = temp_value.parse::<f64>();
                    if temp_value.is_err(){
                        return Err(format!(
                            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
                            "Can't convert type `String` to type `Double`",
                            instruction.line.unwrap(), current_line));
                    }
                    temp_value.unwrap()
                }
                else {
                    return Err(format!(
                        "Engine VM: Convert Instruction -> Can't convert `{}` to `Double`, instruction line: {}, line: {}.",
                        if from_variable.variable_type == Some(TokenType::Bool){
                            from_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone().to_string()
                        }
                        else if from_variable.variable_type == Some(TokenType::Char){
                            from_variable.value.as_ref().unwrap().character.as_ref().unwrap().clone().to_string()
                        }
                        else{
                            String::from("")
                        },
                        instruction.line.unwrap(), current_line));
                }
            ),
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.convert_to.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_convert_to_char(
    current_line: u128,
    environment: &mut Environment,
    instruction: ConvertInstruction
) -> Result<(), String>{

    /* Retrieve From Variable */
    let from_variable = environment.variables.get(
        instruction.convert_from.as_ref().unwrap());
    if from_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_from.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let from_variable = from_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve To Variable */
    let to_variable = environment.variables.get(
        instruction.convert_to.as_ref().unwrap());
    if to_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_to.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let to_variable = to_variable.as_ref().unwrap().as_ref().unwrap();

    if to_variable.variable_type != Some(TokenType::Char) &&
        to_variable.variable_type != Some(TokenType::String)
    {
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is not of type `Char` or `String`",
                to_variable.name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        variable_type: Some(TokenType::Char),
        name: Some(instruction.convert_to.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::Character),
            boolean: None,
            character: Some(
                if from_variable.variable_type == Some(TokenType::Char){
                    from_variable.value.as_ref().unwrap().character.as_ref().unwrap().clone()
                }
                else if from_variable.variable_type == Some(TokenType::String){
                    let temp_value = from_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone();
                    if temp_value.len() > 1{
                        temp_value.chars().nth(0).unwrap()
                    }
                    else{
                        let temp_value = temp_value.parse::<char>();
                        if temp_value.is_err(){
                            return Err(format!(
                                "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
                                "Can't convert type `String` to type `Char`",
                                instruction.line.unwrap(), current_line));
                        }
                        temp_value.unwrap()
                    }
                }
                else {
                    return Err(format!(
                        "Engine VM: Convert Instruction -> Can't convert `{}` to `Char`, instruction line: {}, line: {}.",
                        if from_variable.variable_type == Some(TokenType::Bool){
                            from_variable.value.as_ref().unwrap().boolean.as_ref().unwrap().clone().to_string()
                        }
                        else if from_variable.variable_type == Some(TokenType::Int){
                            from_variable.value.as_ref().unwrap().int.as_ref().unwrap().clone().to_string()
                        }
                        else if from_variable.variable_type == Some(TokenType::Double){
                            from_variable.value.as_ref().unwrap().double.as_ref().unwrap().clone().to_string()
                        }
                        else{
                            String::from("")
                        },
                        instruction.line.unwrap(), current_line));
                }
            ),
            double: None,
            int: None,
            string: None,
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.convert_to.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}


fn execute_convert_to_string(
    current_line: u128,
    environment: &mut Environment,
    instruction: ConvertInstruction
) -> Result<(), String>{

    /* Retrieve From Variable */
    let from_variable = environment.variables.get(
        instruction.convert_from.as_ref().unwrap());
    if from_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_from.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let from_variable = from_variable.as_ref().unwrap().as_ref().unwrap();

    /* Retrieve To Variable */
    let to_variable = environment.variables.get(
        instruction.convert_to.as_ref().unwrap());

    if to_variable == None{
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is undefined",
                instruction.convert_to.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    let to_variable = to_variable.as_ref().unwrap().as_ref().unwrap();

    if from_variable.variable_type != Some(TokenType::Char) &&
        from_variable.variable_type != Some(TokenType::String)
    {
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Can't convert from type `{:?}` to `String`",
                from_variable.variable_type.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }
    else if to_variable.variable_type != Some(TokenType::Char) &&
        to_variable.variable_type != Some(TokenType::String)
    {
        return Err(format!(
            "Engine VM: Convert Instruction -> {}, instruction line: {}, line: {}.",
            format!(
                "Variable `{}` is not of type `Char` or `String`",
                to_variable.name.as_ref().unwrap()),
            instruction.line.unwrap(), current_line));
    }

    let variable = Variable{
        variable_type: Some(TokenType::String),
        name: Some(instruction.convert_to.as_ref().unwrap().clone()),
        value: Some(Value{
            value_type: Some(ValueType::String),
            boolean: None,
            character: None,
            double: None,
            int: None,
            string: Some(
                if from_variable.variable_type == Some(TokenType::String){
                    from_variable.value.as_ref().unwrap().string.as_ref().unwrap().clone()
                } else {
                    String::from(from_variable.value.as_ref().unwrap().character.as_ref().unwrap().clone())
                }
            ),
            string_value: None
        })
    };

    environment.variables.insert(
        instruction.convert_to.as_ref().unwrap().clone(),
        Some(variable));

    return Ok(());
}
