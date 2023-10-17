#[derive(Debug, PartialEq, Clone)]
pub enum TokenType{
    Eof,
    NewLine,
    Space,

    SingleLineComment,
    MultiLineComment,

    Bool,
    Int,
    Double,
    Char,
    String,
    Var,

    As,

    True,
    False,
    IntNumber,
    DoubleNumber,
    Character,
    StringSequence,

    Assign,
    Equal,
    GreaterThanOrEqual,
    GreaterThan,
    LessThanOrEqual,
    LessThan,
    NotEqual,

    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Mul,
    MulEqual,
    Div,
    DivEqual,
    Mod,
    ModEqual,

    Or,
    And,

    OpenParenthes,
    CloseParenthes,
    OpenBracket,
    CloseBracket,

    Variable,

    Print,
    Input,

    If,
    Else,

    For,
    Comma,
    In,
    Continue,
    Break,

    BadToken,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Token{
    pub token_type: TokenType,
    pub start_line: u64,
    pub start_pos: u64,
    pub value: String
}


pub fn get_token_type(variable: &String) -> TokenType{
    if variable == "bool"{
        return TokenType::Bool
    }
    else if variable == "int"{
        return TokenType::Int
    }
    else if variable == "double"{
        return TokenType::Double
    }
    else if variable == "char"{
        return TokenType::Char
    }
    else if variable == "string"{
        return TokenType::String
    }
    else if variable == "var"{
        return TokenType::Var
    }

    else if variable == "as"{
        return TokenType::As;
    }

    else if variable == "True"{
        return TokenType::True
    }
    else if variable == "False"{
        return TokenType::False
    }

    else if variable == "input"{
        return TokenType::Input
    }
    else if variable == "print"{
        return TokenType::Print
    }

    else if variable == "if"{
        return TokenType::If
    }
    else if variable == "else"{
        return TokenType::Else
    }

    else if variable == "for"{
        return TokenType::For
    }
    else if variable == "in"{
        return TokenType::In
    }
    else if variable == "continue"{
        return TokenType::Continue;
    }
    else if variable == "break"{
        return TokenType::Break;
    }

    return TokenType::Variable;
}
