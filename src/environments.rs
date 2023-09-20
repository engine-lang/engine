use std::collections::HashMap;

use crate::tokens::TokenType;


#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentScope{
    Main,
    If,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Environment{
    pub scope: EnvironmentScope,
    pub variables: HashMap<String, Option<Variable>>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Variable{
    pub variable_type: Option<TokenType>,
    pub name: Option<String>,
    pub value: Option<String>,
    pub is_reasigned: bool,
}

impl Variable{
    pub fn new() -> Self{
        return Variable{
            variable_type: None,
            name: None,
            value: None,
            is_reasigned: false
        };
    }
}
