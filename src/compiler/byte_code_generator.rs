use std::ops::Deref;
use std::collections::{
    HashMap,
    VecDeque
};

use crate::file::File;
use crate::constants::Mode;
use crate::tokens::TokenType;
use crate::constants::VERSION;
use crate::constants::BYTECODE_SPACE_STRING_LENGTH;
use crate::environments::{
    Environment,
    EnvironmentScope,
    Variable,
    Value,
    ValueType
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


#[derive(Debug)]
pub struct ByteCodeGenerator{
    file: File,
    syntax_tree: StatementsNode,
    current_counter: u128,
    current_instruction_line: u128,
    environments_stack: VecDeque<Environment>,
}

impl ByteCodeGenerator{
    pub fn new(
        syntax_tree: StatementsNode,
        parent_folder_path: String,
        file_name: String
    ) -> Result<Self, std::io::Error>{

        let mut _parent_folder_path = parent_folder_path.clone();
        _parent_folder_path.push_str("/");

        let new_file_path = _parent_folder_path + &file_name + &String::from(".en.byte");

        let mut _file = File::create_new(
            new_file_path.clone(), Mode::ByteCodeGenerator)?;

        let mut environments_stack = VecDeque::new();
        environments_stack.push_back(Environment {
            scope: EnvironmentScope::Main,
            variables: HashMap::new(),
            internal_variables: HashMap::new(),
            stop_statements_execution: false,
        });

        return Ok(ByteCodeGenerator{
            file: _file,
            syntax_tree,
            current_counter: 0,
            current_instruction_line: 0,
            environments_stack
        });
    }

    fn generate_temp_variable_name(&mut self) -> String{
        self.current_counter += 1;
        return format!(
            "temp_stack{}_variable_{}",
            self.environments_stack.len(), self.current_counter);
    }

    fn generate_variable_name(
        &mut self, variable_name: &String
    ) -> Result<String, String>{

        let mut index = 1;
        for environment in &self.environments_stack{
            if environment.variables.contains_key(variable_name){
                return Ok(format!("stack{index}_variable_{variable_name}"));
            }
            index += 1;
        }

        return Err(format!(
            "Engine Compiler: Byte Code Generation Error -> Variable `{}` not found.",
            variable_name));
    }

    fn get_current_line(&mut self) -> u128{
        self.current_instruction_line += 1;
        return self.current_instruction_line;
    }

    fn insert_variable_into_environments_stack(
        &mut self, variable_name: String, variable: Variable
    ){
        for environment in &mut self.environments_stack{
            if environment.variables.contains_key(&variable_name){
                environment.variables.insert(variable_name, Some(variable));
                return;
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
            "Engine Compiler: Byte Code Generation Error -> Variable `{}` not found.",
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

    fn is_variable_exists(&mut self, variable_name: &String) -> bool{
        for environment in &self.environments_stack{
            if environment.variables.contains_key(variable_name){
                return true;
            }
        }

        return false;
    }

    fn append_empty_lines(&mut self, string: String) -> String{
        let mut string = string;

        while string.len() < BYTECODE_SPACE_STRING_LENGTH as usize{
            string.push(' ');
        }
        return string;
    }
}


pub fn generate_byte_code(
    byte_code_generator: &mut ByteCodeGenerator
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    byte_code_generator.file.writeln(format!("0:EngineByteCode:{VERSION}"));

    let mut tree = byte_code_generator.syntax_tree.clone();

    generate_statements_node(
        &mut byte_code_generator,
        &mut tree)?;

    let current_line = byte_code_generator.get_current_line();
    byte_code_generator.file.writeln(format!("{current_line}:End:"));

    return Ok(());
}

fn generate_statements_node(
    byte_code_generator: &mut ByteCodeGenerator,
    statements_node: &mut StatementsNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    for statement in &mut statements_node.statements{
        generate_statement_node(&mut byte_code_generator, statement)?;
    }

    return Ok(());
}

fn generate_statement_node(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &mut StatementNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    if statement.statement_type == Some(StatementType::DefineBool){
        generate_define_bool_variable(
            &mut byte_code_generator,
            &statement.define_bool_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineInt){
        generate_define_int_variable(
            &mut byte_code_generator,
            &statement.define_int_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineDouble){
        generate_define_double_variable(
            &mut byte_code_generator,
            &statement.define_double_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineChar){
        generate_define_char_variable(
            &mut byte_code_generator,
            &statement.define_char_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineString){
        generate_define_string_variable(
            &mut byte_code_generator,
            &statement.define_string_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineVar){
        generate_define_var_variable(
            &mut byte_code_generator,
            &statement.define_var_statement.as_ref().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::DefineVariable){
        generate_define_variable(
            &mut byte_code_generator,
            &statement.define_variable_statement.as_ref().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::Print){
        generate_define_print_variable(
            &mut byte_code_generator,
            &statement.define_print_statement.as_ref().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::DefineIf){
        generate_if_statement(
            &mut byte_code_generator,
            &mut statement.define_if_statement.as_mut().unwrap())?;
    }

    else if statement.statement_type == Some(StatementType::DefineForLoop){
        generate_for_loop_statement(
            &mut byte_code_generator,
            &mut statement.define_for_loop_statement.as_mut().unwrap())?;
    }
    else if statement.statement_type == Some(StatementType::Continue){
        generate_continue_statement(&mut byte_code_generator)?;
    }

    return Ok(());
}

fn generate_define_bool_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineBoolNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    byte_code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Bool),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    /* Define Variable */
    let variable_name = byte_code_generator.generate_variable_name(
        &statement.name.as_ref().unwrap().value)?;
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{current_line}:Assign:bool:\"{variable_name}\":False"));
    }

    /* Convert Result To Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{}:Convert:bool:\"{}\":\"{}\"",
            current_line, variable_name, result.0));
    }

    return Ok(());
}

fn generate_define_int_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineIntNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    byte_code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Int),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    let variable_name = byte_code_generator.generate_variable_name(
        &statement.name.as_ref().unwrap().value)?;

    /* Define Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{current_line}:Assign:int:\"{variable_name}\":0"));
    }

    /* Convert Result To Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{}:Convert:int:\"{}\":\"{}\"",
            current_line, variable_name, result.0));
    }

    return Ok(());
}

fn generate_define_double_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineDoubleNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    byte_code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Double),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    let variable_name = byte_code_generator.generate_variable_name(
        &statement.name.as_ref().unwrap().value)?;

    /* Define Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{current_line}:Assign:double:\"{variable_name}\":0"));
    }

    /* Convert Result To Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{}:Convert:double:\"{}\":\"{}\"",
            current_line, variable_name, result.0));
    }

    return Ok(());
}

fn generate_define_char_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineCharNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    byte_code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::Char),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    let variable_name = byte_code_generator.generate_variable_name(
        &statement.name.as_ref().unwrap().value)?;

    /* Define Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{current_line}:Assign:char:\"{variable_name}\":''"));
    }

    /* Convert Result To Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{}:Convert:char:\"{}\":\"{}\"",
            current_line, variable_name, result.0));
    }

    return Ok(());
}

fn generate_define_string_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineStringNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    byte_code_generator.insert_variable_into_environments_stack(
        statement.name.as_ref().unwrap().value.clone(),
        Variable {
            variable_type: Some(TokenType::String),
            name: Some(statement.name.as_ref().unwrap().value.clone()),
            value: None,
            is_reasigned: false
        });

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    let variable_name = byte_code_generator.generate_variable_name(
        &statement.name.as_ref().unwrap().value)?;

    /* Define Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{current_line}:Assign:string:\"{variable_name}\":\"\""))
    }

    /* Convert Result To Variable */
    {
        let current_line = byte_code_generator.get_current_line();
        byte_code_generator.file.writeln(format!(
            "{}:Convert:string:\"{}\":\"{}\"",
            current_line, variable_name, result.0));
    }

    return Ok(());
}

fn generate_define_var_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineVarNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    if result.1 == TokenType::IntNumber{
        byte_code_generator.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Int),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let variable_name = byte_code_generator.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{current_line}:Assign:int:\"{variable_name}\":0"));
        }

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:int:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }
    }
    else if result.1 == TokenType::DoubleNumber{
        byte_code_generator.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Double),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let variable_name = byte_code_generator.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{current_line}:Assign:double:\"{variable_name}\":0"));
        }

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:double:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }
    }
    else if result.1 == TokenType::StringSequence{
        byte_code_generator.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::String),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let variable_name = byte_code_generator.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{current_line}:Assign:string:\"{variable_name}\":\"\""));
        }

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:string:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }
    }
    else if result.1 == TokenType::Character{
        byte_code_generator.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Char),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let variable_name = byte_code_generator.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{current_line}:Assign:string:\"{variable_name}\":\"\""));
        }

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:string:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }
    }
    else if result.1 == TokenType::Bool || result.1 == TokenType::True ||
        result.1 == TokenType::False
    {
        byte_code_generator.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Bool),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let variable_name = byte_code_generator.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{current_line}:Assign:bool:\"{variable_name}\":False"));
        }

        {
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:bool:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }
    }

    return Ok(());
}

