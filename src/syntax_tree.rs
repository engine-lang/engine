use std::collections::VecDeque;

use crate::tokens::{
    Token,
    TokenType
};


#[derive(Debug, Clone, PartialEq)]
pub enum StatementType{
    Discarded,

    DefineBool,
    DefineInt,
    DefineDouble,
    DefineChar,
    DefineString,
    DefineVariable,
    DefineVar,

    Print,

    DefineIf,
}


#[derive(Debug, Clone, PartialEq)]
pub enum OperatorType{
    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    Convert,

    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    LessThanOrEqual,
    LessThan,
    NotEqual,

    Or,
    And,
}


#[derive(Debug, Clone, PartialEq)]
pub struct OperationNode{
    pub value: Option<Token>,
    pub operator: Option<OperatorType>,
    pub left: Option<Box<OperationNode>>,
    pub right: Option<Box<OperationNode>>,
}

impl OperationNode{
    pub fn new() -> Self{
        return OperationNode{
            value: None,
            operator: None,
            left: None,
            right: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineBoolNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineBoolNode{
    pub fn new() -> Self{
        return DefineBoolNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineIntNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineIntNode{
    pub fn new() -> Self{
        return DefineIntNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineDoubleNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineDoubleNode{
    pub fn new() -> Self{
        return DefineDoubleNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineCharNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineCharNode{
    pub fn new() -> Self{
        return DefineCharNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineStringNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineStringNode{
    pub fn new() -> Self{
        return DefineStringNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineVariableNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineVariableNode{
    pub fn new() -> Self{
        return DefineVariableNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineVarNode{
    pub name: Option<Token>,
    pub operator: Option<Token>,
    pub left: Option<OperationNode>,
}
impl DefineVarNode{
    pub fn new() -> Self{
        return DefineVarNode{
            name: None,
            operator: None,
            left: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefinePrintNode{
    pub variable: Option<Token>,
}
impl DefinePrintNode{
    pub fn new() -> Self{
        return DefinePrintNode{
            variable: None,
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineElseNode{
    pub statements: StatementsNode,
}
impl DefineElseNode{
    pub fn new() -> Self{
        return DefineElseNode{
            statements: StatementsNode::new()
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineIfElseNode{
    pub token: Option<Token>,
    pub condition: Option<OperationNode>,
    pub statements: StatementsNode,
}
impl DefineIfElseNode{
    pub fn new() -> Self{
        return DefineIfElseNode{
            token: None,
            condition: None,
            statements: StatementsNode::new()
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineIfNode{
    pub token: Option<Token>,
    pub condition: Option<OperationNode>,
    pub statements: StatementsNode,
}
impl DefineIfNode{
    pub fn new() -> Self{
        return DefineIfNode{
            token: None,
            condition: None,
            statements: StatementsNode::new()
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DefineIfStatementNode{
    pub define_if_node: Option<DefineIfNode>,
    pub define_if_else_nodes: VecDeque<DefineIfElseNode>,
    pub define_else_node: Option<DefineElseNode>,
}
impl DefineIfStatementNode{
    pub fn new() -> Self{
        return DefineIfStatementNode{
            define_if_node: None,
            define_if_else_nodes: VecDeque::new(),
            define_else_node: None,
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct StatementNode{
    pub statement_type: Option<StatementType>,

    pub define_bool_statement: Option<DefineBoolNode>,
    pub define_int_statement: Option<DefineIntNode>,
    pub define_double_statement: Option<DefineDoubleNode>,
    pub define_char_statement: Option<DefineCharNode>,
    pub define_string_statement: Option<DefineStringNode>,
    pub define_variable_statement: Option<DefineVariableNode>,
    pub define_var_statement: Option<DefineVarNode>,

    pub define_print_statement: Option<DefinePrintNode>,

    pub define_if_statement: Option<DefineIfStatementNode>,
}

impl StatementNode{
    pub fn new() -> Self{
        return StatementNode{
            statement_type: None,

            define_bool_statement: None,
            define_int_statement: None,
            define_double_statement: None,
            define_char_statement: None,
            define_string_statement: None,
            define_variable_statement: None,
            define_var_statement: None,

            define_print_statement: None,

            define_if_statement: None
        };
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct StatementsNode{
    pub statements: VecDeque<StatementNode>,
}
impl StatementsNode{
    pub fn new() -> Self{
        return StatementsNode{
            statements: VecDeque::new()
        };
    }
}



/* VM Instructions */
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    Assign,
    Convert,
    Operation,

    Print,
    Input,

    If,
    Else,
    GoTo,

    End
}


#[derive(Debug, Clone, PartialEq)]
pub struct AssignInstruction{
    pub assign_type: Option<TokenType>,
    pub variable_name: Option<String>,
    pub value: Option<String>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ConvertInstruction{
    pub convertion_type: Option<TokenType>,
    pub convert_to: Option<String>,
    pub convert_from: Option<String>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct OperationInstruction{
    pub operation_type: Option<TokenType>,
    pub left_variable: Option<String>,
    pub right_variable: Option<String>,
    pub assign_to_variable: Option<String>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct PrintInstruction{
    pub variable_name: Option<String>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct InputInstruction{
    pub variable_name: Option<String>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct IfInstruction{
    pub variable_name: Option<String>,
    pub condition_fail_goto_line: Option<u128>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct ElseInstruction{
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct GoToInstruction{
    pub goto_line: Option<u128>,
    pub line: Option<u128>,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Instruction{
    pub instruction_type: Option<InstructionType>,
    pub assign_instruction: Option<AssignInstruction>,
    pub convert_instruction: Option<ConvertInstruction>,
    pub operation_instruction: Option<OperationInstruction>,
    pub print_instruction: Option<PrintInstruction>,
    pub input_instruction: Option<InputInstruction>,

    pub if_instruction: Option<IfInstruction>,
    pub else_instruction: Option<ElseInstruction>,
    pub goto_instruction: Option<GoToInstruction>,
}


pub fn construct_expression_node(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut tokens = tokens;
    return __first_precedence_expression(&mut tokens);
}


/** [ || ] */
fn __first_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    node.left = Some(Box::from( __second_precedence_expression(tokens) ));

    let maybe_token = match tokens.pop_front(){
        Some(token) => token,
        _ => return *node.left.unwrap()
    };

    match maybe_token.token_type{
        TokenType::Or => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Or);
            node.right = Some(Box::from( __first_precedence_expression(tokens) ));
        },
        _ => return {
            tokens.push_front(maybe_token);
            *node.left.unwrap()
        },
    }

    return node;
}


/** [ && ]  */
fn __second_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    node.left = Some(Box::from( __third_precedence_expression(tokens) ));

    let maybe_token = match tokens.pop_front(){
        Some(token) => token,
        _ => return *node.left.unwrap()
    };

    match maybe_token.token_type{
        TokenType::And => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::And);
            node.right = Some(Box::from( __second_precedence_expression(tokens) ));
        },
        _ => return {
            tokens.push_front(maybe_token);
            *node.left.unwrap()
        },
    }

    return node;
}


/** [ | ] */
fn __third_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut tokens = tokens;
    return __fourth_precedence_expression(&mut tokens);
}


/** [ ^ ] */
fn __fourth_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut tokens = tokens;
    return __fifth_precedence_expression(&mut tokens);
}


/** [ & ] */
fn __fifth_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut tokens = tokens;
    return __six_precedence_expression(&mut tokens);
}


/** [ != == ] */
fn __six_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    node.left = Some(Box::from( __seven_precedence_expression(tokens) ));

    let maybe_token = match tokens.pop_front(){
        Some(token) => token,
        _ => return *node.left.unwrap()
    };

    match maybe_token.token_type{
        TokenType::Equal => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Equal);
            node.right = Some(Box::from( __six_precedence_expression(tokens) ));
        },
        TokenType::NotEqual => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::NotEqual);
            node.right = Some(Box::from( __six_precedence_expression(tokens) ));
        },
        _ => return {
            tokens.push_front(maybe_token);
            *node.left.unwrap()
        },
    }

    return node;
}


/** [ < <= > >= ] */
fn __seven_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    node.left = Some(Box::from( __eight_precedence_expression(tokens) ));

    let maybe_token = match tokens.pop_front(){
        Some(token) => token,
        _ => return *node.left.unwrap()
    };

    match maybe_token.token_type{
        TokenType::GreaterThan => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::GreaterThan);
            node.right = Some(Box::from( __seven_precedence_expression(tokens) ));
        },
        TokenType::GreaterThanOrEqual => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::GreaterThanOrEqual);
            node.right = Some(Box::from( __seven_precedence_expression(tokens) ));
        },
        TokenType::LessThan => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::LessThan);
            node.right = Some(Box::from( __seven_precedence_expression(tokens) ));
        },
        TokenType::LessThanOrEqual => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::LessThanOrEqual);
            node.right = Some(Box::from( __seven_precedence_expression(tokens) ));
        },
        _ => return {
            tokens.push_front(maybe_token);
            *node.left.unwrap()
        },
    }

    return node;
}


/** [ + - ] */
fn __eight_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    node.left = Some(Box::from( __nine_precedence_expression(tokens) ));

    let maybe_token = match tokens.pop_front(){
        Some(token) => token,
        _ => return *node.left.unwrap()
    };

    match maybe_token.token_type{
        TokenType::Plus => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Plus);
            node.right = Some(Box::from( __eight_precedence_expression(tokens) ));
        },
        TokenType::Minus => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Minus);
            node.right = Some(Box::from( __eight_precedence_expression(tokens) ));
        },
        _ => return {
            tokens.push_front(maybe_token);
            *node.left.unwrap()
        },
    }

    return node;
}


/** [ * / % ] */
fn __nine_precedence_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    node.left = Some(Box::from( __last_expression(tokens) ));

    let maybe_token = match tokens.pop_front(){
        Some(token) => token,
        _ => return *node.left.unwrap()
    };

    match maybe_token.token_type{
        TokenType::Mul => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Mul);
            node.right = Some(Box::from( __nine_precedence_expression(tokens) ));
        },
        TokenType::Div => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Div);
            node.right = Some(Box::from( __nine_precedence_expression(tokens) ));
        },
        TokenType::Mod => {
            node.value = Some(maybe_token);
            node.operator = Some(OperatorType::Mod);
            node.right = Some(Box::from( __nine_precedence_expression(tokens) ));
        },
        _ => return {
            tokens.push_front(maybe_token);
            *node.left.unwrap()
        },
    }

    return node;
}


/** [ True False Number Character StringSequence Null Variable () ! ] */
fn __last_expression(tokens: &mut VecDeque<Token>) -> OperationNode{
    let mut node = OperationNode::new();

    let maybe_token = tokens.pop_front().unwrap();

    if maybe_token.token_type == TokenType::IntNumber{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::DoubleNumber{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::Variable{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::True{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::False{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::Character{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::StringSequence{
        node.value = Some(maybe_token.clone());
    }
    else if maybe_token.token_type == TokenType::OpenParenthes{
        let node = __first_precedence_expression(tokens);
        tokens.pop_front().unwrap();

        return node;
    }
    else if maybe_token.token_type == TokenType::Input{
        node.operator = Some(OperatorType::Convert);
        node.value = Some(tokens.pop_front().unwrap());
    }

    return node;
}
