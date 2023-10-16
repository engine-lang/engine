use std::collections::HashMap;

use crate::tokens::TokenType;


#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentScope{
    Main,
    If,
    ForLoop,
}


#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Boolean,
    Integer,
    Double,
    Character,
    String,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Environment{
    pub scope: EnvironmentScope,
    pub variables: HashMap<String, Option<Variable>>,
    pub internal_variables: HashMap<String, Variable>,
    pub stop_statements_execution: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Variable{
    pub variable_type: Option<TokenType>,
    pub name: Option<String>,
    pub value: Option<Value>,
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


#[derive(Debug, Clone, PartialEq)]
pub struct Value{
    pub value_type: Option<ValueType>,
    pub string_value: Option<String>,
    pub boolean: Option<bool>,
    pub int: Option<i64>,
    pub double: Option<f64>,
    pub character: Option<char>,
    pub string: Option<String>,
}

impl Value{
    pub fn new() -> Self{
        return Value{
            value_type: None,
            string_value: None,
            boolean: None,
            int: None,
            double: None,
            character: None,
            string: None
        };
    }
}