fn generate_define_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefineVariableNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.left.as_ref().unwrap())?;

    if byte_code_generator.is_variable_exists(&statement.name.as_ref().unwrap().value){

        let variable_type = byte_code_generator.get_variable_type(&statement.name.as_ref().unwrap().value)?;

        let variable_name = byte_code_generator.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        if variable_type == TokenType::Bool{

            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:bool:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }
        else if variable_type == TokenType::Int{

            let mut left_variable_name = result.0;
            if result.1 == TokenType::DoubleNumber{
                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:int:\"{temp_variable_name}\":0"));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Convert:int:\"{temp_variable_name}\":\"{left_variable_name}\""));
                }

                left_variable_name = temp_variable_name;
            }

            if statement.operator.as_ref().unwrap().token_type == TokenType::PlusEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name, variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::MinusEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Minus:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name, variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::MulEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Mul:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name, variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::DivEqual{

                /* Initialize First Variable */
                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Assign:double:\"{}\":0",
                        current_line, temp_variable_name));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Convert:double:\"{}\":\"{}\"",
                        current_line, temp_variable_name, left_variable_name));
                }

                /* Initialize Second Variable */
                let second_temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Assign:double:\"{}\":0",
                        current_line, second_temp_variable_name));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Convert:double:\"{}\":\"{}\"",
                        current_line, second_temp_variable_name, variable_name));
                }

                /* Do Div Operation */
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Mod:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    second_temp_variable_name, temp_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::ModEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Mod:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    variable_name, left_variable_name));
            }
            else{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, variable_name, left_variable_name));
            }
        }
        else if variable_type == TokenType::Double{

            let mut left_variable_name = result.0;

            if result.1 == TokenType::IntNumber{
                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Convert:double:\"{}\":\"{}\"",
                        current_line, temp_variable_name, left_variable_name));
                }

                left_variable_name = temp_variable_name;
            }

            if statement.operator.as_ref().unwrap().token_type == TokenType::PlusEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::MinusEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Minus:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::MulEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Mul:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::DivEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Div:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    variable_name, left_variable_name));
            }
            else if statement.operator.as_ref().unwrap().token_type == TokenType::ModEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:Mod:\"{}\":\"{}\":\"{}\"",
                    current_line, variable_name,
                    variable_name, left_variable_name));
            }
            else{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:double:\"{}\":\"{}\"",
                    current_line, variable_name, left_variable_name));
            }
        }
        else if variable_type == TokenType::Char{

            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Convert:char:\"{}\":\"{}\"",
                current_line, variable_name, result.0));

        }
        else if variable_type == TokenType::String{
            let mut left_variable_name = result.0;

            if result.1 == TokenType::Character{
                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Convert:string:\"{temp_variable_name}\":\"{left_variable_name}\""));
                }

                left_variable_name = temp_variable_name;
            }

            if statement.operator.as_ref().unwrap().token_type == TokenType::PlusEqual{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Operation:Plus:\"{variable_name}\":\"{variable_name}\":\"{left_variable_name}\""));
            }
            else{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Convert:string:\"{variable_name}\":\"{left_variable_name}\""));
            }
        }
    }
    else{

        if result.1 == TokenType::IntNumber{
            byte_code_generator.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = byte_code_generator.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{variable_name}\":0"));
            }

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::DoubleNumber{
            byte_code_generator.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Double),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = byte_code_generator.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:double:\"{variable_name}\":0"));
            }

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:double:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::StringSequence{
            byte_code_generator.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::String),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = byte_code_generator.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:string:\"{variable_name}\":\"\""));
            }

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:string:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::Character{
            byte_code_generator.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Char),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = byte_code_generator.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:string:\"{variable_name}\":\"\""));
            }

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:string:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::Bool || result.1 == TokenType::True ||
            result.1 == TokenType::False
        {
            byte_code_generator.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Bool),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = byte_code_generator.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:bool:\"{variable_name}\":False"));
            }

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:bool:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
    }

    return Ok(());
}

