use std::collections::{
    HashMap,
    VecDeque
};
use std::ops::Deref;
use std::process::Command;

use crate::environments::{
    Environment,
    EnvironmentScope,
    Variable
};
use crate::syntax_tree::{
    StatementsNode,
    StatementNode,
    StatementType,
    DefineBoolNode,
    OperationNode,
    DefineIntNode,
    DefineDoubleNode,
    DefineCharNode,
    DefineStringNode,
    OperatorType,
    DefinePrintNode,
    DefineVarNode,
    DefineVariableNode,
    DefineIfStatementNode,
    DefineForLoopStatementNode
};
use crate::tokens::TokenType;
use crate::constants::Mode;

use crate::file::File;


#[derive(Debug)]
pub struct CodeGenerator{
    file: File,
    parent_folder: String,
    new_file_path: String,
    syntax_tree: StatementsNode,
    currrent_counter: u128,
    environments_stack: VecDeque<Environment>,
}

impl CodeGenerator{
    pub fn new(
        syntax_tree: StatementsNode,
        parent_folder_path: String,
        file_name: String,
    ) -> Result<Self, std::io::Error>{

        let mut _parent_folder_path = parent_folder_path.clone();
        _parent_folder_path.push_str("/");

        let new_file_path = _parent_folder_path + &file_name + &String::from(".rs");

        let mut _file = File::create_new(
            new_file_path.clone(), Mode::Compiler)?;

        let mut environments_stack = VecDeque::new();
        environments_stack.push_back(Environment {
            scope: EnvironmentScope::Main,
            variables: HashMap::new(),
            internal_variables: HashMap::new(),
        });

        return Ok(CodeGenerator{
            file: _file,
            parent_folder: parent_folder_path.clone(),
            new_file_path,
            syntax_tree,
            currrent_counter: 0,
            environments_stack
        });
    }

    fn generate_variable_name(&mut self) -> String{
        self.currrent_counter += 1;
        return format!("_{}", self.currrent_counter);
    }

    fn insert_variable_into_environments_stack(
        &mut self, variable_name: String, variable: Variable,
    ){

        for environment in &mut self.environments_stack{
            if environment.variables.contains_key(&variable_name){
                environment.variables.insert(variable_name, Some(variable));
                return ;
            }
        }

        self.environments_stack.back_mut().as_mut().unwrap().variables.insert(
            variable_name, Some(variable));
    }

    fn insert_internal_variable_into_environment_stack(
        &mut self, variable_name: String, variable: Variable
    ){
        for environment in &mut self.environments_stack{
            if environment.internal_variables.contains_key(&variable_name){
                environment.internal_variables.insert(variable_name, variable);
                return ;
            }
        }

        self.environments_stack.back_mut().as_mut().unwrap().internal_variables.insert(
            variable_name, variable);
    }

    fn is_variable_exists(&mut self, variable_name: &String) -> bool{
        for environment in &self.environments_stack{
            if environment.variables.contains_key(variable_name){
                return true;
            }
        }

        return false;
    }

    fn get_variable_type(
        &self, variable_name: &String
    ) -> Result<TokenType, String>{

        for environment in &self.environments_stack{
            if environment.variables.contains_key(variable_name){
                return Ok(
                    environment.variables.get(variable_name)
                        .as_ref().unwrap().as_ref().unwrap()
                        .variable_type.as_ref().unwrap().clone());
            }
        }

        return Err(format!(
            "Engine Compiler: Code Generation Error -> Variable `{}` not found.",
            variable_name));
    }

    fn get_internal_variable(
        &mut self, variable_name: &String
    ) -> Option<Variable>{

        for environment in &self.environments_stack{
            if environment.internal_variables.contains_key(variable_name){
                return Some(
                    environment.internal_variables.get(variable_name)
                        .as_ref().unwrap().deref().clone());
            }
        }

        return None;
    }

