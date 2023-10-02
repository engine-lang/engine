use std::collections::{
    HashMap,
    VecDeque
};
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
    DefineIfStatementNode
};
use crate::tokens::TokenType;
use crate::constants::Mode;

use crate::compiler::file::File;


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
            variables: HashMap::new()
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
}

impl CodeGenerator{
    pub fn generate(&mut self) -> Result<(), String>{
        self.file.writeln(String::from("#![allow(arithmetic_overflow)]"));
        self.file.writeln(String::from("use std::io;"));
        self.file.writeln(String::from("use std::panic;"));
        self.file.writeln(String::from("fn main(){"));
        self.file.writeln(String::from("use std::io::Write;"));
        self.file.writeln(String::from("panic::set_hook(Box::new(|panic_info| {"));
        self.file.writeln(String::from("if let Some(panic_message) = panic_info.payload().downcast_ref::<String>() {"));
        self.file.writeln(String::from("println!(\"{}\", panic_message);"));
        self.file.writeln(String::from("} else if let Some(panic_message) = panic_info.payload().downcast_ref::<&str>() {"));
        self.file.writeln(String::from("println!(\"{}\", panic_message);"));
        self.file.writeln(String::from("} else {"));
        self.file.writeln(String::from("println!(\"Engine Compiler -> Interperter Error {}\", panic_info);"));
        self.file.writeln(String::from("}"));
        self.file.writeln(String::from("}));"));
        self.generate_statements_node(&mut self.syntax_tree.clone())?;
        self.file.writeln(String::from("}"));

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
            self.generate_define_if_statement(
                &mut statement.define_if_statement.as_mut().unwrap())?;
        }

