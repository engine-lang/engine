use std::collections::{
    HashMap,
    VecDeque
};

use crate::tokens::TokenType;
use crate::constants::{
    INT_NUMBER_MAX_LENGTH,
    DOUBLE_NUMBER_MAX_LENGTH
};
use crate::environments::{
    Environment,
    EnvironmentScope,
    Variable
};
use crate::syntax_tree::{
    StatementsNode,
    StatementType,
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
    DefineIfStatementNode,
};


#[derive(Debug)]
pub struct Analyzer{
    pub environments_stack: VecDeque<Environment>,
}

impl Analyzer{
    pub fn new() -> Self{
        let mut environments_stack = VecDeque::new();
        environments_stack.push_back(Environment {
            scope: EnvironmentScope::Main,
            variables: HashMap::new()
        });

        return Analyzer{
            environments_stack
        };
    }
}


fn is_variable_exists(analyzer: &Analyzer, variable_name: &String) -> bool{
    for environment in &analyzer.environments_stack{
        if environment.variables.contains_key(variable_name){
            return true;
        }
    }

    return false;
}


fn insert_variable_into_current_environmment(
    analyzer: &mut Analyzer, variable: Variable
){
    analyzer.environments_stack.back_mut().unwrap().variables.insert(
        variable.name.as_ref().unwrap().clone(),
        Some(variable));
}


fn get_variable(
    analyzer: &Analyzer, variable_name: &String
) -> Result<Variable, String>{

    for environment in &analyzer.environments_stack{
        if environment.variables.contains_key(variable_name){
            return Ok(environment.variables.get(variable_name).unwrap().as_ref().unwrap().clone());
        }
    }

    return Err(format!(
        "Engine Compiler: Analyzer Error -> Variable `{}` not found.",
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
        "Engine Compiler: Analyzer Error -> Variable `{}` not found.",
        variable_name));
}


pub fn analyze(
    analyzer: &mut Analyzer,
    syntax_tree: StatementsNode
) -> Result<(), String>{

    let mut analyzer = analyzer;
    for statement in syntax_tree.statements{
        if statement.statement_type == Some(StatementType::DefineBool){
            analyze_define_bool(
                &mut analyzer, statement.define_bool_statement.unwrap().clone())?;
        }
        else if statement.statement_type == Some(StatementType::DefineInt){
            analyze_define_int(
                &mut analyzer, statement.define_int_statement.unwrap().clone())?;
        }
        else if statement.statement_type == Some(StatementType::DefineDouble){
            analyze_define_double(
                &mut analyzer, statement.define_double_statement.unwrap().clone())?;
        }
        else if statement.statement_type == Some(StatementType::DefineChar){
            analyze_define_char(
                &mut analyzer, statement.define_char_statement.unwrap().clone())?;
        }
        else if statement.statement_type == Some(StatementType::DefineString){
            analyze_define_string(
                &mut analyzer, statement.define_string_statement.unwrap().clone())?;
        }
        else if statement.statement_type == Some(StatementType::DefineVar){
            analyze_define_var(
                &mut analyzer, statement.define_var_statement.unwrap().clone())?;
        }
        else if statement.statement_type == Some(StatementType::DefineVariable){
            analyze_define_variable(
                &mut analyzer, statement.define_variable_statement.unwrap().clone())?;
        }

        else if statement.statement_type == Some(StatementType::Print){
            analyze_define_print(
                &mut analyzer, statement.define_print_statement.unwrap().clone())?;
        }

        else if statement.statement_type == Some(StatementType::DefineIf){
            analyze_define_if_statement(
                &mut analyzer, statement.define_if_statement.unwrap().clone())?;
        }
    }

    return Ok(());
}

