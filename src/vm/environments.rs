use std::collections::HashMap;

use crate::vm::tokens::TokenType;


#[derive(Debug, Clone, PartialEq)]
pub enum EnvironmentScope{
    Main,
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


#[derive(Debug, Clone, PartialEq)]
pub struct Variable{
    pub variable_type: Option<TokenType>,
    pub name: Option<String>,
    pub value: Option<Value>,
}
