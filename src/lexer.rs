use crate::character::Character;
use crate::file::File;
use crate::constants::{
    Mode,
    VARIABLE_MAX_LENGTH,
};
use crate::tokens::{
    Token,
    TokenType,
    get_token_type
};


#[derive(Debug)]
pub struct Lexer{
    file: File,
    pub current_line: u64,
    pub current_pos: u64,
    pub current_character: Character,
    pub mode: Mode
}

impl Lexer{
    pub fn new(file: File, mode: Mode) -> Result<Self, std::io::Error>{
        let mut file = file;
        return Ok(Lexer{
            current_character: file.read(),
            file,
            current_line: 1,
            current_pos: 1,
            mode
        })
    }
}


fn current(lexer: &mut Lexer) -> &Character{
    return &lexer.current_character;
}


fn peek(lexer: &mut Lexer, index: u64) -> Character{
    return lexer.file.peek(index - 1);
}


fn next(lexer: &mut Lexer) -> String{
    lexer.current_pos += 1;
    let old_character = lexer.current_character.clone();
    lexer.current_character = lexer.file.read();
    return old_character.to_string().clone();
}


fn add_line(lexer: &mut Lexer){
    lexer.current_line += 1;
    lexer.current_pos = 1;
}


fn get_number_token(lexer: &mut Lexer, line: u64, position: u64) -> Result<Token, String>{
    let mut lexer = lexer;

    let mut _number: String = next(&mut lexer);

    while current(&mut lexer).is_digit(){
        _number += &next(&mut lexer);
    }

    if current(&mut lexer).to_string() == "." && peek(&mut lexer, 1).is_digit() {
        _number += &next(&mut lexer);
        while current(&mut lexer).is_digit() {
            _number += &next(&mut lexer);
        }
        return Ok(Token{
            token_type: TokenType::DoubleNumber,
            start_line: line,
            start_pos: position,
            value: _number
        })
    }

    return Ok(Token{
        token_type: TokenType::IntNumber,
        start_line: line,
        start_pos: position,
        value: _number
    })
}


fn get_variable_token(lexer: &mut Lexer, line: u64, position: u64) -> Result<Token, String>{
    let mut lexer = lexer;
    let mut _variable = next(&mut lexer);
    let mut variable_length = 1;

    while current(&mut lexer).is_alpha() || current(&mut lexer).is_digit() ||
        current(&mut lexer).to_string() == "_"
    {
        _variable += &next(&mut lexer);
        variable_length += 1;

        if variable_length > VARIABLE_MAX_LENGTH{
            return Err(format!(
                "{}: Token Error -> {}, line {}:{}.",
                lexer.mode,
                format!(
                    "Variable length must not exceed {} characters",
                    VARIABLE_MAX_LENGTH),
                line, position));
        }
    }

    return Ok(Token{
        token_type: get_token_type(&_variable),
        start_line: line,
        start_pos: position,
        value: _variable
    });
}


fn get_string_token(lexer: &mut Lexer, line: u64, position: u64) -> Result<Token, String>{
    let mut lexer = lexer;

    let qotation_type = current(&mut lexer).to_string().clone();
    next(&mut lexer);
    let mut _string = String::from("");

    while !current(&mut lexer).is_eof(){

        if current(&mut lexer).is_newline(){
            _string += &next(&mut lexer);
            add_line(&mut lexer);
        }

        if current(&mut lexer).to_string() == "\\"{
            if peek(&mut lexer, 1).to_string() == &qotation_type{
                next(&mut lexer);
                _string += &next(&mut lexer);
            }
            else if peek(&mut lexer, 1).to_string() == "\\"{
                next(&mut lexer);
                _string += &next(&mut lexer);
            }
            else if peek(&mut lexer, 1).to_string() == "n"{
                next(&mut lexer); next(&mut lexer);
                _string += "\n";
            }
            else if peek(&mut lexer, 1).to_string() == "t"{
                next(&mut lexer); next(&mut lexer);
                _string += "\t";
            }
            else{
                _string += &next(&mut lexer);
                _string += &next(&mut lexer);
            }
        }
        else if current(&mut lexer).to_string() == &qotation_type {
            next(&mut lexer);

            if _string.len() > 1{
                return Ok(Token{
                    token_type: TokenType::StringSequence,
                    start_line: line,
                    start_pos: position,
                    value: _string
                })
            }
            return Ok(Token{
                token_type: TokenType::Character,
                start_line: line,
                start_pos: position,
                value: _string
            })
        }
        else{
            _string += &next(&mut lexer);
        }
    }

    return Err(format!(
        "{}: Syntax Error -> {}, line {}:{}.",
        lexer.mode, "End of file reached", line, position));
}


fn get_single_line_comment(lexer: &mut Lexer, line: u64, position: u64) -> Result<Token, String>{
    let mut lexer = lexer;
    let mut _comment = String::from("");

    while !current(&mut lexer).is_newline() && !current(&mut lexer).is_eof(){
        _comment += &next(&mut lexer);
    }

    return Ok(Token{
        token_type: TokenType::SingleLineComment,
        start_line: line,
        start_pos: position,
        value: _comment
    })
}