fn generate_define_print_variable(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &DefinePrintNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    let result = define_operation_node_variables(
        &mut byte_code_generator,
        statement.expression.as_ref().unwrap())?;

    let current_line = byte_code_generator.get_current_line();
    byte_code_generator.file.writeln(format!(
        "{current_line}:Print:\"{}\"", result.0));

    return Ok(());
}

fn generate_if_statement(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &mut DefineIfStatementNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    let mut lines_to_be_edited: Vec<(u128, u64)> = Vec::new();
    let mut if_conditions_lines: Vec<(u128, u64, String)> = Vec::new();

    /* Generate If Statement */
    {
        let define_if_node = statement.define_if_node.as_mut().unwrap();

        let if_condition_result = define_operation_node_variables(
            &mut byte_code_generator,
            define_if_node.condition.as_ref().unwrap())?;

        byte_code_generator.environments_stack.push_back(Environment {
            scope: EnvironmentScope::If,
            variables: HashMap::new(),
            internal_variables: HashMap::new(),
            stop_statements_execution: false,
        });

        /*
            * Set String as with the spaces do not edit
            * This is a fix for rewriting line to not overwrite file content
            */
        {
            let current_line = byte_code_generator.get_current_line();
            if_conditions_lines.push((
                current_line, byte_code_generator.file.get_stream_position(),
                if_condition_result.0.clone()));

            let space_line = byte_code_generator.append_empty_lines(String::from("0"));
            byte_code_generator.file.writeln(format!(
                "{}:If:\"{}\":{}",
                current_line, if_condition_result.0, space_line));
        }

        generate_statements_node(&mut byte_code_generator, &mut define_if_node.statements)?;

        {
            let current_line = byte_code_generator.get_current_line();
            lines_to_be_edited.push((current_line, byte_code_generator.file.get_stream_position()));

            let space_line = byte_code_generator.append_empty_lines(String::from("0"));
            byte_code_generator.file.writeln(format!("{current_line}:GoTo:{space_line}"));
        }

        byte_code_generator.environments_stack.pop_back();
    }

    /* Generate Else if Statements */
    {
        let mut index = 0;
        for define_if_else_node in &mut statement.define_if_else_nodes{

            /* Re-Write Previous If Line */
            {
                let current_instruction_line = byte_code_generator.append_empty_lines(
                    format!("{}", byte_code_generator.current_instruction_line + 1));

                byte_code_generator.file.rewrite_line(
                    if_conditions_lines[index].1, format!(
                        "{}:If:\"{}\":{}",
                        if_conditions_lines[index].0,
                        if_conditions_lines[index].2,
                        current_instruction_line));
            }

            let result = define_operation_node_variables(
                &mut byte_code_generator,
                define_if_else_node.condition.as_ref().unwrap())?;

            byte_code_generator.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new(),
                internal_variables: HashMap::new(),
                stop_statements_execution: false,
            });

            /*
            * Set String as with the spaces do not edit
            * This is a fix for rewriting line to not overwrite file content
            */
            {
                let current_line = byte_code_generator.get_current_line();
                if_conditions_lines.push((
                    current_line, byte_code_generator.file.get_stream_position(),
                    result.0.clone()));

                let space_line = byte_code_generator.append_empty_lines(String::from("0"));
                byte_code_generator.file.writeln(format!(
                    "{}:If:\"{}\":{}", current_line, result.0, space_line));
            }

            generate_statements_node(&mut byte_code_generator, &mut define_if_else_node.statements)?;

            {
                let current_line = byte_code_generator.get_current_line();
                lines_to_be_edited.push((current_line, byte_code_generator.file.get_stream_position()));

                let space_line = byte_code_generator.append_empty_lines(String::from("0"));
                byte_code_generator.file.writeln(format!("{current_line}:GoTo:{space_line}"));
            }

            byte_code_generator.environments_stack.pop_back();

            index += 1;
        }
    }

    /* Generate Else Statement */
    {
        if statement.define_else_node != None{
            byte_code_generator.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new(),
                internal_variables: HashMap::new(),
                stop_statements_execution: false,
            });

            let define_else_node = statement.define_else_node.as_mut().unwrap();

            /* Re-Write Previous If Line */
            {
                let current_instruction_line = byte_code_generator.append_empty_lines(
                    format!("{}", byte_code_generator.current_instruction_line + 1));

                byte_code_generator.file.rewrite_line(
                    if_conditions_lines.last().as_ref().unwrap().1, format!(
                        "{}:If:\"{}\":{}",
                        if_conditions_lines.last().as_ref().unwrap().0,
                        if_conditions_lines.last().as_ref().unwrap().2,
                        current_instruction_line));
            }

            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!("{current_line}:Else:"));
            }

            generate_statements_node(&mut byte_code_generator, &mut define_else_node.statements)?;

            byte_code_generator.environments_stack.pop_back();
        }
        else{
            /* Re-Write Previous If Line */
            {
                let current_instruction_line = byte_code_generator.append_empty_lines(
                    format!("{}", byte_code_generator.current_instruction_line + 1));

                byte_code_generator.file.rewrite_line(
                    if_conditions_lines.last().as_ref().unwrap().1, format!(
                        "{}:If:\"{}\":{}",
                        if_conditions_lines.last().as_ref().unwrap().0,
                        if_conditions_lines.last().as_ref().unwrap().2,
                        current_instruction_line));
            }
        }
    }

    /* Re-Write GoTo Lines */
    {
        let current_instruction_line = byte_code_generator.append_empty_lines(
            format!("{}", byte_code_generator.current_instruction_line + 1));

        for (line, stream_position) in lines_to_be_edited{
            byte_code_generator.file.rewrite_line(
                stream_position, format!("{line}:GoTo:{current_instruction_line}"));
        }
    }

    return Ok(());
}