    pub fn execute(&mut self) -> Result<(), String>{
        Command::new("rustc")
            .arg(self.file.file_path.clone())
            .arg("--out-dir")
            .arg(self.parent_folder.clone())
            .output()
            .expect("failed to execute process");

        return Ok(());
    }

    pub fn clean(&mut self) -> Result<(), String>{
        File::delete_file(self.new_file_path.clone(), Mode::Compiler);

        return Ok(());
    }
}


pub fn generate(
    code_generator: &mut CodeGenerator
) -> Result<(), String>{
    let mut code_generator = code_generator;

    code_generator.file.writeln(String::from("#![allow(arithmetic_overflow)]"));
    code_generator.file.writeln(String::from("use std::io;"));
    code_generator.file.writeln(String::from("use std::panic;"));
    code_generator.file.writeln(String::from("fn main(){"));
    code_generator.file.writeln(String::from("use std::io::Write;"));
    code_generator.file.writeln(String::from("panic::set_hook(Box::new(|panic_info| {"));
    code_generator.file.writeln(String::from("if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {"));
    code_generator.file.writeln(String::from("println!(\"{}\", panic_message);"));
    code_generator.file.writeln(String::from("} else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {"));
    code_generator.file.writeln(String::from("println!(\"{}\", panic_message);"));
    code_generator.file.writeln(String::from("} else {"));
    code_generator.file.writeln(String::from("println!(\"Engine Compiler -> Interperter Error {}\", panic_info);"));
    code_generator.file.writeln(String::from("}"));
    code_generator.file.writeln(String::from("}));"));

    let mut tree = code_generator.syntax_tree.clone();

    generate_statements_node(
        &mut code_generator, &mut tree)?;

    code_generator.file.writeln(String::from("}"));

    return Ok(());
}

fn generate_statements_node(
    code_generator: &mut CodeGenerator,
    statements_node: &mut StatementsNode
) -> Result<(), String>{
    let mut code_generator = code_generator;

    for statement in &mut statements_node.statements{
        generate_statement_node(&mut code_generator, statement)?;
    }

    return Ok(());
}

fn generate_statement_node(
    code_generator: &mut CodeGenerator,
    statement: &mut StatementNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    if statement.statement_type == Some(StatementType::DefineBool){
        generate_define_bool_variable(
            &mut code_generator,
            &statement.define_bool_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineInt){
        generate_define_int_variable(
            &mut code_generator,
            &statement.define_int_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineDouble){
        generate_define_double_variable(
            &mut code_generator,
            &statement.define_double_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineChar){
        generate_define_char_variable(
            &mut code_generator,
            &statement.define_char_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineString){
        generate_define_string_variable(
            &mut code_generator,
            &statement.define_string_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineVar){
        generate_define_var_variable(
            &mut code_generator,
            &statement.define_var_statement.as_ref().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::DefineVariable){
        generate_define_variable(
            &mut code_generator,
            &statement.define_variable_statement.as_ref().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::Print){
        generate_define_print_variable(
            &mut code_generator,
            &statement.define_print_statement.as_ref().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::DefineIf){
        generate_define_if_statement(
            &mut code_generator,
            &mut statement.define_if_statement.as_mut().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::DefineForLoop){
        generate_define_for_loop_statement(
            &mut code_generator,
            &mut statement.define_for_loop_statement.as_mut().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::Continue){
        generate_continue_statement(&mut code_generator)?;
    }

    return Ok(());
}

fn generate_define_bool_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineBoolNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    code_generator.file.writeln(format!(
        "let mut variable_{}: bool = {};",
        statement.name.as_ref().unwrap().value,
        result.0,
    ));

    code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Bool),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    return Ok(());
}

fn generate_define_int_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineIntNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    code_generator.file.writeln(format!(
        "let mut variable_{}: i64 = {} as i64;",
        statement.name.as_ref().unwrap().value,
        result.0
    ));

    code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Int),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    return Ok(());
}

fn generate_define_double_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineDoubleNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    code_generator.file.writeln(format!(
        "let mut variable_{}: f64 = {} as f64;",
        statement.name.as_ref().unwrap().value,
        result.0
    ));

    code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Double),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    return Ok(());
}

fn generate_define_char_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineCharNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    if result.1 == TokenType::Character{
        code_generator.file.writeln(format!(
            "let mut variable_{}: char = {};",
            statement.name.as_ref().unwrap().value,
            result.0
        ));
    }
    else{
        code_generator.file.writeln(format!(
            "let mut variable_{}: char = {}.chars().nth(0).unwrap();",
            statement.name.as_ref().unwrap().value,
            result.0
        ));
    }

    code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Char),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    return Ok(());
}

fn generate_define_string_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineStringNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    code_generator.file.writeln(format!(
        "let mut variable_{}: String = {};",
        statement.name.as_ref().unwrap().value,
        result.0
    ));

    code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::String),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    return Ok(());
}

