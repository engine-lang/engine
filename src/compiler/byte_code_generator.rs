use std::collections::{HashMap, VecDeque};

use crate::compiler::file::File;
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
    DefineIfStatementNode
};
use crate::constants::Mode;
use crate::tokens::TokenType;
use crate::constants::VERSION;
use crate::constants::compiler::BYTECODE_SPACE_STRING_LENGTH;
use crate::environments::{
    Environment,
    EnvironmentScope,
    Variable
};


#[derive(Debug)]
pub struct ByteCodeGenerator{
    file: File,
    syntax_tree: StatementsNode,
    currrent_counter: u128,
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
            variables: HashMap::new()
        });

        return Ok(ByteCodeGenerator{
            file: _file,
            syntax_tree,
            currrent_counter: 0,
            current_instruction_line: 0,
            environments_stack
        });
    }

    fn generate_temp_variable_name(&mut self) -> String{
        self.currrent_counter += 1;
        return format!(
            "temp_stack{}_variable_{}",
            self.environments_stack.len(), self.currrent_counter);
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

impl ByteCodeGenerator{
    pub fn generate(&mut self) -> Result<(), String>{
        self.file.writeln(format!("0:EngineByteCode:{VERSION}"));

        self.generate_statements_node(&mut self.syntax_tree.clone())?;

        let current_line = self.get_current_line();
        self.file.writeln(format!("{current_line}:End:"));

        return Ok(());
    }

    fn generate_statements_node(
        &mut self, statements_node: &mut StatementsNode
    ) -> Result<(), String>{

        for statement in &mut statements_node.statements{
            self.generate_statement_node(statement)?;
        }

        return Ok(());
    }

    fn generate_statement_node(
        &mut self, statement: &mut StatementNode
    ) -> Result<(), String>{

        if statement.statement_type == Some(StatementType::DefineBool){
            self.generate_define_bool_variable(
                &statement.define_bool_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineInt){
            self.generate_define_int_variable(
                &statement.define_int_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineDouble){
            self.generate_define_double_variable(
                &statement.define_double_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineChar){
            self.generate_define_char_variable(
                &statement.define_char_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineString){
            self.generate_define_string_variable(
                &statement.define_string_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineVar){
            self.generate_define_var_variable(
                &statement.define_var_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineVariable){
            self.generate_define_variable(
                &statement.define_variable_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::Print){
            self.generate_define_print_variable(
                &statement.define_print_statement.as_ref().unwrap())?;
        }
        else if statement.statement_type == Some(StatementType::DefineIf){
            self.generate_if_statement(
                &mut statement.define_if_statement.as_mut().unwrap())?;
        }

        return Ok(());
    }

    fn generate_define_bool_variable(
        &mut self, statement: &DefineBoolNode
    ) -> Result<(), String>{

        self.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Bool),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        /* Define Variable */
        let variable_name = self.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{current_line}:Assign:bool:\"{variable_name}\":False"));
        }

        /* Convert Result To Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{}:Convert:bool:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }

        return Ok(());
    }

    fn generate_define_int_variable(
        &mut self, statement: &DefineIntNode
    ) -> Result<(), String>{

        self.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Int),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        let variable_name = self.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        /* Define Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{current_line}:Assign:int:\"{variable_name}\":0"));
        }

        /* Convert Result To Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{}:Convert:int:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }

        return Ok(());
    }

    fn generate_define_double_variable(
        &mut self, statement: &DefineDoubleNode
    ) -> Result<(), String>{

        self.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Double),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        let variable_name = self.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        /* Define Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{current_line}:Assign:double:\"{variable_name}\":0"));
        }

        /* Convert Result To Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{}:Convert:double:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }

        return Ok(());
    }

    fn generate_define_char_variable(
        &mut self, statement: &DefineCharNode
    ) -> Result<(), String>{

        self.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::Char),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        let variable_name = self.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        /* Define Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{current_line}:Assign:char:\"{variable_name}\":''"));
        }

        /* Convert Result To Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{}:Convert:char:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }

        return Ok(());
    }

    fn generate_define_string_variable(
        &mut self, statement: &DefineStringNode
    ) -> Result<(), String>{

        self.insert_variable_into_environments_stack(
            statement.name.as_ref().unwrap().value.clone(),
            Variable {
                variable_type: Some(TokenType::String),
                name: Some(statement.name.as_ref().unwrap().value.clone()),
                value: None,
                is_reasigned: false
            });

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        let variable_name = self.generate_variable_name(
            &statement.name.as_ref().unwrap().value)?;

        /* Define Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{current_line}:Assign:string:\"{variable_name}\":\"\""))
        }

        /* Convert Result To Variable */
        {
            let current_line = self.get_current_line();
            self.file.writeln(format!(
                "{}:Convert:string:\"{}\":\"{}\"",
                current_line, variable_name, result.0));
        }

        return Ok(());
    }

    fn generate_define_var_variable(
        &mut self, statement: &DefineVarNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        if result.1 == TokenType::IntNumber{
            self.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Int),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = self.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{current_line}:Assign:int:\"{variable_name}\":0"));
            }

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:int:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::DoubleNumber{
            self.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Double),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = self.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{current_line}:Assign:double:\"{variable_name}\":0"));
            }

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:double:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::StringSequence{
            self.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::String),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = self.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{current_line}:Assign:string:\"{variable_name}\":\"\""));
            }

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:string:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::Character{
            self.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Char),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = self.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{current_line}:Assign:string:\"{variable_name}\":\"\""));
            }

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:string:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }
        else if result.1 == TokenType::Bool || result.1 == TokenType::True ||
            result.1 == TokenType::False
        {
            self.insert_variable_into_environments_stack(
                statement.name.as_ref().unwrap().value.clone(),
                Variable {
                    variable_type: Some(TokenType::Bool),
                    name: Some(statement.name.as_ref().unwrap().value.clone()),
                    value: None,
                    is_reasigned: false
                });

            let variable_name = self.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{current_line}:Assign:bool:\"{variable_name}\":False"));
            }

            {
                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:bool:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
        }

        return Ok(());
    }

    fn generate_define_variable(
        &mut self, statement: &DefineVariableNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        if self.is_variable_exists(&statement.name.as_ref().unwrap().value){

            let variable_type = self.get_variable_type(&statement.name.as_ref().unwrap().value)?;

            let variable_name = self.generate_variable_name(
                &statement.name.as_ref().unwrap().value)?;

            if variable_type == TokenType::Bool{

                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:bool:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));
            }
            else if variable_type == TokenType::Int{

                let mut left_variable_name = result.0;
                if result.1 == TokenType::DoubleNumber{
                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:int:\"{temp_variable_name}\":0"));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Convert:int:\"{temp_variable_name}\":\"{left_variable_name}\""));
                    }

                    left_variable_name = temp_variable_name;
                }

                if statement.operator.as_ref().unwrap().token_type == TokenType::PlusEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name, variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::MinusEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Minus:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name, variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::MulEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Mul:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name, variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::DivEqual{

                    /* Initialize First Variable */
                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Assign:double:\"{}\":0",
                            current_line, temp_variable_name));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, temp_variable_name, left_variable_name));
                    }

                    /* Initialize Second Variable */
                    let second_temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Assign:double:\"{}\":0",
                            current_line, second_temp_variable_name));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, second_temp_variable_name, variable_name));
                    }

                    /* Do Div Operation */
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Mod:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        second_temp_variable_name, temp_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::ModEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Mod:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        variable_name, left_variable_name));
                }
                else{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:int:\"{}\":\"{}\"",
                        current_line, variable_name, left_variable_name));
                }
            }
            else if variable_type == TokenType::Double{

                let mut left_variable_name = result.0;

                if result.1 == TokenType::IntNumber{
                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Convert:double:\"{}\":\"{}\"",
                            current_line, temp_variable_name, left_variable_name));
                    }

                    left_variable_name = temp_variable_name;
                }

                if statement.operator.as_ref().unwrap().token_type == TokenType::PlusEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::MinusEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Minus:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::MulEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Mul:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::DivEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Div:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        variable_name, left_variable_name));
                }
                else if statement.operator.as_ref().unwrap().token_type == TokenType::ModEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Operation:Mod:\"{}\":\"{}\":\"{}\"",
                        current_line, variable_name,
                        variable_name, left_variable_name));
                }
                else{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:double:\"{}\":\"{}\"",
                        current_line, variable_name, left_variable_name));
                }
            }
            else if variable_type == TokenType::Char{

                let current_line = self.get_current_line();
                self.file.writeln(format!(
                    "{}:Convert:char:\"{}\":\"{}\"",
                    current_line, variable_name, result.0));

            }
            else if variable_type == TokenType::String{
                let mut left_variable_name = result.0;

                if result.1 == TokenType::Character{
                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Convert:string:\"{temp_variable_name}\":\"{left_variable_name}\""));
                    }

                    left_variable_name = temp_variable_name;
                }

                if statement.operator.as_ref().unwrap().token_type == TokenType::PlusEqual{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Operation:Plus:\"{variable_name}\":\"{variable_name}\":\"{left_variable_name}\""));
                }
                else{

                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Convert:string:\"{variable_name}\":\"{left_variable_name}\""));
                }
            }
        }
        else{

            if result.1 == TokenType::IntNumber{
                self.insert_variable_into_environments_stack(
                    statement.name.as_ref().unwrap().value.clone(),
                    Variable {
                        variable_type: Some(TokenType::Int),
                        name: Some(statement.name.as_ref().unwrap().value.clone()),
                        value: None,
                        is_reasigned: false
                    });

                let variable_name = self.generate_variable_name(
                    &statement.name.as_ref().unwrap().value)?;

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Assign:int:\"{variable_name}\":0"));
                }

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:int:\"{}\":\"{}\"",
                        current_line, variable_name, result.0));
                }
            }
            else if result.1 == TokenType::DoubleNumber{
                self.insert_variable_into_environments_stack(
                    statement.name.as_ref().unwrap().value.clone(),
                    Variable {
                        variable_type: Some(TokenType::Double),
                        name: Some(statement.name.as_ref().unwrap().value.clone()),
                        value: None,
                        is_reasigned: false
                    });

                let variable_name = self.generate_variable_name(
                    &statement.name.as_ref().unwrap().value)?;

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Assign:double:\"{variable_name}\":0"));
                }

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:double:\"{}\":\"{}\"",
                        current_line, variable_name, result.0));
                }
            }
            else if result.1 == TokenType::StringSequence{
                self.insert_variable_into_environments_stack(
                    statement.name.as_ref().unwrap().value.clone(),
                    Variable {
                        variable_type: Some(TokenType::String),
                        name: Some(statement.name.as_ref().unwrap().value.clone()),
                        value: None,
                        is_reasigned: false
                    });

                let variable_name = self.generate_variable_name(
                    &statement.name.as_ref().unwrap().value)?;

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Assign:string:\"{variable_name}\":\"\""));
                }

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:string:\"{}\":\"{}\"",
                        current_line, variable_name, result.0));
                }
            }
            else if result.1 == TokenType::Character{
                self.insert_variable_into_environments_stack(
                    statement.name.as_ref().unwrap().value.clone(),
                    Variable {
                        variable_type: Some(TokenType::Char),
                        name: Some(statement.name.as_ref().unwrap().value.clone()),
                        value: None,
                        is_reasigned: false
                    });

                let variable_name = self.generate_variable_name(
                    &statement.name.as_ref().unwrap().value)?;

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Assign:string:\"{variable_name}\":\"\""));
                }

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:string:\"{}\":\"{}\"",
                        current_line, variable_name, result.0));
                }
            }
            else if result.1 == TokenType::Bool || result.1 == TokenType::True ||
                result.1 == TokenType::False
            {
                self.insert_variable_into_environments_stack(
                    statement.name.as_ref().unwrap().value.clone(),
                    Variable {
                        variable_type: Some(TokenType::Bool),
                        name: Some(statement.name.as_ref().unwrap().value.clone()),
                        value: None,
                        is_reasigned: false
                    });

                let variable_name = self.generate_variable_name(
                    &statement.name.as_ref().unwrap().value)?;

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{current_line}:Assign:bool:\"{variable_name}\":False"));
                }

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!(
                        "{}:Convert:bool:\"{}\":\"{}\"",
                        current_line, variable_name, result.0));
                }
            }
        }

        return Ok(());
    }

    fn generate_define_print_variable(
        &mut self, statement: &DefinePrintNode
    ) -> Result<(), String>{

        let variable_name = self.generate_variable_name(
            &statement.variable.as_ref().unwrap().value)?;

        let current_line = self.get_current_line();
        self.file.writeln(format!(
            "{current_line}:Print:\"{variable_name}\""));

        return Ok(());
    }

    fn generate_if_statement(
        &mut self, statement: &mut DefineIfStatementNode
    ) -> Result<(), String>{

        /*
            If Condition:
            If::Go To else if condition
                Go To Line:

            else if condition
            If::Go To else if condition
                Go To Line:

            else if condition
            If::Go To else
                Go To Line:

            else:

            Line
         */

        let mut lines_to_be_edited: Vec<(u128, u64)> = Vec::new();
        let mut if_conditions_lines: Vec<(u128, u64, String)> = Vec::new();

        /* Generate If Statement */
        {
            let define_if_node = statement.define_if_node.as_mut().unwrap();

            let if_condition_result = self.define_operation_node_variables(
                define_if_node.condition.as_ref().unwrap())?;

            self.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new()
            });

            /*
             * Set String as with the spaces do not edit
             * This is a fix for rewriting line to not overwrite file content
             */
            {
                let current_line = self.get_current_line();
                if_conditions_lines.push((
                    current_line, self.file.get_stream_position(),
                    if_condition_result.0.clone()));

                let space_line = self.append_empty_lines(String::from("0"));
                self.file.writeln(format!(
                    "{}:If:\"{}\":{}",
                    current_line, if_condition_result.0, space_line));
            }

            self.generate_statements_node(&mut define_if_node.statements)?;

            {
                let current_line = self.get_current_line();
                lines_to_be_edited.push((current_line, self.file.get_stream_position()));

                let space_line = self.append_empty_lines(String::from("0"));
                self.file.writeln(format!("{current_line}:GoTo:{space_line}"));
            }

            self.environments_stack.pop_back();
        }

        /* Generate Else if Statements */
        {
            let mut index = 0;
            for define_if_else_node in &mut statement.define_if_else_nodes{

                /* Re-Write Previous If Line */
                {
                    let current_instruction_line = self.append_empty_lines(
                        format!("{}", self.current_instruction_line + 1));

                    self.file.rewrite_line(
                        if_conditions_lines[index].1, format!(
                            "{}:If:\"{}\":{}",
                            if_conditions_lines[index].0,
                            if_conditions_lines[index].2,
                            current_instruction_line));
                }

                let result = self.define_operation_node_variables(
                    define_if_else_node.condition.as_ref().unwrap())?;

                self.environments_stack.push_back(Environment {
                    scope: EnvironmentScope::If,
                    variables: HashMap::new()
                });

                /*
                * Set String as with the spaces do not edit
                * This is a fix for rewriting line to not overwrite file content
                */
                {
                    let current_line = self.get_current_line();
                    if_conditions_lines.push((
                        current_line, self.file.get_stream_position(),
                        result.0.clone()));

                    let space_line = self.append_empty_lines(String::from("0"));
                    self.file.writeln(format!(
                        "{}:If:\"{}\":{}", current_line, result.0, space_line));
                }

                self.generate_statements_node(&mut define_if_else_node.statements)?;

                {
                    let current_line = self.get_current_line();
                    lines_to_be_edited.push((current_line, self.file.get_stream_position()));

                    let space_line = self.append_empty_lines(String::from("0"));
                    self.file.writeln(format!("{current_line}:GoTo:{space_line}"));
                }

                self.environments_stack.pop_back();

                index += 1;
            }
        }

        /* Generate Else Statement */
        {
            if statement.define_else_node != None{
                self.environments_stack.push_back(Environment {
                    scope: EnvironmentScope::If,
                    variables: HashMap::new()
                });

                let define_else_node = statement.define_else_node.as_mut().unwrap();

                /* Re-Write Previous If Line */
                {
                    let current_instruction_line = self.append_empty_lines(
                        format!("{}", self.current_instruction_line + 1));

                    self.file.rewrite_line(
                        if_conditions_lines.last().as_ref().unwrap().1, format!(
                            "{}:If:\"{}\":{}",
                            if_conditions_lines.last().as_ref().unwrap().0,
                            if_conditions_lines.last().as_ref().unwrap().2,
                            current_instruction_line));
                }

                {
                    let current_line = self.get_current_line();
                    self.file.writeln(format!("{current_line}:Else:"));
                }

                self.generate_statements_node(&mut define_else_node.statements)?;

                self.environments_stack.pop_back();
            }
            else{
                /* Re-Write Previous If Line */
                {
                    let current_instruction_line = self.append_empty_lines(
                        format!("{}", self.current_instruction_line + 1));

                    self.file.rewrite_line(
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
            let current_instruction_line = self.append_empty_lines(
                format!("{}", self.current_instruction_line + 1));

            for (line, stream_position) in lines_to_be_edited{
                self.file.rewrite_line(
                    stream_position, format!("{line}:GoTo:{current_instruction_line}"));
            }
        }

        return Ok(());
    }

    fn define_operation_node_variables(
        &mut self, operation_node:
        &OperationNode
    ) -> Result<(String, TokenType), String>{

        match &operation_node.operator{
            Some(_operator) => {
                let mut left_variable_name = String::from("");
                let mut left_token_type = TokenType::BadToken;

                let mut right_variable_name = String::from("");
                let mut right_token_type = TokenType::BadToken;

                if operation_node.left != None{
                    let result = self.define_operation_node_variables(
                        operation_node.left.as_ref().unwrap())?;
                    left_variable_name = result.0;
                    left_token_type = result.1;
                }
                if operation_node.right != None{
                    let result = self.define_operation_node_variables(
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
                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Input:\"{temp_variable_name}\""));
                    }

                    let converted_to_variable_name = self.generate_temp_variable_name();

                    if operation_node.value.as_ref().unwrap().token_type == TokenType::Bool{
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:bool:\"{converted_to_variable_name}\":False"));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:bool:\"{}\":\"{}\"",
                                current_line, converted_to_variable_name, temp_variable_name));
                        }

                        return Ok((converted_to_variable_name, TokenType::Bool));
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::Int{
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:int:\"{converted_to_variable_name}\":0"));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:int:\"{}\":\"{}\"",
                                current_line, converted_to_variable_name, temp_variable_name));
                        }

                        return Ok((converted_to_variable_name, TokenType::IntNumber));
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::Double{
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{converted_to_variable_name}\":0"));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:double:\"{}\":\"{}\"",
                                current_line, converted_to_variable_name, temp_variable_name));
                        }

                        return Ok((converted_to_variable_name, TokenType::DoubleNumber));
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::Char{
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:char:\"{converted_to_variable_name}\":''"));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:char:\"{}\":\"{}\"",
                                current_line, converted_to_variable_name, temp_variable_name));
                        }

                        return Ok((converted_to_variable_name, TokenType::Character));
                    }
                    else if operation_node.value.as_ref().unwrap().token_type == TokenType::String{
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:string:\"{converted_to_variable_name}\":\"\""));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
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

                        let temp_variable_name = self.generate_temp_variable_name();

                        /* Initialize Double Variable */
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                        }

                        /* Convert Variable to double */
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:double:\"{}\":\"{}\"",
                                current_line, temp_variable_name, first_variable_name));
                        }

                        first_variable_name = temp_variable_name;
                    }
                    else if right_token_type == TokenType::IntNumber{

                        let temp_variable_name = self.generate_temp_variable_name();

                        /* Initialize Double Variable */
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                        }

                        /* Convert Variable to double */
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:double:\"{}\":\"{}\"",
                                current_line, temp_variable_name, second_variable_name));
                        }

                        second_variable_name = temp_variable_name;
                    }

                    if _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                        _operator == &OperatorType::Mul || _operator == &OperatorType::Div ||
                        _operator == &OperatorType::Mod
                    {

                        let temp_variable_name = self.generate_temp_variable_name();

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                                current_line, _operator, temp_variable_name,
                                first_variable_name, second_variable_name));
                        }

                        return Ok((temp_variable_name, TokenType::DoubleNumber));
                    }

                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:bool:\"{temp_variable_name}\":False"));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
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

                        let temp_variable_name = self.generate_temp_variable_name();

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:int:\"{temp_variable_name}\":0"));
                        }

                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Operation:{:?}:\"{}\":\"{}\":\"{}\"",
                                current_line, _operator, temp_variable_name,
                                left_variable_name, right_variable_name));
                        }

                        return Ok((temp_variable_name, TokenType::IntNumber))
                    }
                    else if _operator == &OperatorType::Div{

                        /* Initialize First Variable */
                        let first_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{first_variable_name}\":0"));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:double:\"{}\":\"{}\"",
                                current_line, first_variable_name, left_variable_name));
                        }

                        /* Initialize Second Variable */
                        let second_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{second_variable_name}\":0"));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:double:\"{}\":\"{}\"",
                                current_line, second_variable_name, right_variable_name));

                        }

                        /* Initialize Result Variable */
                        let temp_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:double:\"{temp_variable_name}\":0"));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Operation:Div:\"{}\":\"{}\":\"{}\"",
                                current_line, temp_variable_name,
                                first_variable_name, second_variable_name));
                        }

                        return Ok((temp_variable_name, TokenType::DoubleNumber));
                    }
                    else{

                        let variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:bool:\"{variable_name}\":False"));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
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
                        let first_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:string:\"{first_variable_name}\":\"\""));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:string:\"{}\":\"{}\"",
                                current_line, first_variable_name, left_variable_name));
                        }

                        /* Concat String */
                        let temp_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                                current_line, temp_variable_name,
                                first_variable_name, right_variable_name));
                        }

                        return Ok((temp_variable_name, TokenType::StringSequence));
                    }
                    else if right_token_type == TokenType::Character{

                        /* Initialize Second Variable */
                        let second_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:string:\"{second_variable_name}\":\"\""));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Convert:string:\"{}\":\"{}\"",
                                current_line, second_variable_name, right_variable_name));
                        }

                        /* Concat String */
                        let temp_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{}:Operation:Plus:\"{}\":\"{}\":\"{}\"",
                                current_line, temp_variable_name,
                                left_variable_name, second_variable_name));
                        }

                        return Ok((temp_variable_name, TokenType::StringSequence));
                    }
                    else{

                        /* Concat String */
                        let temp_variable_name = self.generate_temp_variable_name();
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
                                "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                        }
                        {
                            let current_line = self.get_current_line();
                            self.file.writeln(format!(
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
                    let first_variable_name = self.generate_temp_variable_name();
                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:string:\"{first_variable_name}\":\"\""));
                    }
                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Convert:string:\"{}\":\"{}\"",
                            current_line, first_variable_name, left_variable_name));
                    }

                    /* Initialize Second Variable */
                    let second_variable_name = self.generate_temp_variable_name();
                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:string:\"{second_variable_name}\":\"\""));
                    }
                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Convert:string:\"{}\":\"{}\"",
                            current_line, second_variable_name, right_variable_name));
                    }

                    /* Concat String */
                    let temp_variable_name = self.generate_temp_variable_name();
                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:string:\"{temp_variable_name}\":\"\""));
                    }
                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
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

                    let temp_variable_name = self.generate_temp_variable_name();

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:bool:\"{temp_variable_name}\":False"));
                    }

                    {
                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
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

                        let temp_variable_name = self.generate_temp_variable_name();

                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:bool:\"{temp_variable_name}\":True"));

                        return Ok((temp_variable_name, token.token_type.clone()));
                    },
                    TokenType::False => {

                        let temp_variable_name = self.generate_temp_variable_name();

                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{current_line}:Assign:bool:\"{temp_variable_name}\":False"));

                        return Ok((temp_variable_name, token.token_type.clone()));
                    },
                    TokenType::IntNumber => {

                        let temp_variable_name = self.generate_temp_variable_name();

                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Assign:int:\"{}\":{}",
                            current_line, temp_variable_name, token.value));

                        return Ok((temp_variable_name, token.token_type.clone()));
                    },
                    TokenType::DoubleNumber => {

                        let temp_variable_name = self.generate_temp_variable_name();

                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Assign:double:\"{}\":{}",
                            current_line, temp_variable_name, token.value));

                        return Ok((temp_variable_name, token.token_type.clone()));
                    },
                    TokenType::Character => {

                        let temp_variable_name = self.generate_temp_variable_name();

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

                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Assign:char:\"{}\":'{}'",
                            current_line, temp_variable_name, value));

                        return Ok((temp_variable_name, token.token_type.clone()));
                    },
                    TokenType::StringSequence => {

                        let temp_variable_name = self.generate_temp_variable_name();

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

                        let current_line = self.get_current_line();
                        self.file.writeln(format!(
                            "{}:Assign:string:\"{}\":\"{}\"",
                            current_line, temp_variable_name, value));

                        return Ok((temp_variable_name, token.token_type.clone()));
                    },
                    TokenType::Variable => {

                        let variable_type = self.get_variable_type(&token.value)?;

                        let variable_name = self.generate_variable_name(&token.value)?;

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
}