fn get_multiline_comment(lexer: &mut Lexer, line: u64, position: u64) -> Result<Token, String>{
    let mut lexer = lexer;
    let mut _comment: String = String::from("/*");

    next(&mut lexer); next(&mut lexer);

    while !current(&mut lexer).is_eof(){

        if current(&mut lexer).is_newline(){
            _comment += &next(&mut lexer);
            add_line(&mut lexer);
        }
        else if current(&mut lexer).to_string() == "*" && peek(&mut lexer, 1).to_string() == "/"{
            next(&mut lexer); next(&mut lexer);
            _comment.push_str("*/");
            return Ok(Token{
                token_type: TokenType::MultiLineComment,
                start_line: line,
                start_pos: position,
                value: _comment
            })
        }
        else{
            _comment += &next(&mut lexer);
        }
    }

    return Err(format!(
        "{}: Syntax Error -> {}, line {}:{}.",
        lexer.mode, "Unexcepeted end of file",
        line, position));
}


pub fn next_token(lexer: &mut Lexer) -> Result<Token, String>{
    let mut lexer = lexer;

    if current(&mut lexer).is_eof(){
        return Ok(Token{
            token_type: TokenType::Eof,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: String::new()
        })
    }

    else if current(&mut lexer).is_alpha() || current(&mut lexer).to_string() == "_" {
        let current_line = lexer.current_line.clone();
        let current_pos = lexer.current_pos.clone();

        return get_variable_token(&mut lexer, current_line, current_pos);
    }

    else if current(&mut lexer).is_digit(){
        let current_line = lexer.current_line.clone();
        let current_pos = lexer.current_pos.clone();

        return get_number_token(&mut lexer, current_line, current_pos)
    }

    else if current(&mut lexer).to_string() == "\r"{
        next(&mut lexer);
        lexer.current_pos -= 1;
        return next_token(&mut lexer);
    }

    else if current(&mut lexer).is_newline(){
        let line = lexer.current_line;
        let pos = lexer.current_pos;
        next(&mut lexer);
        add_line(&mut lexer);
        return Ok(Token{
            token_type: TokenType::NewLine,
            start_line: line,
            start_pos: pos,
            value: String::from("\n")
        })
    }

    else if current(&mut lexer).is_space(){
        return Ok(Token{
            token_type: TokenType::Space,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "="{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::Equal,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }

        return Ok(Token{
            token_type: TokenType::Assign,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "'" || current(&mut lexer).to_string() == "\"" {
        let current_line = lexer.current_line.clone();
        let current_pos = lexer.current_pos.clone();

        return get_string_token(&mut lexer, current_line, current_pos)
    }

    else if current(&mut lexer).to_string() == "("{
        return Ok(Token{
            token_type: TokenType::OpenParenthes,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }
    else if current(&mut lexer).to_string() == ")"{
        return Ok(Token{
            token_type: TokenType::CloseParenthes,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "{"{
        return Ok(Token{
            token_type: TokenType::OpenBracket,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }
    else if current(&mut lexer).to_string() == "}"{
        return Ok(Token{
            token_type: TokenType::CloseBracket,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "+"{
        if peek(&mut lexer, 1).to_string() == "=" {
            return Ok(Token{
                token_type: TokenType::PlusEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer),
            })
        }
        return Ok(Token{
            token_type: TokenType::Plus,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "#"{
        let current_line = lexer.current_line.clone();
        let current_pos = lexer.current_pos.clone();

        return get_single_line_comment(&mut lexer, current_line, current_pos);
    }

    else if current(&mut lexer).to_string() == "-"{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::MinusEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
        else if peek(&mut lexer, 1).is_digit(){
            let current_line = lexer.current_line.clone();
            let current_pos = lexer.current_pos.clone();

            return get_number_token(&mut lexer, current_line, current_pos)
        }
        return Ok(Token{
            token_type: TokenType::Minus,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "*"{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::MulEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
        return Ok(Token{
            token_type: TokenType::Mul,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "/"{
        if peek(&mut lexer, 1).to_string() == "*"{
            let current_line = lexer.current_line.clone();
            let current_pos = lexer.current_pos.clone();

            return get_multiline_comment(&mut lexer, current_line, current_pos);
        }
        else if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::DivEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
        return Ok(Token{
            token_type: TokenType::Div,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "%"{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::ModEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
        return Ok(Token{
            token_type: TokenType::Mod,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == ">"{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::GreaterThanOrEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
        return Ok(Token{
            token_type: TokenType::GreaterThan,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "<"{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::LessThanOrEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
        return Ok(Token{
            token_type: TokenType::LessThan,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if current(&mut lexer).to_string() == "!"{
        if peek(&mut lexer, 1).to_string() == "="{
            return Ok(Token{
                token_type: TokenType::NotEqual,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
    }

    else if current(&mut lexer).to_string() == "|"{
        if peek(&mut lexer, 1).to_string() == "|"{
            return Ok(Token{
                token_type: TokenType::Or,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
    }

    else if current(&mut lexer).to_string() == "&"{
        if peek(&mut lexer, 1).to_string() == "&"{
            return Ok(Token{
                token_type: TokenType::And,
                start_line: lexer.current_line,
                start_pos: lexer.current_pos,
                value: next(&mut lexer) + &next(&mut lexer)
            })
        }
    }

    else if current(&mut lexer).to_string() == ","{
        return Ok(Token{
            token_type: TokenType::Comma,
            start_line: lexer.current_line,
            start_pos: lexer.current_pos,
            value: next(&mut lexer)
        })
    }

    else if !current(&mut lexer).is_ascii(){
        return Err(format!(
            "{}: Syntax Error -> {}, line {}:{}.",
            lexer.mode, "Only support ascii characters",
            lexer.current_line, lexer.current_pos));
    }

    return Ok(Token{
        token_type: TokenType::BadToken,
        start_line: lexer.current_line,
        start_pos: lexer.current_pos,
        value: next(&mut lexer)
    })
}