fn generate_for_loop_statement(
    byte_code_generator: &mut ByteCodeGenerator,
    statement: &mut DefineForLoopStatementNode
) -> Result<(), String>{

    let mut byte_code_generator = byte_code_generator;

    let start_loop_variable_name: String;
    let mut stop_loop_variable_name: String = String::from("");
    let mut step_loop_variable_name: String = String::from("");

    /* Generate loop conditions instructions */
    {
        if statement.start != None{
            let result = define_operation_node_variables(
                &mut byte_code_generator,
                statement.start.as_ref().unwrap())?;

            start_loop_variable_name = byte_code_generator.generate_temp_variable_name();

            /* Define Start Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{start_loop_variable_name}\":0"));
            }

            /* Convert Result To Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, start_loop_variable_name, result.0));
            }
        }
        else{
            start_loop_variable_name = byte_code_generator.generate_temp_variable_name();

            /* Define Start Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{start_loop_variable_name}\":0"));
            }
        }

        if statement.stop != None{
            let result = define_operation_node_variables(
                &mut byte_code_generator,
                statement.stop.as_ref().unwrap())?;

            stop_loop_variable_name = byte_code_generator.generate_temp_variable_name();

            /* Define Stop Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{stop_loop_variable_name}\":0"));
            }

            /* Convert Result To Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, stop_loop_variable_name, result.0));
            }
        }

        if statement.step != None{
            let result = define_operation_node_variables(
                &mut byte_code_generator,
                statement.step.as_ref().unwrap())?;

            step_loop_variable_name = byte_code_generator.generate_temp_variable_name();

            /* Define Stop Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{step_loop_variable_name}\":0"));
            }

            /* Convert Result To Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, step_loop_variable_name, result.0));
            }
        }
    }

    /* Generate loop */
    {
        let mut start_loop_instruction_line: u128 = byte_code_generator.current_instruction_line + 1;
        let mut break_loop_instruction_line: (u128, u64, String) = (
            0, 0, String::from(""));

        /* Before Excecute Loop */
        {
            if statement.stop != None{

                let temp_compare_variable = byte_code_generator.generate_temp_variable_name();

                /* Define Stop Variable */
                {
                    start_loop_instruction_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{start_loop_instruction_line}:Assign:bool:\"{temp_compare_variable}\":False"));
                }

                /* Compare Condition */
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                        current_line, TokenType::LessThan,
                        temp_compare_variable,
                        start_loop_variable_name, stop_loop_variable_name));
                }

                /* Break Condition */
                {
                    let space_line = byte_code_generator.append_empty_lines(String::from("0"));
                    let current_stream = byte_code_generator.file.get_stream_position();

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:If:\"{}\":{}",
                        current_line,
                        temp_compare_variable, space_line));

                    break_loop_instruction_line = (
                        current_line, current_stream,
                        temp_compare_variable);
                }
            }
        }

        /* Start Execute Loop Statements */
        {
            byte_code_generator.environments_stack.push_back(Environment {
                scope: EnvironmentScope::ForLoop,
                variables: HashMap::new(),
                internal_variables: HashMap::new(),
                stop_statements_execution: false,
            });

            byte_code_generator.insert_internal_variable_into_environment_stack(
                String::from("start_loop_variable"),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(start_loop_variable_name.clone()),
                    value: None,
                    is_reasigned: false
                });

            if statement.step != None{
                byte_code_generator.insert_internal_variable_into_environment_stack(
                    String::from("step_loop_variable"),
                    Variable {
                        variable_type: Some(TokenType::Int),
                        name: Some(step_loop_variable_name.clone()),
                        value: None,
                        is_reasigned: false
                    });
            }

            byte_code_generator.insert_internal_variable_into_environment_stack(
                String::from("start_loop_instruction_line"), Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(String::from("start_loop_instruction_line")),
                    value: Some(Value{
                        value_type: Some(ValueType::String),
                        boolean: None,
                        character: None,
                        double: None,
                        int: None,
                        string: Some(format!("{start_loop_instruction_line}")),
                        string_value: None
                    }),
                    is_reasigned: false });

            byte_code_generator.insert_variable_into_environments_stack(
                statement.variable.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(statement.variable.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            /* Define Variable */
            let variable_name = byte_code_generator.generate_variable_name(
                &statement.variable.as_ref().unwrap().value)?;

            /* Define Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{variable_name}\":0"));
            }

            /* Assign Value To Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, variable_name, start_loop_variable_name));
            }

            generate_statements_node(
                &mut byte_code_generator, &mut statement.statements)?;

            byte_code_generator.environments_stack.pop_back();
        }

        /* Before Closing Loop */
        {
            if statement.step != None{

                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                    current_line, TokenType::Plus,
                    start_loop_variable_name,
                    start_loop_variable_name, step_loop_variable_name));
            }
            else{
                let temp_step_variable = byte_code_generator.generate_temp_variable_name();

                /* Define Temp Variable */
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:int:\"{temp_step_variable}\":1"));
                }

                /* Add Operation */
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                    current_line, TokenType::Plus,
                    start_loop_variable_name,
                    start_loop_variable_name, temp_step_variable));
            }

            /* Go To Start Loop */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:GoTo:{start_loop_instruction_line}"));
            }

            /* Re-Write If Condition */
            if statement.stop != None{
                let current_instruction_line = byte_code_generator.append_empty_lines(
                    format!("{}", byte_code_generator.current_instruction_line + 1));

                byte_code_generator.file.rewrite_line(
                    break_loop_instruction_line.1,
                    format!(
                        "{}:If:\"{}\":{}",
                        break_loop_instruction_line.0,
                        break_loop_instruction_line.2,
                        current_instruction_line));
            }
        }
    }

    return Ok(());
}


