use std::collections::{
    HashMap,
    VecDeque
};

use crate::interpreter::tokens::{
    TokenType,
    Token
};
use crate::interpreter::environments::{
    Environment,
    EnvironmentScope,
    Variable
};
use crate::interpreter::syntax_tree::{
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
};


#[derive(Debug)]
pub struct Analyzer{
    pub environments_stack: VecDeque<Environment>,
}

impl Analyzer{
    pub fn new() -> Self{
        let mut environments_stack = VecDeque::new();
        environments_stack.push_front(Environment {
            scope: EnvironmentScope::Main,
            variables: HashMap::new()
        });

        return Analyzer{
            environments_stack
        };
    }
}


pub fn is_variable_exists(
    analyzer: &Analyzer, variable_name: &String
) -> bool{

    for environment in &analyzer.environments_stack{
        if environment.variables.contains_key(variable_name){
            return true;
        }
    }

    return false;
}


pub fn insert_variable_into_current_environmment(
    analyzer: &mut Analyzer, variable: Variable
){
    analyzer.environments_stack.back_mut().unwrap().variables.insert(
        variable.name.as_ref().unwrap().clone(),
        Some(variable));
}


pub fn get_variable(
    analyzer: &Analyzer, variable_name: &String
) -> Result<Variable, String>{

    for environment in &analyzer.environments_stack{
        if environment.variables.contains_key(variable_name){
            return Ok(environment.variables.get(
                variable_name).unwrap().as_ref().unwrap().clone());
        }
    }

    return Err(format!(
        "Engine Interpreter: Analyzer Error -> Variable `{}` not found.",
        variable_name));
}


fn get_variable_type(
    analyzer: &Analyzer, variable_name: &String
) -> Result<TokenType, String>{

    for environment in &analyzer.environments_stack{
        if environment.variables.contains_key(variable_name){
            return Ok(
                environment.variables.get(variable_name)
                    .as_ref().unwrap().as_ref().unwrap()
                    .variable_type.as_ref().unwrap().clone());
        }
    }

    return Err(format!(
        "Engine Interpreter: Analyzer Error -> Variable `{}` not found.",
        variable_name));
}


