use std::collections::VecDeque;
use std::vec;
use std::vec::Vec;

use crate::interpreter::lexer::Lexer;
use crate::interpreter::syntax_tree::DefineElseNode;
use crate::interpreter::syntax_tree::DefineIfElseNode;
use crate::interpreter::syntax_tree::DefineIfNode;
use crate::interpreter::syntax_tree::DefineIfStatementNode;
use crate::interpreter::syntax_tree::StatementsNode;
use crate::tokens::Token;
use crate::tokens::TokenType;
use crate::interpreter::syntax_tree::{
    construct_expression_node,
    DefineBoolNode,
    DefineIntNode,
    DefineDoubleNode,
    DefineCharNode,
    DefineStringNode,
    DefineVarNode,
    DefineVariableNode,
    DefinePrintNode,
    StatementNode,
    StatementType,
};


#[derive(Debug)]
pub struct Parser{
    lexer: Lexer,
    current_token: Token,
}

impl Parser{
    pub fn new(lexer: Lexer) -> Result<Self, String>{
        let mut lexer = lexer;
        let current_token = lexer.next_token()?;

        return Ok(Parser{
            lexer,
            current_token,
        })
    }
}

impl Parser{
    fn _move(&mut self) -> Result<(), String>{
        self.current_token = self.lexer.next_token()?;
        return Ok(());
    }

    pub fn _match(&mut self, types: Vec::<TokenType>) -> Result<(), String>{
        for _type in &types{
            if &self.current_token.token_type == _type{
                return Ok(());
            }
        }

        return Err(format!(
            "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
            format!(
                "Exepected {:?} found {:?}",
                types, self.current_token.token_type),
            self.current_token.start_line,
            self.current_token.start_pos));
    }

    pub fn _is_matched_with(&mut self, types: Vec::<TokenType>) -> bool{
        for _type in types{
            if self.current_token.token_type == _type{
                return true;
            }
        }
        return false;
    }

    pub fn bypass(&mut self, types: Vec::<TokenType>) -> Result<(), String>{
        loop {
            if self.current_token.token_type == TokenType::Eof{
                return Err(format!(
                    "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                    "Unexpected end of file",
                    self.current_token.start_line,
                    self.current_token.start_pos));
            }
            let mut found = false;
            for _type in &types{
                if &self.current_token.token_type == _type{
                    self._move()?;
                    found = true;
                    break;
                }
            }
            if found{
                continue;
            }
            break;
        }
        Ok(())
    }
}

impl Parser{
    pub fn match_expression(
        &mut self, allow_new_line: bool,
        tokens_array: &mut VecDeque<Token>
    ) -> Result<bool, String>{

        let mut tokens_array = tokens_array;
        if allow_new_line{
            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
                TokenType::NewLine
            ])?;
        }
        else{
            self.bypass(vec![
                TokenType::Space,
                TokenType::MultiLineComment,
            ])?;
        }
        self._match(vec![
            TokenType::True,
            TokenType::False,
            TokenType::IntNumber,
            TokenType::DoubleNumber,
            TokenType::Character,
            TokenType::StringSequence,
            TokenType::Variable,

            TokenType::Input,

            TokenType::OpenParenthes,
        ])?;

        if self.current_token.token_type == TokenType::OpenParenthes{
            tokens_array.push_back(self.current_token.clone());
            self._move()?;

            self.match_expression(true, &mut tokens_array)?;

            self._match(vec![
                TokenType::CloseParenthes,
            ])?;
            tokens_array.push_back(self.current_token.clone());
            self._move()?;

            if allow_new_line{
                self.bypass(vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                    TokenType::NewLine
                ])?;
            }
            else{
                self.bypass(vec![
                    TokenType::Space,
                    TokenType::MultiLineComment,
                ])?;
            }
        }
        else if self.current_token.token_type == TokenType::Input{
            tokens_array.push_back(self.current_token.clone());
            let default_convert_to_token = Token{
                start_line: self.current_token.start_line,
                start_pos: self.current_token.start_pos,
                token_type: TokenType::String,
                value: String::from("string")
            };
            self._move()?;

            self.bypass(vec![
                TokenType::Space,
                TokenType::MultiLineComment,
            ])?;
            self._match(vec![
                TokenType::OpenParenthes
            ])?;
            self._move()?;

            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
                TokenType::NewLine
            ])?;
            self._match(vec![
                TokenType::CloseParenthes
            ])?;
            self._move()?;

            self.bypass(vec![
                TokenType::Space,
                TokenType::MultiLineComment
            ])?;
            if self._is_matched_with(vec![
                TokenType::As
            ]){
                self._move()?;

                self.bypass(vec![
                    TokenType::Space,
                    TokenType::MultiLineComment
                ])?;
                self._match(vec![
                    TokenType::Bool,
                    TokenType::Int,
                    TokenType::Double,
                    TokenType::Char,
                    TokenType::String
                ])?;
                tokens_array.push_back(self.current_token.clone());
                self._move()?;
            }
            else{
                tokens_array.push_back(default_convert_to_token);
            }
        }
        else{
            tokens_array.push_back(self.current_token.clone());
            self._move()?;
        }

        if allow_new_line{
            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
                TokenType::NewLine
            ])?;
        }
        else{
            self.bypass(vec![
                TokenType::Space,
                TokenType::MultiLineComment,
            ])?;
        }
        if self._is_matched_with(vec![
            TokenType::Equal,
            TokenType::GreaterThanOrEqual,
            TokenType::GreaterThan,
            TokenType::LessThanOrEqual,
            TokenType::LessThan,
            TokenType::NotEqual,

            TokenType::Plus,
            TokenType::Minus,
            TokenType::Mul,
            TokenType::Div,
            TokenType::Mod,

            TokenType::Or,
            TokenType::And,
        ]){
            tokens_array.push_back(self.current_token.clone());
            self._move()?;

            self.match_expression(allow_new_line, &mut tokens_array)?;
        }

        return Ok(false);
    }
}

