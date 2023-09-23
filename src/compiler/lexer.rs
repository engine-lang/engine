use crate::compiler::character::Character;
use crate::compiler::tokens::{
    Token,
    TokenType
};
use crate::compiler::file::File;
use crate::compiler::tokens::get_token_type;
use crate::constants::VARIABLE_MAX_LENGTH;


#[derive(Debug)]
pub struct Lexer{
    file: File,
    pub current_line: u64,
    pub current_pos: u64,
    pub current_character: Character
}

impl Lexer{
    pub fn new(file: File) -> Result<Self, std::io::Error>{
        let mut file = file;
        return Ok(Lexer{
            current_character: file.read(),
            file,
            current_line: 1,
            current_pos: 1
        })
    }
}

impl Lexer{
    fn current(&mut self) -> &Character{
        return &self.current_character;
    }

    fn peek(&mut self, index: u64) -> Character{
        return self.file.peek(index - 1);
    }

    fn next(&mut self) -> String{
        self.current_pos += 1;
        let old_character = self.current_character.clone();
        self.current_character = self.file.read();
        return old_character.to_string().clone();
    }

    fn add_line(&mut self){
        self.current_line += 1;
        self.current_pos = 1;
    }

    fn get_number_token(&mut self, line: u64, position: u64) -> Result<Token, String>{
        let mut _number: String = self.next();

        while self.current().is_digit(){
            _number += &self.next();
        }

        if self.current().to_string() == "." && self.peek(1).is_digit() {
            _number += &self.next();
            while self.current().is_digit() {
                _number += &self.next();
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

    fn get_variable_token(&mut self, line: u64, position: u64) -> Result<Token, String>{
        let mut _variable = self.next();

        let mut variable_length = 1;

        while self.current().is_alpha() || self.current().is_digit() ||
            self.current().to_string() == "_"
        {
            _variable += &self.next();
            variable_length += 1;

            if variable_length > VARIABLE_MAX_LENGTH{
                return Err(format!(
                    "Engine Compiler: Token Error -> {}, line {}:{}.",
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

    fn get_string_token(&mut self, line: u64, position: u64) -> Result<Token, String>{
        let qotation_type = self.current().to_string().clone();
        self.next();
        let mut _string = String::from("");

        while !self.current().is_eof(){

            if self.current().is_newline(){
                _string += &self.next();
                self.add_line();
            }

            if self.current().to_string() == "\\"{
                if self.peek(1).to_string() == &qotation_type{
                    self.next();
                    _string += &self.next();
                }
                else if self.peek(1).to_string() == "\\"{
                    self.next();
                    _string += &self.next();
                }
                else if self.peek(1).to_string() == "n"{
                    self.next(); self.next();
                    _string += "\n";
                }
                else if self.peek(1).to_string() == "t"{
                    self.next(); self.next();
                    _string += "\t";
                }
                else{
                    _string += &self.next();
                    _string += &self.next();
                }
            }
            else if self.current().to_string() == &qotation_type {
                self.next();

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
                _string += &self.next();
            }
        }

        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            "End of file reached", line, position));
    }

    fn get_single_line_comment(&mut self, line: u64, position: u64) -> Result<Token, String>{
        let mut _comment = String::from("");

        while !self.current().is_newline() && !self.current().is_eof(){
            _comment += &self.next();
        }

        return Ok(Token{
            token_type: TokenType::SingleLineComment,
            start_line: line,
            start_pos: position,
            value: _comment
        })
    }

    fn get_multiline_comment(&mut self, line: u64, position: u64) -> Result<Token, String>{
        let mut _comment: String = String::from("/*");
        self.next(); self.next();

        while !self.current().is_eof(){

            if self.current().is_newline(){
                _comment += &self.next();
                self.add_line();
            }
            else if self.current().to_string() == "*" && self.peek(1).to_string() == "/"{
                self.next(); self.next();
                _comment.push_str("*/");
                return Ok(Token{
                    token_type: TokenType::MultiLineComment,
                    start_line: line,
                    start_pos: position,
                    value: _comment
                })
            }
            else{
                _comment += &self.next();
            }
        }

        return Err(format!(
            "Engine Compiler: Syntax Error -> {}, line {}:{}.",
            "Unexcepeted end of file",
            line, position));
    }
}

impl Lexer{
    pub fn next_token(&mut self) -> Result<Token, String>{
        if self.current().is_eof(){
            return Ok(Token{
                token_type: TokenType::Eof,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: String::new()
            })
        }

        else if self.current().is_alpha() || self.current().to_string() == "_" {
            return self.get_variable_token(self.current_line, self.current_pos);
        }

        else if self.current().is_digit(){
            return self.get_number_token(self.current_line, self.current_pos)
        }

        else if self.current().to_string() == "\r"{
            self.next();
            self.current_pos -= 1;
            return self.next_token();
        }

        else if self.current().is_newline(){
            let line = self.current_line;
            let pos = self.current_pos;
            self.next();
            self.add_line();
            return Ok(Token{
                token_type: TokenType::NewLine,
                start_line: line,
                start_pos: pos,
                value: String::from("\n")
            })
        }

        else if self.current().is_space(){
            return Ok(Token{
                token_type: TokenType::Space,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "="{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::Equal,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }

            return Ok(Token{
                token_type: TokenType::Assign,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "'" || self.current().to_string() == "\"" {
            return self.get_string_token(self.current_line, self.current_pos)
        }

        else if self.current().to_string() == "("{
            return Ok(Token{
                token_type: TokenType::OpenParenthes,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }
        else if self.current().to_string() == ")"{
            return Ok(Token{
                token_type: TokenType::CloseParenthes,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "{"{
            return Ok(Token{
                token_type: TokenType::OpenBracket,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }
        else if self.current().to_string() == "}"{
            return Ok(Token{
                token_type: TokenType::CloseBracket,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "+"{
            if self.peek(1).to_string() == "=" {
                return Ok(Token{
                    token_type: TokenType::PlusEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next(),
                })
            }
            return Ok(Token{
                token_type: TokenType::Plus,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "#"{
            return self.get_single_line_comment(self.current_line, self.current_pos);
        }

        else if self.current().to_string() == "-"{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::MinusEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
            else if self.peek(1).is_digit(){
                return self.get_number_token(self.current_line, self.current_pos)
            }
            return Ok(Token{
                token_type: TokenType::Minus,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "*"{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::MulEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
            return Ok(Token{
                token_type: TokenType::Mul,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "/"{
            if self.peek(1).to_string() == "*"{
                return self.get_multiline_comment(self.current_line, self.current_pos);
            }
            else if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::DivEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
            return Ok(Token{
                token_type: TokenType::Div,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "%"{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::ModEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
            return Ok(Token{
                token_type: TokenType::Mod,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == ">"{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::GreaterThanOrEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
            return Ok(Token{
                token_type: TokenType::GreaterThan,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "<"{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::LessThanOrEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
            return Ok(Token{
                token_type: TokenType::LessThan,
                start_line: self.current_line,
                start_pos: self.current_pos,
                value: self.next()
            })
        }

        else if self.current().to_string() == "!"{
            if self.peek(1).to_string() == "="{
                return Ok(Token{
                    token_type: TokenType::NotEqual,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
        }

        else if self.current().to_string() == "|"{
            if self.peek(1).to_string() == "|"{
                return Ok(Token{
                    token_type: TokenType::Or,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
        }

        else if self.current().to_string() == "&"{
            if self.peek(1).to_string() == "&"{
                return Ok(Token{
                    token_type: TokenType::And,
                    start_line: self.current_line,
                    start_pos: self.current_pos,
                    value: self.next() + &self.next()
                })
            }
        }

        else if !self.current().is_ascii(){
            return Err(format!(
                "Engine Compiler: Syntax Error -> {}, line {}:{}.",
                "Only support ascii characters",
                self.current_line, self.current_pos));
        }

        return Ok(Token{
            token_type: TokenType::BadToken,
            start_line: self.current_line,
            start_pos: self.current_pos,
            value: self.next()
        })
    }
}