fn analyze_define_bool(
    analyzer: &mut Analyzer,
    statement: DefineBoolNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::Bool &&
        node_type != TokenType::True &&
        node_type != TokenType::False
    {
        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Boolean", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Bool);
    variable.value = None;

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

fn analyze_define_int(
    analyzer: &mut Analyzer,
    statement: DefineIntNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::IntNumber && node_type != TokenType::DoubleNumber{
        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Int", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Int);
    variable.value = None;

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

fn analyze_define_double(
    analyzer: &mut Analyzer,
    statement: DefineDoubleNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::IntNumber && node_type != TokenType::DoubleNumber{
        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Double", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Double);
    variable.value = None;

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

fn analyze_define_char(
    analyzer: &mut Analyzer,
    statement: DefineCharNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::Character && node_type != TokenType::StringSequence{
        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to Character", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::Char);
    variable.value = None;

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

fn analyze_define_string(
    analyzer: &mut Analyzer,
    statement: DefineStringNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;

    if node_type != TokenType::StringSequence && node_type != TokenType::Character{
        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            format!("Can't assign `{:?}` to String", node_type),
            statement.name.as_ref().unwrap().start_line,
            statement.name.as_ref().unwrap().start_pos));
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(TokenType::String);
    variable.value = None;

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}


fn analyze_define_var(
    analyzer: &mut Analyzer,
    statement: DefineVarNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    let mut node_type = analyze_operation_node(
        &analyzer, statement.left.as_ref().unwrap())?;
    if node_type == TokenType::True || node_type == TokenType::False{
        node_type = TokenType::Bool;
    }
    else if node_type == TokenType::IntNumber{
        node_type = TokenType::Int;
    }
    else if node_type == TokenType::DoubleNumber{
        node_type = TokenType::Double;
    }
    else if node_type == TokenType::Character ||
        node_type == TokenType::StringSequence
    {
        node_type = TokenType::String;
    }

    let mut variable = Variable::new();

    variable.name = Some(statement.name.as_ref().unwrap().value.clone());
    variable.variable_type = Some(node_type);
    variable.value = None;

    insert_variable_into_current_environmment(&mut analyzer, variable);

    return Ok(());
}

fn analyze_define_variable(
    analyzer: &mut Analyzer,
    statement: DefineVariableNode
) -> Result<(), String>{

    let mut analyzer = analyzer;

    if !is_variable_exists(&analyzer, &statement.name.as_ref().unwrap().value){
        if statement.operator.as_ref().unwrap().token_type != TokenType::Assign{
            return Err(format!(
                "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Undefined variable `{}`",
                    statement.name.as_ref().unwrap().value),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }

        let mut node_type = analyze_operation_node(
            &analyzer, statement.left.as_ref().unwrap())?;

        if node_type == TokenType::True || node_type == TokenType::False{
            node_type = TokenType::Bool;
        }
        else if node_type == TokenType::IntNumber{
            node_type = TokenType::Int;
        }
        else if node_type == TokenType::DoubleNumber{
            node_type = TokenType::Double;
        }
        else if node_type == TokenType::Character ||
            node_type == TokenType::StringSequence
        {
            node_type = TokenType::String;
        }

        let mut variable = Variable::new();

        variable.name = Some(
            statement.name.as_ref().unwrap().value.clone());
        variable.variable_type = Some(node_type);
        variable.value = None;

        insert_variable_into_current_environmment(&mut analyzer, variable);
    }
    else{
        let name = statement.name.as_ref().unwrap().value.clone();
        let variable = get_variable(&analyzer, &name)?;
        let variable_type = variable.variable_type.as_ref().unwrap().clone();
        let operator_type = statement.operator.as_ref().unwrap().token_type.clone();

        /* Validate Operator is Right with returned node type */
        if variable_type == TokenType::String &&
            operator_type != TokenType::Assign &&
            operator_type != TokenType::PlusEqual
        {
                return Err(format!(
                    "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                    format!(
                        "Invalid operation `{:?}` on `{:?}` which has type `{:?}`",
                        operator_type,
                        variable.name.as_ref().unwrap(),
                        variable_type),
                    statement.name.as_ref().unwrap().start_line,
                    statement.name.as_ref().unwrap().start_pos));
        }
        else if variable_type == TokenType::Char &&
            operator_type != TokenType::Assign
        {
            return Err(format!(
                "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Invalid operation `{:?}` on `{:?}` which has type `{:?}`",
                    operator_type,
                    variable.name.as_ref().unwrap(),
                    variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }
        else if variable_type == TokenType::Bool &&
            operator_type != TokenType::Assign
        {
            return Err(format!(
                "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                format!(
                    "Invalid operation `{:?}` on `{:?}` which has type `{:?}`",
                    operator_type,
                    variable.name.as_ref().unwrap(),
                    variable_type),
                statement.name.as_ref().unwrap().start_line,
                statement.name.as_ref().unwrap().start_pos));
        }

        /* Validate node type is correct with variable type */
        let node_type = analyze_operation_node(
            &analyzer, statement.left.as_ref().unwrap())?;

        if node_type == TokenType::True ||
            node_type == TokenType::False ||
            node_type == TokenType::Bool
        {
            if variable_type != TokenType::Bool{
                return Err(format!(
                    "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                    format!(
                        "Can't assign `{:?}` to `{:?}`",
                        node_type, variable_type),
                    statement.name.as_ref().unwrap().start_line,
                    statement.name.as_ref().unwrap().start_pos));
            }
        }
        else if node_type == TokenType::IntNumber ||
            node_type == TokenType::DoubleNumber
        {
            if variable_type != TokenType::Int &&
                variable_type != TokenType::Double
            {
                return Err(format!(
                    "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                    format!(
                        "Can't assign `{:?}` to `{:?}`",
                        node_type, variable_type),
                    statement.name.as_ref().unwrap().start_line,
                    statement.name.as_ref().unwrap().start_pos));
            }
        }
        else if node_type == TokenType::Character ||
            node_type == TokenType::StringSequence
        {
            if variable_type != TokenType::String &&
                variable_type != TokenType::Char
            {
                return Err(format!(
                    "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                    format!(
                        "Can't assign `{:?}` to `{:?}`",
                        node_type, variable_type),
                    statement.name.as_ref().unwrap().start_line,
                    statement.name.as_ref().unwrap().start_pos));
            }
        }

        let mut new_variable = Variable::new();
        new_variable.name = Some(name.clone());
        new_variable.is_reasigned = true;
        new_variable.variable_type = Some(variable_type);
        new_variable.value = variable.value.clone();

        insert_variable_into_current_environmment(&mut analyzer, new_variable);
    }

    return Ok(());
}

fn analyze_define_print(
    analyzer: &Analyzer,
    statement: DefinePrintNode
) -> Result<(), String>{

    analyze_operation_node(&analyzer, statement.expression.as_ref().unwrap())?;

    return Ok(());
}


fn analyze_define_if_statement(
    analyzer: &mut Analyzer,
    statement: DefineIfStatementNode,
) -> Result<(), String>{
    let mut analyzer = analyzer;

    /* Analyze If Statement */
    {
        analyzer.environments_stack.push_back(Environment {
            scope: EnvironmentScope::If,
            variables: HashMap::new()
        });

        let define_if_node = statement.define_if_node.as_ref().unwrap();

        let node_type = analyze_operation_node(
            &analyzer, define_if_node.condition.as_ref().unwrap())?;

        if node_type != TokenType::Bool && node_type != TokenType::True &&
            node_type != TokenType::False
        {
            return Err(format!(
                "Engine Compiler: Analyze Error -> {}, line {}:{}.",
                "if condition must be `Bool`",
                define_if_node.token.as_ref().unwrap().start_line,
                define_if_node.token.as_ref().unwrap().start_pos));
        }

        analyze(&mut analyzer, define_if_node.statements.clone())?;

        analyzer.environments_stack.pop_back();
    }

    /* Analyze If Else Statements */
    {
        for define_if_else_node in &statement.define_if_else_nodes{
            analyzer.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new()
            });

            let node_type = analyze_operation_node(
                &analyzer, define_if_else_node.condition.as_ref().unwrap())?;

            if node_type != TokenType::Bool && node_type != TokenType::True &&
                node_type != TokenType::False
            {
                return Err(format!(
                    "Engine Compiler: Analyze Error -> {}, line {}:{}.",
                    "if condition must be `Bool`",
                    define_if_else_node.token.as_ref().unwrap().start_line,
                    define_if_else_node.token.as_ref().unwrap().start_pos));
            }

            analyze(&mut analyzer, define_if_else_node.statements.clone())?;

            analyzer.environments_stack.pop_back();
        }
    }

    /* Analyze Else Statement */
    {
        if statement.define_else_node != None{
            analyzer.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new()
            });

            let define_else_node = statement.define_else_node.as_ref().unwrap();

            analyze(&mut analyzer, define_else_node.statements.clone())?;

            analyzer.environments_stack.pop_back();
        }
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "Invalid operation {:?} on Boolean types",
                            operator),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos));
                }
                return Ok(TokenType::Bool);
            }

            return Err(format!(
                "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                format!("Undefined operator {:?} behavior", operator),
                operation_node.value.as_ref().unwrap().start_line,
                operation_node.value.as_ref().unwrap().start_pos));
        },
        None => {
            if operation_node.value.as_ref().unwrap().token_type == TokenType::Variable{
                if !is_variable_exists(&analyzer, &operation_node.value.as_ref().unwrap().value){
                    return Err(format!(
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
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

            if operation_node.value.as_ref().unwrap().token_type == TokenType::IntNumber{
                if operation_node.value.as_ref().unwrap().value.len() > INT_NUMBER_MAX_LENGTH as usize{
                    return Err(format!(
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "The litral `{}` does not fit into int type",
                            &operation_node.value.as_ref().unwrap().value),
                        &operation_node.value.as_ref().unwrap().start_line,
                        &operation_node.value.as_ref().unwrap().start_pos));
                }
            }
            else if operation_node.value.as_ref().unwrap().token_type == TokenType::DoubleNumber{
                if operation_node.value.as_ref().unwrap().value.len() > DOUBLE_NUMBER_MAX_LENGTH as usize{
                    return Err(format!(
                        "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                        format!(
                            "The litral `{}` does not fit into double type",
                            &operation_node.value.as_ref().unwrap().value),
                        &operation_node.value.as_ref().unwrap().start_line,
                        &operation_node.value.as_ref().unwrap().start_pos));
                }
            }

            return Ok(operation_node.value.as_ref().unwrap().token_type.clone());
        }
    }
}
