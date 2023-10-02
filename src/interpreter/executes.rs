use std::collections::HashMap;

use crate::interpreter::symantic_analyzer::Analyzer;
use crate::tokens::TokenType;
use crate::environments::{
    Variable,
    Value,
    ValueType,
    Environment,
    EnvironmentScope
};
use crate::syntax_tree::{
    DefineBoolNode,
    DefineVariableNode,
    DefineIntNode,
    DefineDoubleNode,
    DefineCharNode,
    DefineStringNode,
    DefineVarNode,
    DefinePrintNode,
    OperationNode,
    OperatorType,
    StatementNode,
    StatementType,
    DefineIfStatementNode,
    StatementsNode
};
use crate::interpreter::symantic_analyzer::{
    analyze_define_bool,
    analyze_define_int,
    analyze_define_double,
    analyze_define_char,
    analyze_define_string,
    analyze_define_var,
    analyze_define_variable,
    analyze_define_print,
    analyze_if_condition,
    is_variable_exists,
    get_variable,
    insert_variable_into_current_environmment
};


pub fn execute_statement(
    analyzer: &mut Analyzer, node: &StatementNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    if node.statement_type.as_ref().unwrap() == &StatementType::DefineBool{
        analyze_define_bool(
            &mut analyzer, &(node.define_bool_statement.as_ref().unwrap()))?;

        define_bool(
            &mut analyzer, &(node.define_bool_statement.as_ref().unwrap()))?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::DefineInt{
        analyze_define_int(
            &mut analyzer, node.define_int_statement.as_ref().unwrap().clone())?;

        define_int(
            &mut analyzer, node.define_int_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::DefineDouble{
        analyze_define_double(
            &mut analyzer, node.define_double_statement.as_ref().unwrap().clone())?;

        define_double(
            &mut analyzer, node.define_double_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::DefineChar{
        analyze_define_char(
            &mut analyzer, node.define_char_statement.as_ref().unwrap().clone())?;

        define_char(
            &mut analyzer, node.define_char_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::DefineString{
        analyze_define_string(
            &mut analyzer, node.define_string_statement.as_ref().unwrap().clone())?;

        define_string(
            &mut analyzer, node.define_string_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::DefineVar{
        analyze_define_var(
            &mut analyzer, node.define_var_statement.as_ref().unwrap().clone())?;

        define_var(
            &mut analyzer, node.define_var_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::DefineVariable{
        analyze_define_variable(
            &mut analyzer, node.define_variable_statement.as_ref().unwrap().clone())?;

        define_variable(
            &mut analyzer, node.define_variable_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type.as_ref().unwrap() == &StatementType::Print{
        analyze_define_print(
            &mut analyzer, node.define_print_statement.as_ref().unwrap().clone())?;

        define_print(
            &mut analyzer, node.define_print_statement.as_ref().unwrap().clone())?;
    }
    else if node.statement_type == Some(StatementType::DefineIf){
        execute_define_if_statement(
            &mut analyzer, node.define_if_statement.as_ref().unwrap())?;
    }

    return Ok(());
}


pub fn define_bool(
    analyzer: &mut Analyzer,
    statement: &DefineBoolNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let node_value = execute_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Bool);
    variable.value = Some(node_value);

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

pub fn define_int(
    analyzer: &mut Analyzer,
    statement: DefineIntNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let mut node_value = execute_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_value.value_type.as_ref().unwrap() == &ValueType::Double{
        node_value.value_type = Some(ValueType::Integer);
        node_value.int = Some(node_value.double.unwrap() as i64);
        node_value.double = None;
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Int);
    variable.value = Some(node_value);

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

pub fn define_double(
    analyzer: &mut Analyzer,
    statement: DefineDoubleNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let mut node_value = execute_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_value.value_type.as_ref().unwrap() == &ValueType::Integer{
        node_value.value_type = Some(ValueType::Double);
        node_value.double = Some(node_value.int.unwrap() as f64);
        node_value.int = None;
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Double);
    variable.value = Some(node_value);

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

pub fn define_char(
    analyzer: &mut Analyzer,
    statement: DefineCharNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let mut node_value = execute_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_value.value_type.as_ref().unwrap() == &ValueType::String{
        node_value.value_type = Some(ValueType::Character);
        node_value.character = Some(
            node_value.string.unwrap().chars().nth(0).unwrap());
        node_value.string = None;
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Char);
    variable.value = Some(node_value);

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

pub fn define_string(
    analyzer: &mut Analyzer,
    statement: DefineStringNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let mut node_value = execute_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_value.value_type.as_ref().unwrap() == &ValueType::Character{
        node_value.value_type = Some(ValueType::String);
        node_value.string = Some(String::from(node_value.character.unwrap()));
        node_value.character = None;
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::String);
    variable.value = Some(node_value);

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

pub fn define_var(
    analyzer: &mut Analyzer,
    statement: DefineVarNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let mut node_value = execute_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    if node_value.value_type.as_ref().unwrap() == &ValueType::Boolean{
        variable.variable_type = Some(TokenType::Bool);
    }
    else if node_value.value_type.as_ref().unwrap() == &ValueType::Integer{
        variable.variable_type = Some(TokenType::Int);
    }
    else if node_value.value_type.as_ref().unwrap() == &ValueType::Double{
        variable.variable_type = Some(TokenType::Double);
    }
    else if node_value.value_type.as_ref().unwrap() == &ValueType::Character{
        variable.variable_type = Some(TokenType::String);

        node_value.value_type = Some(ValueType::String);
        node_value.string = Some(String::from(node_value.character.unwrap()));
        node_value.character = None;
    }
    else if node_value.value_type.as_ref().unwrap() == &ValueType::String{
        variable.variable_type = Some(TokenType::String);
    }
    variable.value = Some(node_value);

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

pub fn define_variable(
    analyzer: &mut Analyzer,
    statement: DefineVariableNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    if !is_variable_exists(&analyzer, &statement.name.as_ref().unwrap().value){
        let mut node_value = execute_operation_node(
            &analyzer, statement.left.as_ref().unwrap())?;

        let mut variable = Variable::new();

        variable.name = Some(statement.name.as_ref().unwrap().value.clone());
        if node_value.value_type.as_ref().unwrap() == &ValueType::Boolean{
            variable.variable_type = Some(TokenType::Bool);
        }
        else if node_value.value_type.as_ref().unwrap() == &ValueType::Integer{
            variable.variable_type = Some(TokenType::Int);
        }
        else if node_value.value_type.as_ref().unwrap() == &ValueType::Double{
            variable.variable_type = Some(TokenType::Double);
        }
        else if node_value.value_type.as_ref().unwrap() == &ValueType::Character{
            variable.variable_type = Some(TokenType::String);
            node_value.value_type = Some(ValueType::String);
            node_value.string = Some(String::from(node_value.character.unwrap()));
            node_value.character = None;
        }
        else if node_value.value_type.as_ref().unwrap() == &ValueType::String{
            variable.variable_type = Some(TokenType::String);
        }
        variable.value = Some(node_value);

        insert_variable_into_current_environmment(&mut analyzer, variable);
    }
    else{
        let name = statement.name.as_ref().unwrap().value.clone();
        let variable = get_variable(&analyzer, &name)?;
        let variable_type = variable.variable_type.as_ref().unwrap().clone();
        let operator_type = statement.operator.as_ref().unwrap().token_type.clone();

        if operator_type == TokenType::Assign{
            let node_value = execute_operation_node(
                &analyzer, statement.left.as_ref().unwrap())?;

            let mut variable = Variable::new();

            variable.name = Some(statement.name.as_ref().unwrap().value.clone());
            if node_value.value_type == Some(ValueType::Boolean){
                variable.variable_type = Some(TokenType::Bool);
            }
            else if node_value.value_type == Some(ValueType::Integer){
                variable.variable_type = Some(TokenType::Int);
            }
            else if node_value.value_type == Some(ValueType::Double){
                variable.variable_type = Some(TokenType::Double);
            }
            else if node_value.value_type == Some(ValueType::Character){
                variable.variable_type = Some(TokenType::String);
            }
            else if node_value.value_type == Some(ValueType::String){
                variable.variable_type = Some(TokenType::String);
            }
            variable.value = Some(node_value);

            insert_variable_into_current_environmment(&mut analyzer, variable);
            return Ok(());
        }

        let mut node_value = execute_operation_node(
            &analyzer, statement.left.as_ref().unwrap())?;

        let mut new_variable = Variable::new();
        new_variable.name = Some(name.clone());

        if variable_type == TokenType::String{
            new_variable.variable_type = Some(TokenType::String);

            if node_value.value_type == Some(ValueType::Character){
                node_value.value_type = Some(ValueType::String);
                node_value.string = Some(
                    String::from(node_value.character.unwrap()));
                node_value.character = None;
            }

            let old_value_string = variable.value.as_ref().unwrap()
                                            .string.as_ref().unwrap().clone();
            node_value.string = Some(
                old_value_string + &node_value.string.unwrap().clone()
            );

            new_variable.value = Some(node_value);
        }
        else if variable_type == TokenType::Double{
            new_variable.variable_type = Some(TokenType::Double);

            if node_value.value_type == Some(ValueType::Integer){
                node_value.value_type = Some(ValueType::Double);
                node_value.double = Some(node_value.int.unwrap() as f64);
                node_value.int = None;
            }

            let old_value_double = variable.value.as_ref().unwrap()
                                            .double.as_ref().unwrap();
            if operator_type == TokenType::PlusEqual{
                node_value.double = Some(
                    old_value_double + node_value.double.unwrap());
            }
            else if operator_type == TokenType::MinusEqual{
                node_value.double = Some(
                    old_value_double - node_value.double.unwrap());
            }
            else if operator_type == TokenType::MulEqual{
                node_value.double = Some(
                    old_value_double * node_value.double.unwrap());
            }
            else if operator_type == TokenType::DivEqual{
                node_value.double = Some(
                    old_value_double / node_value.double.unwrap());
            }
            else{
                node_value.double = Some(
                    old_value_double % node_value.double.unwrap());
            }

            new_variable.value = Some(node_value);
        }
        else if variable_type == TokenType::Int{
            new_variable.variable_type = Some(TokenType::Int);

            if node_value.value_type == Some(ValueType::Double){
                node_value.value_type = Some(ValueType::Integer);
                node_value.int = Some(node_value.double.unwrap() as i64);
                node_value.double = None;
            }

            let old_value_int = variable.value.as_ref().unwrap()
                                        .int.as_ref().unwrap();
            if operator_type == TokenType::PlusEqual{
                node_value.int = Some(old_value_int + node_value.int.unwrap());
            }
            else if operator_type == TokenType::MinusEqual{
                node_value.int = Some(old_value_int - node_value.int.unwrap());
            }
            else if operator_type == TokenType::MulEqual{
                node_value.int = Some(old_value_int * node_value.int.unwrap());
            }
            else if operator_type == TokenType::DivEqual{
                node_value.int = Some(old_value_int / node_value.int.unwrap());
            }
            else{
                node_value.int = Some(old_value_int % node_value.int.unwrap());
            }

            new_variable.value = Some(node_value);
        }

        insert_variable_into_current_environmment(&mut analyzer, new_variable);
    }

    return Ok(());
}

pub fn define_print(
    analyzer: &Analyzer,
    statement: DefinePrintNode
) -> Result<(), String>{

    let node_value = execute_operation_node(
        &analyzer, statement.expression.as_ref().unwrap())?;

    if node_value.value_type == Some(ValueType::Boolean){
        if node_value.boolean == Some(true){
            print!("True");
        }
        else{
            print!("False");
        }
    }
    else if node_value.value_type == Some(ValueType::Integer){
        print!("{}", node_value.int.unwrap());
    }
    else if node_value.value_type == Some(ValueType::Double){
        print!("{}", node_value.double.unwrap());
    }
    else if node_value.value_type == Some(ValueType::Character){
        print!("{}", node_value.character.unwrap());
    }
    else if node_value.value_type == Some(ValueType::String){
        print!("{}", node_value.string.as_ref().unwrap());
    }

    use std::io::Write;

    /* Flush Stdin */
    if std::io::stdout().flush().is_err(){
        return Err(format!(
            "Engine Interpreter: Execute Error -> Failed to print to console."));
    }

    return Ok(());
}


fn execute_define_if_statement(
    analyzer: &mut Analyzer,
    statement: &DefineIfStatementNode
) -> Result<(), String>{

    let mut is_if_executed = false;
    let mut analyzer = analyzer;

    /* Execute If Statement */
    {
        analyzer.environments_stack.push_front(Environment {
            scope: EnvironmentScope::If,
            variables: HashMap::new()
        });

        let define_if_node = statement.define_if_node.as_ref().unwrap();

        analyze_if_condition(
            &analyzer, define_if_node.condition.as_ref().unwrap(),
            define_if_node.token.as_ref().unwrap())?;

        let condition_result = execute_if_condition(
            &analyzer, define_if_node.condition.as_ref().unwrap())?;

        if condition_result.boolean == Some(true){
            is_if_executed = true;

            execute_statements(
                &mut analyzer, &define_if_node.statements)?;
        }

        analyzer.environments_stack.pop_front();
    }

    /* Analyze If Else Statements */
    {
        for define_if_else_node in &statement.define_if_else_nodes{
            if is_if_executed{
                break;
            }

            analyzer.environments_stack.push_front(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new()
            });

            analyze_if_condition(
                &analyzer, define_if_else_node.condition.as_ref().unwrap(),
                define_if_else_node.token.as_ref().unwrap())?;

            let condition_result = execute_if_condition(
                &analyzer, define_if_else_node.condition.as_ref().unwrap())?;

            if condition_result.boolean == Some(true){
                is_if_executed = true;

                execute_statements(
                    &mut analyzer, &define_if_else_node.statements)?;
            }

            analyzer.environments_stack.pop_front();
        }
    }

    /* Analyze Else Statement */
    {
        if statement.define_else_node != None && !is_if_executed{
            analyzer.environments_stack.push_front(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new()
            });

            let define_else_node = statement.define_else_node.as_ref().unwrap();

            execute_statements(
                &mut analyzer, &define_else_node.statements)?;

            analyzer.environments_stack.pop_front();
        }
    }

    return Ok(());
}


fn execute_statements(
    analyzer: &mut Analyzer,
    statements: &StatementsNode
) -> Result<(), String>{

    for statement in &statements.statements{
        execute_statement(analyzer, statement)?;
    }

    return Ok(());
}


fn execute_if_condition(
    analyzer: &Analyzer,
    condition: &OperationNode
) -> Result<Value, String>{

    let node_value = execute_operation_node(&analyzer, condition)?;

    return Ok(node_value);
}


fn get_double_operations_value(operator: &OperatorType, v1: f64, v2: f64) -> f64{
    if operator == &OperatorType::Plus{
        return v1 + v2;
    }
    else if operator == &OperatorType::Minus{
        return v1 - v2;
    }
    else if operator == &OperatorType::Mul{
        return v1 * v2;
    }
    else if operator == &OperatorType::Div{
        return v1 / v2;
    }
    return v1 % v2;
}


fn get_boolean_operations_value(operator: &OperatorType, v1: f64, v2: f64) -> bool{
    if operator == &OperatorType::Equal{
        return v1 == v2;
    }
    else if operator == &OperatorType::NotEqual{
        return v1 != v2;
    }
    else if operator == &OperatorType::GreaterThan{
        return v1 > v2;
    }
    else if operator == &OperatorType::GreaterThanOrEqual{
        return v1 >= v2;
    }
    else if operator == &OperatorType::LessThan{
        return v1 < v2;
    }
    return v1 <= v2;
}

fn execute_operation_node(
    analyzer: &Analyzer,
    operation_node: &OperationNode
) -> Result<Value, String>{

    match &operation_node.operator{
        Some(operator) => {
            let mut left_value: Value = Value::new();
            let mut right_value: Value = Value::new();

            if operation_node.left != None{
                left_value = execute_operation_node(
                    analyzer, &operation_node.left.as_ref().unwrap())?;
            }
            if operation_node.right != None{
                right_value = execute_operation_node(
                    analyzer, &operation_node.right.as_ref().unwrap())?;
            }

            if operation_node.left != None && operation_node.right == None{
                return Ok(left_value);
            }
            else if operation_node.right != None && operation_node.left == None{
                return Ok(right_value);
            }

            /* Input Operation */
            if operator == &OperatorType::Convert{
                /* Input */
                let mut line = String::new();
                if std::io::stdin().read_line(&mut line).is_err(){
                    return Err(format!(
                        "Engine Interpreter: Execute Error -> {}, line {}:{}.",
                        "Failed to read line",
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }

                let line = String::from(line.trim_end());

                let mut new_value = Value::new();

                /* Convert To Type */
                if operation_node.value.as_ref().unwrap().token_type == TokenType::Bool{
                    let boolean_value = line.parse::<bool>();
                    if boolean_value.is_err(){
                        return Err(format!(
                            "Engine Interpreter: Execute Error -> {}, line {}:{}.",
                            "Failed to convert from `String` to `Bool`",
                            operation_node.value.as_ref().unwrap().start_line,
                            operation_node.value.as_ref().unwrap().start_pos));
                    }

                    new_value.value_type = Some(ValueType::Boolean);
                    new_value.boolean = Some(boolean_value.unwrap());
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Int{
                    let int_value = line.parse::<i64>();
                    if int_value.is_err(){
                        return Err(format!(
                            "Engine Interpreter: Execute Error -> {}, line {}:{}.",
                            "Failed to convert from `String` to `Int`",
                            operation_node.value.as_ref().unwrap().start_line,
                            operation_node.value.as_ref().unwrap().start_pos));
                    }

                    new_value.value_type = Some(ValueType::Integer);
                    new_value.int = Some(int_value.unwrap());
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Double{
                    let double_value = line.parse::<f64>();
                    if double_value.is_err(){
                        return Err(format!(
                            "Engine Interpreter: Execute Error -> {}, line {}:{}.",
                            "Failed to convert from `String` to `Double`",
                            operation_node.value.as_ref().unwrap().start_line,
                            operation_node.value.as_ref().unwrap().start_pos));
                    }

                    new_value.value_type = Some(ValueType::Double);
                    new_value.double = Some(double_value.unwrap());
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Char{
                    let char_value = line.parse::<char>();
                    if char_value.is_err(){
                        return Err(format!(
                            "Engine Interpreter: Execute Error -> {}, line {}:{}.",
                            "Failed to convert from `String` to `Char`",
                            operation_node.value.as_ref().unwrap().start_line,
                            operation_node.value.as_ref().unwrap().start_pos));
                    }

                    new_value.value_type = Some(ValueType::Character);
                    new_value.character = Some(char_value.unwrap());
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::String{
                    new_value.value_type = Some(ValueType::String);
                    new_value.string = Some(line);
                }

                return Ok(new_value);
            }

            // Check Double Values
            if left_value.value_type == Some(ValueType::Double) ||
                right_value.value_type == Some(ValueType::Double)
            {

                let mut new_value = Value::new();

                let mut _v1 = 0.0;
                let mut _v2 = 0.0;

                if left_value.value_type == Some(ValueType::Double){
                    _v1 = left_value.double.unwrap();
                }
                else{
                    _v1 = left_value.int.unwrap() as f64;
                }

                if right_value.value_type == Some(ValueType::Double){
                    _v2 = right_value.double.unwrap();
                }
                else{
                    _v2 = right_value.int.unwrap() as f64;
                }

                if operator == &OperatorType::Plus ||
                    operator == &OperatorType::Minus ||
                    operator == &OperatorType::Mul ||
                    operator == &OperatorType::Div ||
                    operator == &OperatorType::Mod
                {

                    new_value.value_type = Some(ValueType::Double);
                    new_value.double = Some(
                        get_double_operations_value(operator, _v1, _v2));
                }
                else{
                    new_value.boolean = Some(
                        get_boolean_operations_value(operator, _v1, _v2)
                    );
                    new_value.value_type = Some(ValueType::Boolean);
                }

                return Ok(new_value);
            }

            // Check Interger Values
            else if left_value.value_type == Some(ValueType::Integer) ||
                right_value.value_type == Some(ValueType::Integer){

                let mut new_value = Value::new();

                if operator == &OperatorType::Plus ||
                    operator == &OperatorType::Minus ||
                    operator == &OperatorType::Mul ||
                    operator == &OperatorType::Mod
                {

                    new_value.value_type = Some(ValueType::Integer);
                    new_value.int = Some(
                        get_double_operations_value(
                            &operator, left_value.int.unwrap() as f64,
                            right_value.int.unwrap() as f64) as i64
                    );
                }
                else if operator == &OperatorType::Div{

                    new_value.value_type = Some(ValueType::Double);
                    new_value.double = Some(
                        get_double_operations_value(
                            &operator, left_value.int.unwrap() as f64,
                            right_value.int.unwrap() as f64)
                    );
                }
                else{
                    new_value.boolean = Some(
                        get_boolean_operations_value(
                            &operator, left_value.int.unwrap() as f64,
                            right_value.int.unwrap() as f64)
                    );
                    new_value.value_type = Some(ValueType::Boolean);
                }

                return Ok(new_value);
            }

            // Check Boolean Values
            else if left_value.value_type == Some(ValueType::Boolean) ||
                    right_value.value_type == Some(ValueType::Boolean){

                let mut new_value = Value::new();
                let v1 = left_value.boolean.unwrap();
                let v2 = right_value.boolean.unwrap();

                new_value.value_type = Some(ValueType::Boolean);
                if operator == &OperatorType::Equal{
                    new_value.boolean = Some(v1 == v2);
                }
                else if operator == &OperatorType::NotEqual{
                    new_value.boolean = Some(v1 != v2);
                }
                else if operator == &OperatorType::Or{
                    new_value.boolean = Some(v1 || v2);
                }
                else if operator == &OperatorType::And{
                    new_value.boolean = Some(v1 && v2);
                }
                else if operator == &OperatorType::GreaterThan{
                    new_value.boolean = Some(v1 > v2);
                }
                else if operator == &OperatorType::GreaterThanOrEqual{
                    new_value.boolean = Some(v1 >= v2);
                }
                else if operator == &OperatorType::LessThan{
                    new_value.boolean = Some(v1 < v2);
                }
                else{
                    new_value.boolean = Some(v1 <= v2);
                }

                return Ok(new_value);
            }

            // Check String Values
            else if left_value.value_type == Some(ValueType::String) ||
                    right_value.value_type == Some(ValueType::String){

                let mut new_value = Value::new();

                new_value.value_type = Some(ValueType::String);
                if left_value.value_type.as_ref().unwrap() == &ValueType::Character{
                    new_value.string = Some(
                        String::from(left_value.character.unwrap()) +
                        &right_value.string.unwrap().clone()
                    );
                }
                else if right_value.value_type.as_ref().unwrap() == &ValueType::Character{
                    new_value.string = Some(
                        left_value.string.unwrap().clone() +
                        &String::from(right_value.character.unwrap())
                    );
                }
                else{
                    new_value.string = Some(
                        left_value.string.unwrap().clone() +
                        &right_value.string.unwrap().clone()
                    );
                }

                return Ok(new_value);
            }

            // Check Character Values
            else if left_value.value_type == Some(ValueType::Character) ||
                    right_value.value_type == Some(ValueType::Character){

                let mut new_value = Value::new();

                new_value.value_type = Some(ValueType::String);
                new_value.string = Some(
                    String::from(left_value.character.unwrap()) +
                    &String::from(right_value.character.unwrap())
                );
                return Ok(new_value);
            }

            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!("Undefined operator {:?} behavior", operator),
                operation_node.value.as_ref().unwrap().start_line,
                operation_node.value.as_ref().unwrap().start_pos));
        },
        None => {
            let operation_value = operation_node.value.as_ref().unwrap();

            let mut value = Value::new();
            value.string_value = Some(operation_value.value.clone());

            if operation_value.token_type == TokenType::Variable{
                let variable = get_variable(
                    analyzer, &operation_node.value.as_ref().unwrap().value)?;
                let value = variable.value.as_ref().unwrap();

                return Ok(value.clone());
            }
            else if operation_value.token_type == TokenType::True{
                value.value_type = Some(ValueType::Boolean);
                value.boolean = Some(true);
            }
            else if operation_value.token_type == TokenType::False{
                value.value_type = Some(ValueType::Boolean);
                value.boolean = Some(false);
            }
            else if operation_value.token_type == TokenType::IntNumber{
                value.value_type = Some(ValueType::Integer);
                value.int = Some(
                    operation_value.value.clone().parse::<i64>().unwrap());
            }
            else if operation_value.token_type == TokenType::DoubleNumber{
                value.value_type = Some(ValueType::Double);
                value.double = Some(
                    operation_value.value.clone().parse::<f64>().unwrap());
            }
            else if operation_value.token_type == TokenType::Character{
                value.value_type = Some(ValueType::Character);
                value.character = Some(
                    operation_value.value.clone().chars().nth(0).unwrap());
            }
            else if operation_value.token_type == TokenType::StringSequence{
                value.value_type = Some(ValueType::String);
                value.string = Some(operation_value.value.clone());
            }

            return Ok(value);
        }
    }
}