fn generate_define_var_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineVarNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    code_generator.file.writeln(format!(
        "let mut variable_{} = {};",
        statement.name.as_ref().unwrap().value,
        result.0
    ));

    code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some({
                if result.1 == TokenType::Bool{
                    TokenType::Bool
                }
                else if result.1 == TokenType::IntNumber{
                    TokenType::Int
                }
                else if result.1 == TokenType::DoubleNumber{
                    TokenType::Double
                }
                else if result.1 == TokenType::Character{
                    TokenType::Char
                }
                else{
                    TokenType::String
                }
            }),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    return Ok(());
}

fn generate_define_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefineVariableNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.left.as_ref().unwrap())?;

    if code_generator.is_variable_exists(&statement.name.as_ref().unwrap().value){
        let variable_type = code_generator.get_variable_type(
            &statement.name.as_ref().unwrap().value)?;

        if variable_type == TokenType::Double{
            code_generator.file.writeln(format!(
                "variable_{} {} {} as f64;",
                statement.name.as_ref().unwrap().value,
                statement.operator.as_ref().unwrap().value,
                result.0
            ));
        }
        else if variable_type == TokenType::Int{
            code_generator.file.writeln(format!(
                "variable_{} {} {} as i64;",
                statement.name.as_ref().unwrap().value,
                statement.operator.as_ref().unwrap().value,
                result.0
            ));
        }
        else if variable_type == TokenType::Char{
            if result.1 == TokenType::Character{
                code_generator.file.writeln(format!(
                    "variable_{} = {};",
                    statement.name.as_ref().unwrap().value,
                    result.0
                ));
            }
            else{
                code_generator.file.writeln(format!(
                    "variable_{} = {}.chars().nth(0).unwrap();",
                    statement.name.as_ref().unwrap().value,
                    result.0
                ));
            }
        }
        else if variable_type == TokenType::String{
            if result.1 == TokenType::Character{
                code_generator.file.writeln(format!(
                    "variable_{}.push({});",
                    statement.name.as_ref().unwrap().value,
                    result.0
                ));
            }
            else{
                if statement.operator.as_ref().unwrap().token_type == TokenType::Assign{
                    code_generator.file.writeln(format!(
                        "variable_{} {} {}.clone();",
                        statement.name.as_ref().unwrap().value,
                        statement.operator.as_ref().unwrap().value,
                        result.0
                    ));
                }
                else{
                    code_generator.file.writeln(format!(
                        "variable_{} {} &{}.clone();",
                        statement.name.as_ref().unwrap().value,
                        statement.operator.as_ref().unwrap().value,
                        result.0
                    ));
                }
            }
        }
        else{
            code_generator.file.writeln(format!(
                "variable_{} = {};",
                statement.name.as_ref().unwrap().value,
                result.0
            ));
        }
    }
    else{
        code_generator.file.writeln(format!(
            "let mut variable_{} = {};",
            statement.name.as_ref().unwrap().value,
            result.0
        ));

        code_generator.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some({
                    if result.1 == TokenType::Bool{
                        TokenType::Bool
                    }
                    else if result.1 == TokenType::IntNumber{
                        TokenType::Int
                    }
                    else if result.1 == TokenType::DoubleNumber{
                        TokenType::Double
                    }
                    else if result.1 == TokenType::Character{
                        TokenType::Char
                    }
                    else{
                        TokenType::String
                    }
                }),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });
    }

    return Ok(());
}