fn generate_continue_statement(
    byte_code_generator: &mut ByteCodeGenerator,
) -> Result<(), String>{

    let start_variable = byte_code_generator.get_internal_variable(
        &String::from("start_loop_variable"));

    let step_variable = byte_code_generator.get_internal_variable(
        &String::from("step_loop_variable"));

    if start_variable != None{

        if step_variable != None{

            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                current_line, TokenType::Plus,
                start_variable.as_ref().unwrap().name.as_ref().unwrap(),
                start_variable.as_ref().unwrap().name.as_ref().unwrap(),
                step_variable.as_ref().unwrap().name.as_ref().unwrap()));

        }
        else{

            let temp_step_variable = byte_code_generator.generate_temp_variable_name();

            /* Define Temp Variable */
            {
                let current_line = byte_code_generator.get_current_line();
                byte_code_generator.file.writeln(format!(
                    "{current_line}:Assign:int:\"{temp_step_variable}\":1"));
            }

            /* Add Operation */
            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                current_line, TokenType::Plus,
                start_variable.as_ref().unwrap().name.as_ref().unwrap(),
                start_variable.as_ref().unwrap().name.as_ref().unwrap(),
                temp_step_variable));
        }

        /* Go To The Start Of The Loop */
        {
            let start_loop_instruction_line = byte_code_generator.get_internal_variable(
                &String::from("start_loop_instruction_line"));

            let current_line = byte_code_generator.get_current_line();
            byte_code_generator.file.writeln(format!(
                "{current_line}:GoTo:{}",
                start_loop_instruction_line.as_ref().unwrap()
                    .value.as_ref().unwrap().string.as_ref().unwrap()));

        }
    }

    return Ok(());
}