impl Parser {
    fn statements(
        &mut self, return_error_if_not_matched: bool
    ) -> Result<StatementsNode, String>{

        let mut syntax_tree = StatementsNode::new();

        loop {
            let statement_node = self.parse_statement(return_error_if_not_matched)?;
            if statement_node.0{
                break;
            }

            if statement_node.1.statement_type == Some(StatementType::Discarded){
                continue;
            }

            syntax_tree.statements.push_back(statement_node.1);
        }
        return Ok(syntax_tree);
    }

    pub fn parse_statement(
        &mut self, return_error_if_not_matched: bool
    ) -> Result<(bool, StatementNode), String>{

        let mut node = StatementNode::new();

        if self.current_token.token_type == TokenType::Bool{
            node.statement_type = Some(StatementType::DefineBool);

            let result = self.define_bool()?;
            node.define_bool_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::Int{
            node.statement_type = Some(StatementType::DefineInt);

            let result = self.define_int()?;
            node.define_int_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::Double{
            node.statement_type = Some(StatementType::DefineDouble);

            let result = self.define_double()?;
            node.define_double_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::Char{
            node.statement_type = Some(StatementType::DefineChar);

            let result = self.define_char()?;
            node.define_char_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::String{
            node.statement_type = Some(StatementType::DefineString);

            let result = self.define_string()?;
            node.define_string_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::Var{
            node.statement_type = Some(StatementType::DefineVar);

            let result = self.define_var()?;
            node.define_var_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::Variable{
            node.statement_type = Some(StatementType::DefineVariable);

            let result = self.define_variable()?;
            node.define_variable_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::Print{
            node.statement_type = Some(StatementType::Print);

            let result = self.define_print()?;
            node.define_print_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if self.current_token.token_type == TokenType::If{
            node.statement_type = Some(StatementType::DefineIf);

            let result = self.define_if_statement()?;
            node.define_if_statement = Some(result.1);

            return Ok((result.0, node));
        }
        else if
            self.current_token.token_type == TokenType::SingleLineComment ||
            self.current_token.token_type == TokenType::MultiLineComment ||
            self.current_token.token_type == TokenType::Space ||
            self.current_token.token_type == TokenType::NewLine
        {
            self._move()?;

            node.statement_type = Some(StatementType::Discarded);

            return Ok((false, node));
        }
        else if self.current_token.token_type == TokenType::Eof{
            return Ok((true, node));
        }

        if return_error_if_not_matched{
            return Err(format!(
                "Engine Interpreter: Syntax Error -> {}, line {}:{}.",
                format!("Unexpected token {:?}", self.current_token.token_type),
                self.current_token.start_line,
                self.current_token.start_pos));
        }

        return Ok((true, node));
    }

    pub fn define_bool(&mut self) -> Result<(bool, DefineBoolNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineBoolNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![TokenType::Variable])?;
        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array));

        return Ok((false, node));
    }

    pub fn define_int(&mut self) -> Result<(bool, DefineIntNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineIntNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![TokenType::Variable])?;
        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array) );

        return Ok((false, node));
    }

    pub fn define_double(&mut self) -> Result<(bool, DefineDoubleNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineDoubleNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![TokenType::Variable])?;
        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array) );