fn generate_define_print_variable(
    code_generator: &mut CodeGenerator,
    statement: &DefinePrintNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let result = define_operation_node_variables(
        &mut code_generator,
        statement.expression.as_ref().unwrap())?;

    /* Print Variable */
    code_generator.file.writeln(format!("print!(\"{{}}\", {});", result.0));

    /* Flush Variable */
    code_generator.file.writeln(format!("if io::stdout().flush().is_err(){{"));
    code_generator.file.writeln(format!(
        "panic!(\"Engine Compiler: Compiler Error -> Error in printing to console\");"));
    code_generator.file.writeln(format!("}}"));

    return Ok(());
}

fn generate_define_if_statement(
    code_generator: &mut CodeGenerator,
    statement: &mut DefineIfStatementNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    let if_condition_result;
    let mut else_if_condition_results: Vec<(String, TokenType)> = Vec::new();

    /* Define Conditions */
    {
        /* Define If Condition */
        {
            let define_if_node = statement.define_if_node.as_mut().unwrap();

            if_condition_result = define_operation_node_variables(
                &mut code_generator,
                define_if_node.condition.as_ref().unwrap())?;
        }

        /* Define Else if Conditions */
        {
            for define_if_else_node in &mut statement.define_if_else_nodes{

                else_if_condition_results.push(define_operation_node_variables(
                    &mut code_generator,
                    define_if_else_node.condition.as_ref().unwrap())?);
            }
        }
    }

    /* Generate If Statement */
    {
        code_generator.environments_stack.push_back(Environment {
            scope: EnvironmentScope::If,
            variables: HashMap::new(),
            internal_variables: HashMap::new(),
        });

        let define_if_node = statement.define_if_node.as_mut().unwrap();

        code_generator.file.writeln(format!("if {}{{", if_condition_result.0));

        generate_statements_node(
            &mut code_generator, &mut define_if_node.statements)?;

        code_generator.file.writeln(String::from("}"));

        code_generator.environments_stack.pop_back();
    }

    /* Generate Else if Statements */
    {
        let mut index = 0;
        for define_if_else_node in &mut statement.define_if_else_nodes{
            code_generator.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new(),
                internal_variables: HashMap::new(),
            });

            let result = &else_if_condition_results[index];

            code_generator.file.writeln(format!("else if {}{{", result.0));

            generate_statements_node(
                &mut code_generator, &mut define_if_else_node.statements)?;

            code_generator.file.writeln(String::from("}"));

            code_generator.environments_stack.pop_back();

            index += 1;
        }
    }

    /* Generate Else Statement */
    {
        if statement.define_else_node != None{
            code_generator.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new(),
                internal_variables: HashMap::new(),
            });

            let define_else_node = statement.define_else_node.as_mut().unwrap();

            code_generator.file.writeln(String::from("else {"));

            generate_statements_node(
                &mut code_generator, &mut define_else_node.statements)?;

            code_generator.file.writeln(String::from("}"));

            code_generator.environments_stack.pop_back();
        }
    }

    return Ok(());
}