        return Ok(());
    }

    fn generate_define_bool_variable(
        &mut self, statement: &DefineBoolNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        self.file.writeln(format!(
            "let mut variable_{}: bool = {};",
            statement.name.as_ref().unwrap().value,
            result.0,
        ));

        self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefineIntNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        self.file.writeln(format!(
            "let mut variable_{}: i64 = {} as i64;",
            statement.name.as_ref().unwrap().value,
            result.0
        ));

        self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefineDoubleNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        self.file.writeln(format!(
            "let mut variable_{}: f64 = {} as f64;",
            statement.name.as_ref().unwrap().value,
            result.0
        ));

        self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefineCharNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        if result.1 == TokenType::Character{
            self.file.writeln(format!(
                "let mut variable_{}: char = {};",
                statement.name.as_ref().unwrap().value,
                result.0
            ));
        }
        else{
            self.file.writeln(format!(
                "let mut variable_{}: char = {}.chars().nth(0).unwrap();",
                statement.name.as_ref().unwrap().value,
                result.0
            ));
        }

        self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefineStringNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        self.file.writeln(format!(
            "let mut variable_{}: String = {};",
            statement.name.as_ref().unwrap().value,
            result.0
        ));

        self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefineVarNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        self.file.writeln(format!(
            "let mut variable_{} = {};",
            statement.name.as_ref().unwrap().value,
            result.0
        ));

        self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefineVariableNode
    ) -> Result<(), String>{

        let result = self.define_operation_node_variables(
            statement.left.as_ref().unwrap())?;

        if self.is_variable_exists(&statement.name.as_ref().unwrap().value){
            let variable_type = self.get_variable_type(
                &statement.name.as_ref().unwrap().value)?;

            if variable_type == TokenType::Double{
                self.file.writeln(format!(
                    "variable_{} {} {} as f64;",
                    statement.name.as_ref().unwrap().value,
                    statement.operator.as_ref().unwrap().value,
                    result.0
                ));
            }
            else if variable_type == TokenType::Int{
                self.file.writeln(format!(
                    "variable_{} {} {} as i64;",
                    statement.name.as_ref().unwrap().value,
                    statement.operator.as_ref().unwrap().value,
                    result.0
                ));
            }
            else if variable_type == TokenType::Char{
                if result.1 == TokenType::Character{
                    self.file.writeln(format!(
                        "variable_{} = {};",
                        statement.name.as_ref().unwrap().value,
                        result.0
                    ));
                }
                else{
                    self.file.writeln(format!(
                        "variable_{} = {}.chars().nth(0).unwrap();",
                        statement.name.as_ref().unwrap().value,
                        result.0
                    ));
                }
            }
            else if variable_type == TokenType::String{
                if result.1 == TokenType::Character{
                    self.file.writeln(format!(
                        "variable_{}.push({});",
                        statement.name.as_ref().unwrap().value,
                        result.0
                    ));
                }
                else{
                    if statement.operator.as_ref().unwrap().token_type == TokenType::Assign{
                        self.file.writeln(format!(
                            "variable_{} {} {}.clone();",
                            statement.name.as_ref().unwrap().value,
                            statement.operator.as_ref().unwrap().value,
                            result.0
                        ));
                    }
                    else{
                        self.file.writeln(format!(
                            "variable_{} {} &{}.clone();",
                            statement.name.as_ref().unwrap().value,
                            statement.operator.as_ref().unwrap().value,
                            result.0
                        ));
                    }
                }
            }
            else{
                self.file.writeln(format!(
                    "variable_{} = {};",
                    statement.name.as_ref().unwrap().value,
                    result.0
                ));
            }
        }
        else{
            self.file.writeln(format!(
                "let mut variable_{} = {};",
                statement.name.as_ref().unwrap().value,
                result.0
            ));

            self.insert_variable_into_environments_stack(
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
        &mut self, statement: &DefinePrintNode
    ) -> Result<(), String>{

        /* Print Variable */
        self.file.writeln(format!(
            "print!(\"{{}}\", variable_{});",
            statement.variable.as_ref().unwrap().value));

        /* Flush Variable */
        self.file.writeln(format!("if io::stdout().flush().is_err(){{"));
        self.file.writeln(format!(
            "panic!(\"{}\");",
            format!(
                "Engine Compiler: Compiler Error -> Error in printing to console, line: {}:{}",
                statement.variable.as_ref().unwrap().start_line,
                statement.variable.as_ref().unwrap().start_pos)));
        self.file.writeln(format!("}}"));

        return Ok(());
    }

    fn generate_define_if_statement(
        &mut self, statement: &mut DefineIfStatementNode
    ) -> Result<(), String>{

        let if_condition_result;
        let mut else_if_condition_results: Vec<(String, TokenType)> = Vec::new();

        /* Define Conditions */
        {
            /* Define If Condition */
            {
                let define_if_node = statement.define_if_node.as_mut().unwrap();

                if_condition_result = self.define_operation_node_variables(
                    define_if_node.condition.as_ref().unwrap())?;
            }

            /* Define Else if Conditions */
            {
                for define_if_else_node in &mut statement.define_if_else_nodes{

                    else_if_condition_results.push(self.define_operation_node_variables(
                        define_if_else_node.condition.as_ref().unwrap())?);
                }
            }
        }

        /* Generate If Statement */
        {
            self.environments_stack.push_back(Environment {
                scope: EnvironmentScope::If,
                variables: HashMap::new()
            });

            let define_if_node = statement.define_if_node.as_mut().unwrap();

            self.file.writeln(format!("if {}{{", if_condition_result.0));

            self.generate_statements_node(&mut define_if_node.statements)?;

            self.file.writeln(String::from("}"));

            self.environments_stack.pop_back();
        }

        /* Generate Else if Statements */
        {
            let mut index = 0;
            for define_if_else_node in &mut statement.define_if_else_nodes{
                self.environments_stack.push_back(Environment {
                    scope: EnvironmentScope::If,
                    variables: HashMap::new()
                });

                let result = &else_if_condition_results[index];

                self.file.writeln(format!("else if {}{{", result.0));

                self.generate_statements_node(&mut define_if_else_node.statements)?;

                self.file.writeln(String::from("}"));

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

                self.file.writeln(String::from("else {"));

                self.generate_statements_node(&mut define_else_node.statements)?;

                self.file.writeln(String::from("}"));

                self.environments_stack.pop_back();
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

                /* Define Input */
                if _operator == &OperatorType::Convert{
                    /* Read Input */
                    let variable_name = format!(
                        "temp{}", self.generate_variable_name().clone());

                    self.file.writeln(format!(
                        "let mut {} = String::new();",
                        variable_name));

                    self.file.writeln(format!(
                        "if io::stdin().read_line(&mut {}).is_err(){{",
                        variable_name));
                    self.file.writeln(format!(
                        "panic!(\"{}\")",
                        format!(
                            "Engine Compiler: Compiler Error -> Failed to read input, line: {}:{}",
                            operation_node.value.as_ref().unwrap().start_line,
                            operation_node.value.as_ref().unwrap().start_pos)));
                    self.file.writeln(format!("}}"));

                    self.file.writeln(format!(
                        "let mut {} = String::from({}.trim_end());",
                        variable_name, variable_name));

                    /* Convert Type */
                    let convert_to_variable = format!(
                        "temp{}", self.generate_variable_name().clone());

                    self.file.writeln(format!(
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

                    self.file.writeln(format!("if {}.is_err(){{", convert_to_variable));
                    self.file.writeln(format!(
                        "panic!(\"{}\");",
                        format!(
                            "Engine Compiler: Convert Error -> {}, line: {}:{}",
                            format!(
                                "can't convert from `String` to `{:?}`",
                                operation_node.value.as_ref().unwrap().token_type),
                            operation_node.value.as_ref().unwrap().start_line,
                            operation_node.value.as_ref().unwrap().start_pos)));
                    self.file.writeln(format!("}}"));
                    self.file.writeln(format!(
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
                        "temp{}", self.generate_variable_name().clone());

                    let operator_type = if
                        _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                        _operator == &OperatorType::Mul || _operator == &OperatorType::Div ||
                        _operator == &OperatorType::Mod {"f64"} else {"bool"};

                    if left_token_type == TokenType::IntNumber{
                        self.file.writeln(format!(
                            "let mut {}: {} = {} as f64 {} {} as f64;",
                            variable_name,
                            operator_type,
                            left_variable_name,
                            operation_node.value.as_ref().unwrap().value.clone(),
                            right_variable_name,
                        ));
                    }
                    else if right_token_type == TokenType::IntNumber{
                        self.file.writeln(format!(
                            "let mut {}: {} = {} as f64 {} {} as f64;",
                            variable_name,
                            operator_type,
                            left_variable_name,
                            operation_node.value.as_ref().unwrap().value.clone(),
                            right_variable_name,
                        ));
                    }
                    else{
                        self.file.writeln(format!(
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
                        "temp{}", self.generate_variable_name().clone());

                    if _operator == &OperatorType::Plus || _operator == &OperatorType::Minus ||
                        _operator == &OperatorType::Mul || _operator == &OperatorType::Mod{

                        self.file.writeln(format!(
                            "let mut {}: i64 = {} as i64 {} {} as i64;",
                            variable_name,
                            left_variable_name,
                            operation_node.value.as_ref().unwrap().value.clone(),
                            right_variable_name,
                        ));
                        return Ok((variable_name, TokenType::IntNumber))
                    }
                    else if _operator == &OperatorType::Div{
                        self.file.writeln(format!(
                            "let mut {}: f64 = {} as f64 {} {} as f64;",
                            variable_name,
                            left_variable_name,
                            operation_node.value.as_ref().unwrap().value.clone(),
                            right_variable_name,
                        ));
                        return Ok((variable_name, TokenType::DoubleNumber));
                    }
                    else{
                        self.file.writeln(format!(
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
                        "temp{}", self.generate_variable_name().clone());

                    if left_token_type == TokenType::Character{
                        self.file.writeln(format!(
                            "let mut {}: String = String::from({}) + &{}.clone();",
                            variable_name,
                            left_variable_name,
                            right_variable_name,
                        ));
                    }
                    else if right_token_type == TokenType::Character{
                        self.file.writeln(format!(
                            "let mut {}: String = {}.clone() + &String::from({});",
                            variable_name,
                            left_variable_name,
                            right_variable_name,
                        ));
                    }
                    else{
                        self.file.write(String::from(format!(
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
                        "temp{}", self.generate_variable_name().clone());

                    self.file.writeln(format!(
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
                        "temp{}", self.generate_variable_name().clone());

                    self.file.writeln(format!(
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
                    "temp{}", self.generate_variable_name().clone());

                match token.token_type{
                    TokenType::IntNumber => {
                        self.file.writeln(format!(
                            "let mut {}: i64 = {};", variable_name, token.value));
                    },
                    TokenType::DoubleNumber => {
                        self.file.writeln(format!(
                            "let mut {}: f64 = {};", variable_name, token.value));
                    },
                    TokenType::True => {
                        self.file.writeln(format!(
                            "let mut {}: bool = true;", variable_name));
                    },
                    TokenType::False => {
                        self.file.writeln(format!(
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
                        self.file.writeln(format!(
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
                        self.file.writeln(format!(
                            "let mut {}: String = String::from(\"{}\");",
                            variable_name, value));
                    },
                    TokenType::Variable => {
                        let variable_type = self.get_variable_type(&token.value)?;

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