pub fn analyze_define_bool(
    analyzer: &mut Analyzer,
    statement: &DefineBoolNode
) -> Result<(), String>{

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::Bool &&
        node_type != TokenType::True &&
        node_type != TokenType::False
    {
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Boolean", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    return Ok(());
}


pub fn analyze_define_int(
    analyzer: &mut Analyzer,
    statement: DefineIntNode
) -> Result<(), String>{

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::IntNumber && node_type != TokenType::DoubleNumber{
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Int", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    return Ok(());
}


pub fn analyze_define_double(
    analyzer: &mut Analyzer,
    statement: DefineDoubleNode
) -> Result<(), String>{

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::IntNumber && node_type != TokenType::DoubleNumber{
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Double", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    return Ok(());
}


pub fn analyze_define_char(
    analyzer: &mut Analyzer,
    statement: DefineCharNode
) -> Result<(), String>{

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::Character && node_type != TokenType::StringSequence{
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Character", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    return Ok(());
}


pub fn analyze_define_string(
    analyzer: &mut Analyzer,
    statement: DefineStringNode
) -> Result<(), String>{

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::StringSequence && node_type != TokenType::Character{
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to String", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    return Ok(());
}


pub fn analyze_define_var(
    analyzer: &mut Analyzer,
    statement: DefineVarNode
) -> Result<(), String>{

    analyze_operation_node(&analyzer, statement.left.as_ref().unwrap())?;

    return Ok(());
}

pub fn analyze_define_variable(
    analyzer: &mut Analyzer,
    statement: DefineVariableNode
) -> Result<(), String>{

    if !is_variable_exists(&analyzer, &statement.name.as_ref().unwrap().value){
        if statement.operator.as_ref().unwrap().token_type != TokenType::Assign{
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Undefined variable `{}`",
                    statement.name.as_ref().unwrap().value),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }
        analyze_operation_node(&analyzer, statement.left.as_ref().unwrap())?;
    }
    else{
        let name = statement.name.as_ref().unwrap().value.clone();
        let variable = get_variable(&analyzer, &name)?;
        let variable_type = variable.variable_type.as_ref().unwrap().clone();
        let operator_type = statement.operator.as_ref().unwrap().token_type.clone();

        if operator_type == TokenType::Assign{
            analyze_operation_node(&analyzer, statement.left.as_ref().unwrap())?;

            return Ok(());
        }

        // Validate Operator is Correct with returned node type
        if variable_type == TokenType::String && operator_type != TokenType::PlusEqual{
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Can't do operation `{:?}` to `{:?}` which has type `{:?}`",
                    operator_type,
                    variable.name.as_ref().unwrap(),
                    variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }
        else if variable_type == TokenType::Char{
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Can't do operation `{:?}` to `{:?}` which has type `{:?}`",
                    operator_type,
                    variable.name.as_ref().unwrap(),
                    variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }
        else if variable_type == TokenType::Bool{
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Can't do operation `{:?}` to `{:?}` which has type `{:?}`",
                    operator_type,
                    variable.name.as_ref().unwrap(),
                    variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }

        // Validate node type is correct with variable type
        let node_type = analyze_operation_node(
            &analyzer, statement.left.as_ref().unwrap())?;

        if (node_type == TokenType::IntNumber || node_type == TokenType::DoubleNumber) &&
            (variable_type != TokenType::Int && variable_type != TokenType::Double)
        {
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!("Can't assign `{:?}` to `{:?}`", node_type, variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }
        else if (node_type == TokenType::Character || node_type == TokenType::StringSequence) &&
            (variable_type != TokenType::String && variable_type != TokenType::Char)
        {
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Can't assign `{:?}` to `{:?}`",
                    node_type, variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }
    }

    return Ok(());
}


pub fn analyze_define_print(
    analyzer: &Analyzer,
    statement: DefinePrintNode
) -> Result<(), String>{

    if !is_variable_exists(&analyzer, &statement.variable.as_ref().unwrap().value){
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!(
                "Undefined variable `{}`",
                statement.variable.as_ref().unwrap().value),
            statement.variable.as_ref().unwrap().start_line,
            statement.variable.as_ref().unwrap().start_pos));
    }

    return Ok(());
}


pub fn analyze_if_condition(
    analyzer: &Analyzer,
    condition: &OperationNode,
    if_token: &Token
) -> Result<(), String>{

    let node_type = analyze_operation_node(&analyzer, condition)?;

    if node_type != TokenType::Bool && node_type != TokenType::True &&
        node_type != TokenType::False
    {
        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!("If condition must be of type `bool` found `{:?}`", node_type),
            if_token.start_line,
            if_token.start_pos));
    }

    return Ok(());
}


fn analyze_operation_node(
    analyzer: &Analyzer,
    operation_node: &OperationNode
) -> Result<TokenType, String>{

    match &operation_node.operator{
        Some(operator) => {
            let mut left_type: TokenType = TokenType::BadToken;
            let mut right_type: TokenType = TokenType::BadToken;

            if operation_node.left != None{
                left_type = analyze_operation_node(
                    analyzer, &operation_node.left.as_ref().unwrap())?;
            }
            if operation_node.right != None{
                right_type = analyze_operation_node(
                    analyzer, &operation_node.right.as_ref().unwrap())?;
            }

            if operation_node.left != None && operation_node.right == None{
                return Ok(left_type);
            }

            else if operation_node.right != None && operation_node.left == None{
                return Ok(right_type);
            }

            /* Check Input Operation */
            if operator == &OperatorType::Convert{
                if operation_node.value.as_ref().unwrap().token_type == TokenType::Bool{
                    return Ok(TokenType::Bool);
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Int{
                    return Ok(TokenType::IntNumber);
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Double{
                    return Ok(TokenType::DoubleNumber);
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Char{
                    return Ok(TokenType::Character);
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::String{
                    return Ok(TokenType::StringSequence);
                }
            }

            // Check String Types
            if left_type == TokenType::StringSequence{
                if right_type != TokenType::StringSequence &&
                    right_type != TokenType::Character
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Can't concate `{:?}` with String types",
                            right_type),
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_line,
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_pos));
                }
                else if operator != &OperatorType::Plus{
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on String Types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::StringSequence);
            }
            else if right_type == TokenType::StringSequence{
                if left_type != TokenType::StringSequence &&
                    left_type != TokenType::Character
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Can't concate `{:?}` with String types",
                            left_type),
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_line,
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_pos));
                }
                else if operator != &OperatorType::Plus{
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on String Types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::StringSequence);
            }

            // Check Character Types
            if left_type == TokenType::Character{
                if right_type != TokenType::Character{
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Can't concate `{:?}` with Character types",
                            right_type),
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_line,
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_pos));
                }
                else if operator != &OperatorType::Plus{
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Character types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::StringSequence);
            }
            else if right_type == TokenType::Character{
                if left_type != TokenType::Character{
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Can't concate `{:?}` with Character types",
                            left_type),
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_line,
                        operation_node.right.as_ref().unwrap()
                            .value.as_ref().unwrap().start_pos));
                }
                else if operator != &OperatorType::Plus{
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Character types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::StringSequence);
            }

            // Check IntNumber Types
            if left_type == TokenType::IntNumber{
                if right_type != TokenType::IntNumber &&
                    right_type != TokenType::DoubleNumber
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on {:?} and {:?}",
                            operator, left_type, right_type),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                else if operator == &OperatorType::Plus ||
                    operator == &OperatorType::Minus ||
                    operator == &OperatorType::Mul ||
                    operator == &OperatorType::Mod
                {
                    return Ok(TokenType::IntNumber);
                }
                else if operator == &OperatorType::Div{
                    return Ok(TokenType::DoubleNumber);
                }
                return Ok(TokenType::Bool);
            }
            else if right_type == TokenType::IntNumber{
                if left_type != TokenType::IntNumber &&
                    left_type != TokenType::DoubleNumber
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on {:?} and {:?}",
                            operator, right_type, left_type),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                else if operator == &OperatorType::Plus ||
                    operator == &OperatorType::Minus ||
                    operator == &OperatorType::Mul ||
                    operator == &OperatorType::Mod
                {
                    return Ok(TokenType::IntNumber);
                }
                else if operator == &OperatorType::Div{
                    return Ok(TokenType::DoubleNumber);
                }
                return Ok(TokenType::Bool);
            }

            // Check Boolean Types
            if left_type == TokenType::Bool ||
                left_type == TokenType::True ||
                left_type == TokenType::False
            {
                if right_type != TokenType::Bool &&
                    right_type != TokenType::True &&
                    right_type != TokenType::False
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Boolean types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                else if operator != &OperatorType::And &&
                    operator != &OperatorType::Or &&
                    operator != &OperatorType::Equal &&
                    operator != &OperatorType::NotEqual
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Boolean types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::Bool);
            }
            else if right_type == TokenType::Bool ||
                right_type == TokenType::True ||
                right_type == TokenType::False
            {
                if left_type != TokenType::Bool &&
                    left_type != TokenType::True &&
                    left_type != TokenType::False
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Boolean types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                else if operator != &OperatorType::And &&
                    operator != &OperatorType::Or &&
                    operator != &OperatorType::Equal &&
                    operator != &OperatorType::NotEqual
                {
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Boolean types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::Bool);
            }

            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!("Undefined operator {:?} behavior", operator),
                operation_node.value.as_ref().unwrap().start_line,
                operation_node.value.as_ref().unwrap().start_pos));
        },
        None => {
            if operation_node.value.as_ref().unwrap().token_type == TokenType::Variable{
                if !is_variable_exists(&analyzer, &operation_node.value.as_ref().unwrap().value){
                    return Err(format!(
                        "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Undefined variable `{}`",
                            &operation_node.value.as_ref().unwrap().value),
                        &operation_node.value.as_ref().unwrap().start_line,
                        &operation_node.value.as_ref().unwrap().start_pos));
                }
                let variable_type = get_variable_type(
                    &analyzer, &operation_node.value.as_ref().unwrap().value)?;
                if variable_type == TokenType::True || variable_type == TokenType::False{
                    return Ok(TokenType::Bool);
                }
                else if variable_type == TokenType::Int{
                    return Ok(TokenType::IntNumber);
                }
                else if variable_type == TokenType::Double{
                    return Ok(TokenType::DoubleNumber);
                }
                else if variable_type == TokenType::String{
                    return Ok(TokenType::StringSequence);
                }
                else if variable_type == TokenType::Char{
                    return Ok(TokenType::Character);
                }
                return Ok(variable_type);
            }
            return Ok(operation_node.value.as_ref().unwrap().token_type.clone());
        }
    }
}