fn generate_define_for_loop_statement(
    code_generator: &mut CodeGenerator,
    statement: &mut DefineForLoopStatementNode
) -> Result<(), String>{

    let mut code_generator = code_generator;

    code_generator.file.writeln(String::from("{"));

    let start_loop_variable_name: String;
    let mut stop_loop_variable_name: String = String::from("");
    let mut step_loop_variable_name: String = String::from("");

    /* Generate Loop Conditions */
    {
        if statement.start != None{
            let result = define_operation_node_variables(
                &mut code_generator,
                statement.start.as_ref().unwrap())?;

            start_loop_variable_name = format!(
                "temp{}", code_generator.generate_variable_name().clone());

            code_generator.insert_internal_variable_into_environment_stack(
                String::from("start_loop_variable"),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(start_loop_variable_name.clone()),
                    value: None,
                    is_reasigned: false
                });

            code_generator.file.writeln(format!(
                "let mut {}: i64 = {} as i64;",
                start_loop_variable_name, result.0));
        }
        else{
            start_loop_variable_name = format!(
                "temp{}", code_generator.generate_variable_name().clone());

            code_generator.insert_internal_variable_into_environment_stack(
                String::from("start_loop_variable"),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(start_loop_variable_name.clone()),
                    value: None,
                    is_reasigned: false
                });

            code_generator.file.writeln(format!(
                "let mut {}: i64 = 0;", start_loop_variable_name));
        }

        if statement.stop != None{
            let result = define_operation_node_variables(
                &mut code_generator,
                statement.stop.as_ref().unwrap())?;

            stop_loop_variable_name = format!(
                "temp{}", code_generator.generate_variable_name().clone());

            code_generator.file.writeln(format!(
                "let mut {}: i64 = {} as i64;",
                stop_loop_variable_name, result.0));
        }

        if statement.step != None{
            let result = define_operation_node_variables(
                &mut code_generator,
                statement.step.as_ref().unwrap())?;

            step_loop_variable_name = format!(
                "temp{}", code_generator.generate_variable_name().clone());

            code_generator.insert_internal_variable_into_environment_stack(
                String::from("step_loop_variable"),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(step_loop_variable_name.clone()),
                    value: None,
                    is_reasigned: false
                });

            code_generator.file.writeln(format!(
                "let mut {}: i64 = {} as i64;",
                step_loop_variable_name, result.0));
        }
    }

    /* Generate Loop */
    {
        code_generator.file.writeln(String::from("loop{"));

        /* Before Execute Statements */
        {
            if statement.stop != None{
                code_generator.file.writeln(format!(
                    "if {} >= {} {{ break; }}",
                    start_loop_variable_name, stop_loop_variable_name));
            }
        }

        /* Start Execute Loop Statements */
        {
            code_generator.environments_stack.push_back(Environment {
                scope: EnvironmentScope::ForLoop,
                variables: HashMap::new(),
                internal_variables: HashMap::new(),
            });

            if statement.variable != None{
                code_generator.file.writeln(format!(
                    "let mut variable_{}: i64 = {} as i64;",
                    statement.variable.as_ref().unwrap().value,
                    start_loop_variable_name));
            }

            code_generator.insert_variable_into_environments_stack(
                statement.variable.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(statement.variable.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            generate_statements_node(
                &mut code_generator, &mut statement.statements)?;

            code_generator.environments_stack.pop_back();
        }

        /* Before Closing Loop */
        {
            if statement.step != None{
                code_generator.file.writeln(format!(
                    "{} += {};", start_loop_variable_name,
                    step_loop_variable_name));
            }
            else{
                code_generator.file.writeln(format!(
                    "{} += 1;", start_loop_variable_name));
            }
        }

        code_generator.file.writeln(String::from("}"));
    }

    code_generator.file.writeln(String::from("}"));

    return Ok(());
}


fn generate_continue_statement(
    code_generator: &mut CodeGenerator
) -> Result<(), String>{

    let start_variable = code_generator.get_internal_variable(
        &String::from("start_loop_variable"));

    let step_variable = code_generator.get_internal_variable(
        &String::from("step_loop_variable"));

    if start_variable != None{

        if step_variable == None{
            code_generator.file.writeln(format!(
                "{} += 1;", start_variable.as_ref().unwrap().name.as_ref().unwrap()));
        }
        else{
            code_generator.file.writeln(format!(
                "{} += {};", start_variable.as_ref().unwrap().name.as_ref().unwrap(),
                step_variable.as_ref().unwrap().name.as_ref().unwrap()));
        }
    }

    code_generator.file.writeln(String::from("continue;"));

    return Ok(());
}


fn define_operation_node_variables(
    code_generator: &mut CodeGenerator,
    operation_node: &OperationNode
) -> Result<(String, TokenType), String>{

    let mut code_generator = code_generator;

    match &operation_node.operator{
        Some(_operator) => {
            let mut left_variable_name = String::from("");
            let mut left_token_type = TokenType::BadToken;

            let mut right_variable_name = String::from("");
            let mut right_token_type = TokenType::BadToken;

            if operation_node.left != None{
                let result = define_operation_node_variables(
                    &mut code_generator,
                    operation_node.left.as_ref().unwrap())?;
                left_variable_name = result.0;
                left_token_type = result.1;
            }
            if operation_node.right != None{
                let result = define_operation_node_variables(
                    &mut code_generator,
                    operation_node.right.as_ref().unwrap())?;
                right_variable_name = result.0;
                right_token_type = result.1;
            }

            if operation_node.left != None && operation_node.right == None{
                return Ok((left_variable_name, left_token_type));
            }
            else if operation_node.right != None && operation_node.left == None{
                return Ok((right_variable_name, right_token_type));
            }

            /* Define Input */
            if _operator == &OperatorType::Convert{
                /* Read Input */
                let variable_name = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                code_generator.file.writeln(format!(
                    "let mut {} = String::new();",
                    variable_name));

                code_generator.file.writeln(format!(
                    "if io::stdin().read_line(&mut {}).is_err(){{",
                    variable_name));
                code_generator.file.writeln(format!(
                    "panic!(\"{}\")",
                    format!(
                        "Engine Compiler: Compiler Error -> Failed to read input, line: {}:{}",
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos)));
                code_generator.file.writeln(format!("}}"));

                code_generator.file.writeln(format!(
                    "let mut {} = String::from({}.trim_end());",
                    variable_name, variable_name));

                /* Convert Type */
                let convert_to_variable = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                code_generator.file.writeln(format!(
                    "let mut {} = {}.parse::<{}>();",
                    convert_to_variable, variable_name,
                    if operation_node.value.as_ref().unwrap().token_type == TokenType::Bool{
                        "bool"
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::Int{
                        "i64"
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::Double{
                        "f64"
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::Char{
                        "char"
                    }
                    else{
                        "String"
                    }));

                code_generator.file.writeln(format!("if {}.is_err(){{", convert_to_variable));
                code_generator.file.writeln(format!(
                    "panic!(\"{}\");",
                    format!(
                        "Engine Compiler: Convert Error -> {}, line: {}:{}",
                        format!(
                            "can't convert from `String` to `{:?}`",
                            operation_node.value.as_ref().unwrap().token_type),
                        operation_node.value.as_ref().unwrap().start_line,
                        operation_node.value.as_ref().unwrap().start_pos)));
                code_generator.file.writeln(format!("}}"));
                code_generator.file.writeln(format!(
                    "let mut {} = {}.unwrap();",
                    convert_to_variable, convert_to_variable));

                return Ok((
                    convert_to_variable,
                    operation_node.value.as_ref().unwrap().token_type.clone()));
            }

            if left_token_type == TokenType::DoubleNumber ||
                right_token_type == TokenType::DoubleNumber
            {
                let variable_name = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                let operator_type = if
                    _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                    _operator == &OperatorType::Mul || _operator == &OperatorType::Div ||
                    _operator == &OperatorType::Mod {"f64"} else {"bool"};

                if left_token_type == TokenType::IntNumber{
                    code_generator.file.writeln(format!(
                        "let mut {}: {} = {} as f64 {} {} as f64;",
                        variable_name,
                        operator_type,
                        left_variable_name,
                        operation_node.value.as_ref().unwrap().value.clone(),
                        right_variable_name,
                    ));
                }
                else if right_token_type == TokenType::IntNumber{
                    code_generator.file.writeln(format!(
                        "let mut {}: {} = {} as f64 {} {} as f64;",
                        variable_name,
                        operator_type,
                        left_variable_name,
                        operation_node.value.as_ref().unwrap().value.clone(),
                        right_variable_name,
                    ));
                }
                else{
                    code_generator.file.writeln(format!(
                        "let mut {}: {} = {} as f64 {} {} as f64;",
                        variable_name,
                        operator_type,
                        left_variable_name,
                        operation_node.value.as_ref().unwrap().value.clone(),
                        right_variable_name,
                    ));
                }

                if _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                    _operator == &OperatorType::Mul || _operator == &OperatorType::Div ||
                    _operator == &OperatorType::Mod{

                    return Ok((variable_name, TokenType::DoubleNumber));
                }
                return Ok((variable_name, TokenType::Bool));
            }
            else if left_token_type == TokenType::IntNumber || right_token_type == TokenType::IntNumber{
                let variable_name = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                if _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                    _operator == &OperatorType::Mul || _operator == &OperatorType::Mod{

                    code_generator.file.writeln(format!(
                        "let mut {}: i64 = {} as i64 {} {} as i64;",
                        variable_name,
                        left_variable_name,
                        operation_node.value.as_ref().unwrap().value.clone(),
                        right_variable_name,
                    ));
                    return Ok((variable_name, TokenType::IntNumber))
                }
                else if _operator == &OperatorType::Div{
                    code_generator.file.writeln(format!(
                        "let mut {}: f64 = {} as f64 {} {} as f64;",
                        variable_name,
                        left_variable_name,
                        operation_node.value.as_ref().unwrap().value.clone(),
                        right_variable_name,
                    ));
                    return Ok((variable_name, TokenType::DoubleNumber));
                }
                else{
                    code_generator.file.writeln(format!(
                        "let mut {}: bool = {} {} {};",
                        variable_name,
                        left_variable_name,
                        operation_node.value.as_ref().unwrap().value.clone(),
                        right_variable_name,
                    ));
                    return Ok((variable_name, TokenType::Bool));
                }
            }
            else if left_token_type == TokenType::StringSequence || right_token_type == TokenType::StringSequence{
                let variable_name = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                if left_token_type == TokenType::Character{
                    code_generator.file.writeln(format!(
                        "let mut {}: String = String::from({}) + &{}.clone();",
                        variable_name,
                        left_variable_name,
                        right_variable_name,
                    ));
                }
                else if right_token_type == TokenType::Character{
                    code_generator.file.writeln(format!(
                        "let mut {}: String = {}.clone() + &String::from({});",
                        variable_name,
                        left_variable_name,
                        right_variable_name,
                    ));
                }
                else{
                    code_generator.file.write(String::from(format!(
                        "\tlet mut {}: String = {}.clone() + &{}.clone();\n",
                        variable_name,
                        left_variable_name,
                        right_variable_name,
                    )));
                }
                return Ok((variable_name, TokenType::StringSequence));
            }
            else if left_token_type == TokenType::Character || right_token_type == TokenType::Character{
                let variable_name = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                code_generator.file.writeln(format!(
                    "let mut {}: String = String::from({}) + &String::from({});",
                    variable_name,
                    left_variable_name,
                    right_variable_name,
                ));

                return Ok((variable_name, TokenType::StringSequence));
            }
            else if left_token_type == TokenType::Bool || left_token_type == TokenType::True ||
                left_token_type == TokenType::False || right_token_type == TokenType::Bool ||
                right_token_type == TokenType::True || right_token_type == TokenType::False{

                let variable_name = format!(
                    "temp{}", code_generator.generate_variable_name().clone());

                code_generator.file.writeln(format!(
                    "let mut {}: bool = {} {} {};",
                    variable_name,
                    left_variable_name,
                    operation_node.value.as_ref().unwrap().value.clone(),
                    right_variable_name,
                ));

                return Ok((variable_name, TokenType::Bool));
            }
            return Err(format!(
                "Engine Compiler: Code Generation Error -> {}, line: {}:{}",
                format!(
                    "Unknown Operation {} for types {:?} and {:?}",
                    operation_node.value.as_ref().unwrap().value,
                    left_token_type,
                    right_token_type),
                operation_node.value.as_ref().unwrap().start_line,
                operation_node.value.as_ref().unwrap().start_pos));
        },
        None => {
            let token = operation_node.value.as_ref().unwrap();
            let variable_name = format!(
                "temp{}", code_generator.generate_variable_name().clone());

            match token.token_type{
                TokenType::IntNumber => {
                    code_generator.file.writeln(format!(
                        "let mut {}: i64 = {};", variable_name, token.value));
                },
                TokenType::DoubleNumber => {
                    code_generator.file.writeln(format!(
                        "let mut {}: f64 = {};", variable_name, token.value));
                },
                TokenType::True => {
                    code_generator.file.writeln(format!(
                        "let mut {}: bool = true;", variable_name));
                },
                TokenType::False => {
                    code_generator.file.writeln(format!(
                        "let mut {}: bool = false;", variable_name));
                },
                TokenType::Character => {
                    let mut value = String::from("");
                    for ch in token.value.chars(){
                        if String::from(ch) == "\n"{
                            value += &String::from("\\n");
                        }
                        else if String::from(ch) == "'"{
                            value += &String::from("\\'");
                        }
                        else if String::from(ch) == "\""{
                            value += &String::from("\"");
                        }
                        else if String::from(ch) == "\t"{
                            value += &String::from("\\t");
                        }
                        else if String::from(ch) == "\\"{
                            value += &String::from("\\\\");
                        }
                        else{
                            value += &String::from(ch);
                        }
                    }
                    code_generator.file.writeln(format!(
                        "let mut {}: char = '{}';",
                        variable_name, value));
                },
                TokenType::StringSequence => {
                    let mut value = String::from("");
                    for ch in token.value.chars(){
                        if String::from(ch) == "'"{
                            value += &String::from("'");
                        }
                        else if String::from(ch) == "\""{
                            value += &String::from("\\\"");
                        }
                        else if String::from(ch) == "\t"{
                            value += &String::from("\\t");
                        }
                        else if String::from(ch) == "\\"{
                            value += &String::from("\\\\");
                        }
                        else{
                            value += &String::from(ch);
                        }
                    }
                    code_generator.file.writeln(format!(
                        "let mut {}: String = String::from(\"{}\");",
                        variable_name, value));
                },
                TokenType::Variable => {
                    let variable_type = code_generator.get_variable_type(&token.value)?;

                    if variable_type == TokenType::Int{
                        return Ok((
                            format!("variable_{}", token.value.clone()),
                            TokenType::IntNumber));
                    }
                    else if variable_type == TokenType::Double{
                        return Ok((
                            format!("variable_{}", token.value.clone()),
                            TokenType::DoubleNumber));
                    }
                    else if variable_type == TokenType::String{
                        return Ok((
                            format!("variable_{}", token.value.clone()),
                            TokenType::StringSequence));
                    }
                    else if variable_type == TokenType::Char{
                        return Ok((
                            format!("variable_{}", token.value.clone()),
                            TokenType::Character));
                    }
                    return Ok((
                        format!("variable_{}", token.value.clone()),
                        TokenType::Bool));
                }
                _ => return Err(format!(
                    "Engine Compiler: File Error -> {}, line {}:{}",
                    "Can't Generate File",
                    token.start_line, token.start_pos))
            }

            return Ok((variable_name, token.token_type.clone()));
        }
    }
}
