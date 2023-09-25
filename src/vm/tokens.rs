#[derive(Debug, PartialEq, Clone)]
pub enum TokenType{
    Bool,
    Int,
    Double,
    Char,
    String,

    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    LessThanOrEqual,
    LessThan,
    NotEqual,

    Plus,
    Minus,
    Mul,
    Div,
    Mod,

    Or,
    And,

    BadToken,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token{
    pub token_type: TokenType,
    pub start_line: u64,
    pub start_pos: u64,
    pub value: String
}
