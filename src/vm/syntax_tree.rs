use crate::tokens::TokenType;


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