        return Ok((false, node));
    }

    pub fn define_char(&mut self) -> Result<(bool, DefineCharNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineCharNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![TokenType::Variable])?;
        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array) );

        return Ok((false, node));
    }

    pub fn define_string(&mut self) -> Result<(bool, DefineStringNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineStringNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![TokenType::Variable])?;
        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array) );

        return Ok((false, node));
    }

    pub fn define_var(&mut self) -> Result<(bool, DefineVarNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineVarNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Variable,
        ])?;
        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array) );

        return Ok((false, node));
    }

    pub fn define_variable(&mut self) -> Result<(bool, DefineVariableNode), String>{
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        let mut node = DefineVariableNode::new();

        node.name = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Assign,
            TokenType::PlusEqual,
            TokenType::MinusEqual,
            TokenType::MulEqual,
            TokenType::DivEqual,
            TokenType::ModEqual
        ])?;
        node.operator = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self.match_expression(false, &mut tokens_array)?;

        self._match(vec![TokenType::NewLine])?;
        self._move()?;

        node.left = Some(construct_expression_node(&mut tokens_array) );

        return Ok((false, node));
    }

    pub fn define_print(&mut self) -> Result<(bool, DefinePrintNode), String>{
        let mut node = DefinePrintNode::new();

        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::OpenParenthes,
        ])?;
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::Variable,
        ])?;
        node.variable = Some(self.current_token.clone());
        self._move()?;

        self.bypass(vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        self._match(vec![
            TokenType::CloseParenthes,
        ])?;
        self._move()?;

        return Ok((false, node));
    }

    pub fn define_if_statement(
        &mut self
    ) -> Result<(bool, DefineIfStatementNode), String>{

        let mut define_if_statement_node = DefineIfStatementNode::new();

        {
            /* Define If Statement */
            let mut define_if_node = DefineIfNode::new();
            define_if_node.token = Some(self.current_token.clone());

            self._move()?;

            /* Match Expression */
            self.bypass(vec![
                TokenType::Space,
                TokenType::MultiLineComment,
            ])?;
            let mut tokens_array: VecDeque<Token> = VecDeque::new();
            self.match_expression(false, &mut tokens_array)?;
            define_if_node.condition = Some(construct_expression_node(&mut tokens_array));

            /* Match Open Bracket */
            self._match(vec![
                TokenType::OpenBracket
            ])?;
            self._move()?;

            /* Match New Line */
            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
            ])?;
            self._match(vec![
                TokenType::NewLine
            ])?;
            self._move()?;

            /* Match Statements */
            define_if_node.statements = self.statements(false)?;

            /* Match Close Bracket */
            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
                TokenType::NewLine,
            ])?;
            self._match(vec![
                TokenType::CloseBracket
            ])?;
            self._move()?;

            /* Match New Line */
            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
            ])?;
            self._match(vec![TokenType::NewLine])?;
            self._move()?;

            define_if_statement_node.define_if_node = Some(define_if_node);
        }

        loop{
            /* Match Else Token */
            self.bypass(vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
                TokenType::NewLine,
            ])?;
            if self._is_matched_with(vec![
                TokenType::Else
            ]){
                self._move()?;

                self.bypass(vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                ])?;

                /* Match If Token */
                if self._is_matched_with(vec![
                    TokenType::If
                ]){
                    let mut define_if_else_node = DefineIfElseNode::new();
                    define_if_else_node.token = Some(self.current_token.clone());

                    self._move()?;

                    /* Match Expression */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::MultiLineComment,
                    ])?;
                    let mut tokens_array: VecDeque<Token> = VecDeque::new();
                    self.match_expression(false, &mut tokens_array)?;
                    define_if_else_node.condition = Some(
                        construct_expression_node(&mut tokens_array));

                    /* Match Open Bracket */
                    self._match(vec![
                        TokenType::OpenBracket
                    ])?;
                    self._move()?;

                    /* Match New Line */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::SingleLineComment,
                        TokenType::MultiLineComment,
                    ])?;
                    self._match(vec![
                        TokenType::NewLine
                    ])?;
                    self._move()?;

                    /* Match Statements */
                    define_if_else_node.statements = self.statements(false)?;

                    /* Match Close Bracket */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::SingleLineComment,
                        TokenType::MultiLineComment,
                        TokenType::NewLine,
                    ])?;
                    self._match(vec![
                        TokenType::CloseBracket
                    ])?;
                    self._move()?;

                    /* Match New Line */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::SingleLineComment,
                        TokenType::MultiLineComment,
                    ])?;
                    self._match(vec![TokenType::NewLine])?;
                    self._move()?;

                    define_if_statement_node.define_if_else_nodes.push_back(
                        define_if_else_node);

                    continue;
                }

                /* Match Else Token */
                else{
                    let mut define_else_node = DefineElseNode::new();

                    /* Match Open Bracket */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::SingleLineComment,
                        TokenType::MultiLineComment,
                    ])?;
                    self._match(vec![
                        TokenType::OpenBracket
                    ])?;
                    self._move()?;

                    /* Match Statements */
                    define_else_node.statements = self.statements(false)?;

                    /* Match Close Bracket */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::SingleLineComment,
                        TokenType::MultiLineComment,
                        TokenType::NewLine,
                    ])?;
                    self._match(vec![
                        TokenType::CloseBracket
                    ])?;
                    self._move()?;

                    /* Match New Line */
                    self.bypass(vec![
                        TokenType::Space,
                        TokenType::SingleLineComment,
                        TokenType::MultiLineComment,
                    ])?;
                    self._match(vec![TokenType::NewLine])?;
                    self._move()?;

                    define_if_statement_node.define_else_node = Some(define_else_node);
                }
            }
            break;
        }

        return Ok((false, define_if_statement_node));
    }
}