fn define_operation_node_variables(
    byte_code_generator: &mut ByteCodeGenerator,
    operation_node: &OperationNode
) -> Result<(String, TokenType), String>{

    let mut byte_code_generator = byte_code_generator;

    match &operation_node.operator{
        Some(_operator) => {
            let mut left_variable_name = String::from("");
            let mut left_token_type = TokenType::BadToken;

            let mut right_variable_name = String::from("");
            let mut right_token_type = TokenType::BadToken;

            if operation_node.left != None{
                let result = define_operation_node_variables(
                    &mut byte_code_generator,
                    operation_node.left.as_ref().unwrap())?;
                left_variable_name = result.0;
                left_token_type = result.1;
            }
            if operation_node.right != None{
                let result = define_operation_node_variables(
                    &mut byte_code_generator,
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

            /* Convert Operation */
            if _operator == &OperatorType::Convert{

                /* Initialize String Variable */
                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Input:\"{temp_variable_name}\""));
                }

                let converted_to_variable_name = byte_code_generator.generate_temp_variable_name();

                if operation_node.value.as_ref().unwrap().token_type == TokenType::Bool{
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:bool:\"{converted_to_variable_name}\":False"));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:bool:\"{}\":\"{}\"",
                            current_line, converted_to_variable_name, temp_variable_name));
                    }

                    return Ok((converted_to_variable_name, TokenType::Bool));
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Int{
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:int:\"{converted_to_variable_name}\":0"));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:int:\"{}\":\"{}\"",
                            current_line, converted_to_variable_name, temp_variable_name));
                    }

                    return Ok((converted_to_variable_name, TokenType::IntNumber));
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Double{
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{converted_to_variable_name}\":0"));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, converted_to_variable_name, temp_variable_name));
                    }

                    return Ok((converted_to_variable_name, TokenType::DoubleNumber));
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::Char{
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:char:\"{converted_to_variable_name}\":''"));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:char:\"{}\":\"{}\"",
                            current_line, converted_to_variable_name, temp_variable_name));
                    }

                    return Ok((converted_to_variable_name, TokenType::Character));
                }
                else if operation_node.value.as_ref().unwrap().token_type == TokenType::String{
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:string:\"{converted_to_variable_name}\":\"\""));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:string:\"{}\":\"{}\"",
                            current_line, converted_to_variable_name, temp_variable_name));
                    }

                    return Ok((converted_to_variable_name, TokenType::StringSequence));
                }
            }

            if left_token_type == TokenType::DoubleNumber ||
                right_token_type == TokenType::DoubleNumber
            {
                let mut first_variable_name = left_variable_name;
                let mut second_variable_name = right_variable_name;

                if left_token_type == TokenType::IntNumber{

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    /* Initialize Double Variable */
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                    }

                    /* Convert Variable to double */
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, temp_variable_name, first_variable_name));
                    }

                    first_variable_name = temp_variable_name;
                }
                else if right_token_type == TokenType::IntNumber{

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    /* Initialize Double Variable */
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                    }

                    /* Convert Variable to double */
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, temp_variable_name, second_variable_name));
                    }

                    second_variable_name = temp_variable_name;
                }

                if _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                    _operator == &OperatorType::Mul || _operator == &OperatorType::Div ||
                    _operator == &OperatorType::Mod
                {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                            current_line, _operator, temp_variable_name,
                            first_variable_name, second_variable_name));
                    }

                    return Ok((temp_variable_name, TokenType::DoubleNumber));
                }

                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:bool:\"{temp_variable_name}\":False"));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                        current_line, _operator, temp_variable_name,
                        first_variable_name, second_variable_name));
                }

                return Ok((temp_variable_name, TokenType::Bool));
            }

            else if left_token_type == TokenType::IntNumber ||
                right_token_type == TokenType::IntNumber
            {

                if _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                    _operator == &OperatorType::Mul || _operator == &OperatorType::Mod
                {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:int:\"{temp_variable_name}\":0"));
                    }

                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                            current_line, _operator, temp_variable_name,
                            left_variable_name, right_variable_name));
                    }

                    return Ok((temp_variable_name, TokenType::IntNumber))
                }
                else if _operator == &OperatorType::Div{

                    /* Initialize First Variable */
                    let first_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{first_variable_name}\":0"));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, first_variable_name, left_variable_name));
                    }

                    /* Initialize Second Variable */
                    let second_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{second_variable_name}\":0"));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, second_variable_name, right_variable_name));

                    }

                    /* Initialize Result Variable */
                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:Div:\"{}\":\"{}\":\"{}\"",
                            current_line, temp_variable_name,
                            first_variable_name, second_variable_name));
                    }

                    return Ok((temp_variable_name, TokenType::DoubleNumber));
                }
                else{

                    let variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:bool:\"{variable_name}\":False"));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                            current_line, _operator, variable_name,
                            left_variable_name, right_variable_name));
                    }

                    return Ok((variable_name, TokenType::Bool));
                }
            }
            else if left_token_type == TokenType::StringSequence ||
                right_token_type == TokenType::StringSequence
            {
                if left_token_type == TokenType::Character{

                    /* Initialize First Variable */
                    let first_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:string:\"{first_variable_name}\":\"\""));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:string:\"{}\":\"{}\"",
                            current_line, first_variable_name, left_variable_name));
                    }

                    /* Concat String */
                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                            current_line, temp_variable_name,
                            first_variable_name, right_variable_name));
                    }

                    return Ok((temp_variable_name, TokenType::StringSequence));
                }
                else if right_token_type == TokenType::Character{

                    /* Initialize Second Variable */
                    let second_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:string:\"{second_variable_name}\":\"\""));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Convert:string:\"{}\":\"{}\"",
                            current_line, second_variable_name, right_variable_name));
                    }

                    /* Concat String */
                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                            current_line, temp_variable_name,
                            left_variable_name, second_variable_name));
                    }

                    return Ok((temp_variable_name, TokenType::StringSequence));
                }
                else{

                    /* Concat String */
                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                    }
                    {
                        let current_line = byte_code_generator.get_current_line();
                        byte_code_generator.file.writeln(format!(
                            "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                            current_line, temp_variable_name,
                            left_variable_name, right_variable_name));
                    }

                    return Ok((temp_variable_name, TokenType::StringSequence));
                }
            }
            else if left_token_type == TokenType::Character ||
                right_token_type == TokenType::Character
            {
                /* Initialize First Variable */
                let first_variable_name = byte_code_generator.generate_temp_variable_name();
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:string:\"{first_variable_name}\":\"\""));
                }
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Convert:string:\"{}\":\"{}\"",
                        current_line, first_variable_name, left_variable_name));
                }

                /* Initialize Second Variable */
                let second_variable_name = byte_code_generator.generate_temp_variable_name();
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:string:\"{second_variable_name}\":\"\""));
                }
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Convert:string:\"{}\":\"{}\"",
                        current_line, second_variable_name, right_variable_name));
                }

                /* Concat String */
                let temp_variable_name = byte_code_generator.generate_temp_variable_name();
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                }
                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                        current_line, temp_variable_name,
                        first_variable_name, second_variable_name));
                }

                return Ok((temp_variable_name, TokenType::StringSequence));
            }
            else if left_token_type == TokenType::Bool || left_token_type == TokenType::True ||
                left_token_type == TokenType::False || right_token_type == TokenType::Bool ||
                right_token_type == TokenType::True || right_token_type == TokenType::False
            {

                let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:bool:\"{temp_variable_name}\":False"));
                }

                {
                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                        current_line, _operator, temp_variable_name,
                        left_variable_name, right_variable_name));
                }

                return Ok((temp_variable_name, TokenType::Bool));
            }

            return Err(format!(
                "Engine Compiler: ByteCode Generation Error -> {}, line: {}:{}",
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

            match token.token_type{
                TokenType::True => {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:bool:\"{temp_variable_name}\":True"));

                    return Ok((temp_variable_name, token.token_type.clone()));
                },
                TokenType::False => {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{current_line}:Assign:bool:\"{temp_variable_name}\":False"));

                    return Ok((temp_variable_name, token.token_type.clone()));
                },
                TokenType::IntNumber => {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Assign:int:\"{}\":{}",
                        current_line, temp_variable_name, token.value));

                    return Ok((temp_variable_name, token.token_type.clone()));
                },
                TokenType::DoubleNumber => {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Assign:double:\"{}\":{}",
                        current_line, temp_variable_name, token.value));

                    return Ok((temp_variable_name, token.token_type.clone()));
                },
                TokenType::Character => {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

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

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Assign:char:\"{}\":'{}'",
                        current_line, temp_variable_name, value));

                    return Ok((temp_variable_name, token.token_type.clone()));
                },
                TokenType::StringSequence => {

                    let temp_variable_name = byte_code_generator.generate_temp_variable_name();

                    let mut value = String::from("");
                    for ch in token.value.chars(){
                        if String::from(ch) == "\n"{
                            value += &String::from("\\n");
                        }
                        else if String::from(ch) == "'"{
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

                    let current_line = byte_code_generator.get_current_line();
                    byte_code_generator.file.writeln(format!(
                        "{}:Assign:string:\"{}\":\"{}\"",
                        current_line, temp_variable_name, value));

                    return Ok((temp_variable_name, token.token_type.clone()));
                },
                TokenType::Variable => {

                    let variable_type = byte_code_generator.get_variable_type(&token.value)?;

                    let variable_name = byte_code_generator.generate_variable_name(&token.value)?;

                    if variable_type == TokenType::Int{
                        return Ok((variable_name, TokenType::IntNumber));
                    }
                    else if variable_type == TokenType::Double{
                        return Ok((variable_name, TokenType::DoubleNumber));
                    }
                    else if variable_type == TokenType::String{
                        return Ok((variable_name, TokenType::StringSequence));
                    }
                    else if variable_type == TokenType::Char{
                        return Ok((variable_name, TokenType::Character));
                    }
                    return Ok((variable_name, TokenType::Bool));
                }
                _ => return Err(format!(
                    "Engine Compiler: Byte Code Generation Error -> {}, line {}:{}",
                    "Can't Generate File",
                    token.start_line, token.start_pos))
            }
        }
    }
}
