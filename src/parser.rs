use std::collections::VecDeque;
use std::vec;
use std::vec::Vec;

use crate::constants::Mode;
use crate::lexer::{
    Lexer,
    next_token
};
use crate::tokens::{
    Token,
    TokenType
};
use crate::syntax_tree::{
    StatementsNode,
    construct_expression_node,
    DefineBoolNode,
    DefineIntNode,
    DefineDoubleNode,
    DefineCharNode,
    DefineStringNode,
    DefineVarNode,
    DefineVariableNode,
    DefinePrintNode,
    DefineIfStatementNode,
    StatementNode,
    StatementType,
    DefineElseNode,
    DefineIfElseNode,
    DefineIfNode,
    DefineForLoopStatementNode,
    DefineContinueStatementNode,
    DefineBreakStatementNode,
};


#[derive(Debug)]
pub struct Parser{
    pub lexer: Lexer,
    pub current_token: Token,
    pub mode: Mode
}

impl Parser{
    pub fn new(lexer: Lexer, mode: Mode) -> Result<Self, String>{
        let mut lexer = lexer;
        let current_token = next_token(&mut lexer)?;

        return Ok(Parser{
            lexer,
            current_token,
            mode
        })
    }
}


pub fn parse(parser: &mut Parser) -> Result<StatementsNode, String>{
    let mut parser = parser;

    return statements(&mut parser, true);
}


pub fn statement(
    parser: &mut Parser, return_error_if_not_matched: bool
) -> Result<(bool, StatementNode), String>{

    let mut parser = parser;
    let mut node = StatementNode::new();

    if parser.current_token.token_type == TokenType::Bool{
        node.statement_type = Some(StatementType::DefineBool);

        let result = define_bool(&mut parser)?;
        node.define_bool_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::Int{
        node.statement_type = Some(StatementType::DefineInt);

        let result = define_int(&mut parser)?;
        node.define_int_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::Double{
        node.statement_type = Some(StatementType::DefineDouble);

        let result = define_double(&mut parser)?;
        node.define_double_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::Char{
        node.statement_type = Some(StatementType::DefineChar);

        let result = define_char(&mut parser)?;
        node.define_char_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::String{
        node.statement_type = Some(StatementType::DefineString);

        let result = define_string(&mut parser)?;
        node.define_string_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::Var{
        node.statement_type = Some(StatementType::DefineVar);

        let result = define_var(&mut parser)?;
        node.define_var_statement = Some(result.1);

        return Ok((result.0, node));
    }

    else if parser.current_token.token_type == TokenType::Variable{
        node.statement_type = Some(StatementType::DefineVariable);

        let result = define_variable(&mut parser)?;
        node.define_variable_statement = Some(result.1);

        return Ok((result.0, node));
    }

    else if parser.current_token.token_type == TokenType::Print{
        node.statement_type = Some(StatementType::Print);

        let result = define_print(&mut parser)?;
        node.define_print_statement = Some(result.1);

        return Ok((result.0, node));
    }

    else if parser.current_token.token_type == TokenType::If{
        node.statement_type = Some(StatementType::DefineIf);

        let result = define_if_statement(&mut parser)?;
        node.define_if_statement = Some(result.1);

        return Ok((result.0, node));
    }

    else if parser.current_token.token_type == TokenType::For{
        node.statement_type = Some(StatementType::DefineForLoop);

        let result = define_for_loop_statement(&mut parser)?;
        node.define_for_loop_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::Continue{
        node.statement_type = Some(StatementType::Continue);

        let result = define_continue_statement(&mut parser)?;
        node.define_continue_statement = Some(result.1);

        return Ok((result.0, node));
    }
    else if parser.current_token.token_type == TokenType::Break{
        node.statement_type = Some(StatementType::Break);

        let result = define_break_statement(&mut parser)?;
        node.define_break_statement = Some(result.1);

        return Ok((result.0, node));
    }

    else if
        parser.current_token.token_type == TokenType::SingleLineComment ||
        parser.current_token.token_type == TokenType::MultiLineComment ||
        parser.current_token.token_type == TokenType::Space ||
        parser.current_token.token_type == TokenType::NewLine
    {
        _move(&mut parser)?;

        node.statement_type = Some(StatementType::Discarded);

        return Ok((false, node));
    }

    else if parser.current_token.token_type == TokenType::Eof{
        return Ok((true, node));
    }

    if return_error_if_not_matched{
        return Err(format!(
            "{}: Syntax Error -> {}, line {}:{}.",
            parser.mode,
            format!("Unexpected token {:?}", parser.current_token.token_type),
            parser.current_token.start_line,
            parser.current_token.start_pos));
    }

    return Ok((true, node));
}


fn statements(
    parser: &mut Parser, return_error_if_not_matched: bool
) -> Result<StatementsNode, String>{
    let mut parser = parser;

    let mut syntax_tree = StatementsNode::new();

    loop {
        let statement_node = statement(&mut parser, return_error_if_not_matched)?;
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


fn _move(parser: &mut Parser) -> Result<(), String>{
    parser.current_token = next_token(&mut parser.lexer)?;
    return Ok(());
}


fn _match(parser: &mut Parser, types: Vec::<TokenType>) -> Result<(), String>{
    for _type in &types{
        if &parser.current_token.token_type == _type{
            return Ok(());
        }
    }

    return Err(format!(
        "{}: Syntax Error -> {}, line {}:{}.",
        parser.mode,
        format!(
            "Exepected {:?} found {:?}",
            types, parser.current_token.token_type),
        parser.current_token.start_line,
        parser.current_token.start_pos));
}


fn _is_matched_with(parser: &mut Parser, types: Vec::<TokenType>) -> bool{
    for _type in types{
        if parser.current_token.token_type == _type{
            return true;
        }
    }
    return false;
}


fn bypass(parser: &mut Parser, types: Vec::<TokenType>) -> Result<(), String>{
    let mut parser = parser;

    loop {
        if parser.current_token.token_type == TokenType::Eof{
            return Err(format!(
                "{}: Syntax Error -> {}, line {}:{}.",
                parser.mode,
                "Unexpected end of file",
                parser.current_token.start_line,
                parser.current_token.start_pos));
        }
        let mut found = false;
        for _type in &types{
            if &parser.current_token.token_type == _type{
                _move(&mut parser)?;
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


fn match_expression(
    parser: &mut Parser, allow_new_line: bool,
    tokens_array: &mut VecDeque<Token>
) -> Result<(), String>{
    let mut parser = parser;

    let mut tokens_array = tokens_array;
    if allow_new_line{
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
    }
    else{
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
    }
    _match(&mut parser, vec![
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

    if parser.current_token.token_type == TokenType::OpenParenthes{
        tokens_array.push_back(parser.current_token.clone());
        _move(&mut parser)?;

        match_expression(&mut parser, true, &mut tokens_array)?;

        _match(&mut parser, vec![
            TokenType::CloseParenthes,
        ])?;
        tokens_array.push_back(parser.current_token.clone());
        _move(&mut parser)?;

        if allow_new_line{
            bypass(&mut parser, vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
                TokenType::NewLine
            ])?;
        }
        else{
            bypass(&mut parser, vec![
                TokenType::Space,
                TokenType::MultiLineComment,
            ])?;
        }
    }
    else if parser.current_token.token_type == TokenType::Input{
        tokens_array.push_back(parser.current_token.clone());
        let default_convert_to_token = Token{
            start_line: parser.current_token.start_line,
            start_pos: parser.current_token.start_pos,
            token_type: TokenType::String,
            value: String::from("string")
        };
        _move(&mut parser)?;

        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        _match(&mut parser, vec![
            TokenType::OpenParenthes
        ])?;
        _move(&mut parser)?;

        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
        _match(&mut parser, vec![
            TokenType::CloseParenthes
        ])?;
        _move(&mut parser)?;

        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment
        ])?;
        if _is_matched_with(&mut parser, vec![
            TokenType::As
        ]){
            _move(&mut parser)?;

            bypass(&mut parser, vec![
                TokenType::Space,
                TokenType::MultiLineComment
            ])?;
            _match(&mut parser, vec![
                TokenType::Bool,
                TokenType::Int,
                TokenType::Double,
                TokenType::Char,
                TokenType::String
            ])?;
            tokens_array.push_back(parser.current_token.clone());
            _move(&mut parser)?;
        }
        else{
            tokens_array.push_back(default_convert_to_token);
        }
    }
    else{
        tokens_array.push_back(parser.current_token.clone());
        _move(&mut parser)?;
    }

    if allow_new_line{
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
    }
    else{
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
    }
    if _is_matched_with(&mut parser, vec![
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
        tokens_array.push_back(parser.current_token.clone());
        _move(&mut parser)?;

        match_expression(&mut parser, allow_new_line, &mut tokens_array)?;
    }

    return Ok(());
}


fn define_bool(parser: &mut Parser) -> Result<(bool, DefineBoolNode), String>{
    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineBoolNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![TokenType::Variable])?;
    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array));

    return Ok((false, node));
}


fn define_int(parser: &mut Parser) -> Result<(bool, DefineIntNode), String>{
    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineIntNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![TokenType::Variable])?;
    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array) );

    return Ok((false, node));
}


fn define_double(
    parser: &mut Parser
) -> Result<(bool, DefineDoubleNode), String>{

    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineDoubleNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![TokenType::Variable])?;
    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array) );

    return Ok((false, node));
}


fn define_char(parser: &mut Parser) -> Result<(bool, DefineCharNode), String>{
    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineCharNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![TokenType::Variable])?;
    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array) );

    return Ok((false, node));
}


fn define_string(
    parser: &mut Parser
) -> Result<(bool, DefineStringNode), String>{

    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineStringNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![TokenType::Variable])?;
    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array) );

    return Ok((false, node));
}


fn define_var(parser: &mut Parser) -> Result<(bool, DefineVarNode), String>{
    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineVarNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Variable,
    ])?;
    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array) );

    return Ok((false, node));
}


fn define_variable(
    parser: &mut Parser
) -> Result<(bool, DefineVariableNode), String>{

    let mut parser = parser;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    let mut node = DefineVariableNode::new();

    node.name = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::Assign,
        TokenType::PlusEqual,
        TokenType::MinusEqual,
        TokenType::MulEqual,
        TokenType::DivEqual,
        TokenType::ModEqual
    ])?;
    node.operator = Some(parser.current_token.clone());
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    match_expression(&mut parser, false, &mut tokens_array)?;

    _match(&mut parser, vec![TokenType::NewLine])?;
    _move(&mut parser)?;

    node.left = Some(construct_expression_node(&mut tokens_array));

    return Ok((false, node));
}


fn define_print(parser: &mut Parser) -> Result<(bool, DefinePrintNode), String>{
    let mut parser = parser;
    let mut node = DefinePrintNode::new();

    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::OpenParenthes,
    ])?;
    _move(&mut parser)?;

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    let mut tokens_array: VecDeque<Token> = VecDeque::new();
    match_expression(&mut parser, false, &mut tokens_array)?;
    node.expression = Some(construct_expression_node(&mut tokens_array));

    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::CloseParenthes,
    ])?;
    _move(&mut parser)?;

    return Ok((false, node));
}


fn define_if_statement(
    parser: &mut Parser
) -> Result<(bool, DefineIfStatementNode), String>{

    let mut parser = parser;
    let mut define_if_statement_node = DefineIfStatementNode::new();

    {
        /* Define If Statement */
        let mut define_if_node = DefineIfNode::new();
        define_if_node.token = Some(parser.current_token.clone());

        _move(&mut parser)?;

        /* Match Expression */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        let mut tokens_array: VecDeque<Token> = VecDeque::new();
        match_expression(&mut parser, false, &mut tokens_array)?;
        define_if_node.condition = Some(
            construct_expression_node(&mut tokens_array));

        /* Match Open Bracket */
        _match(&mut parser, vec![
            TokenType::OpenBracket
        ])?;
        _move(&mut parser)?;

        /* Match New Line */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
        ])?;
        _match(&mut parser, vec![
            TokenType::NewLine
        ])?;
        _move(&mut parser)?;

        /* Match Statements */
        define_if_node.statements = statements(&mut parser, false)?;

        /* Match Close Bracket */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine,
        ])?;
        _match(&mut parser, vec![
            TokenType::CloseBracket
        ])?;
        _move(&mut parser)?;

        /* Match New Line */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
        ])?;
        _match(&mut parser, vec![TokenType::NewLine])?;
        _move(&mut parser)?;

        define_if_statement_node.define_if_node = Some(define_if_node);
    }

    loop{
        /* Match Else Token */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine,
        ])?;
        if _is_matched_with(&mut parser, vec![
            TokenType::Else
        ]){
            _move(&mut parser)?;

            bypass(&mut parser, vec![
                TokenType::Space,
                TokenType::SingleLineComment,
                TokenType::MultiLineComment,
            ])?;

            /* Match If Token */
            if _is_matched_with(&mut parser, vec![
                TokenType::If
            ]){
                let mut define_if_else_node = DefineIfElseNode::new();
                define_if_else_node.token = Some(parser.current_token.clone());

                _move(&mut parser)?;

                /* Match Expression */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::MultiLineComment,
                ])?;
                let mut tokens_array: VecDeque<Token> = VecDeque::new();
                match_expression(&mut parser, false, &mut tokens_array)?;
                define_if_else_node.condition = Some(
                    construct_expression_node(&mut tokens_array));

                /* Match Open Bracket */
                _match(&mut parser, vec![
                    TokenType::OpenBracket
                ])?;
                _move(&mut parser)?;

                /* Match New Line */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                ])?;
                _match(&mut parser, vec![
                    TokenType::NewLine
                ])?;
                _move(&mut parser)?;

                /* Match Statements */
                define_if_else_node.statements = statements(&mut parser, false)?;

                /* Match Close Bracket */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                    TokenType::NewLine,
                ])?;
                _match(&mut parser, vec![
                    TokenType::CloseBracket
                ])?;
                _move(&mut parser)?;

                /* Match New Line */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                ])?;
                _match(&mut parser, vec![TokenType::NewLine])?;
                _move(&mut parser)?;

                define_if_statement_node.define_if_else_nodes.push_front(
                    define_if_else_node);

                continue;
            }

            /* Match Else Token */
            else{
                let mut define_else_node = DefineElseNode::new();

                /* Match Open Bracket */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                ])?;
                _match(&mut parser, vec![
                    TokenType::OpenBracket
                ])?;
                _move(&mut parser)?;

                /* Match Statements */
                define_else_node.statements = statements(&mut parser, false)?;

                /* Match Close Bracket */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                    TokenType::NewLine,
                ])?;
                _match(&mut parser, vec![
                    TokenType::CloseBracket
                ])?;
                _move(&mut parser)?;

                /* Match New Line */
                bypass(&mut parser, vec![
                    TokenType::Space,
                    TokenType::SingleLineComment,
                    TokenType::MultiLineComment,
                ])?;
                _match(&mut parser, vec![TokenType::NewLine])?;
                _move(&mut parser)?;

                define_if_statement_node.define_else_node = Some(define_else_node);
            }
        }
        break;
    }

    return Ok((false, define_if_statement_node));
}


fn define_for_loop_statement(
    parser: &mut Parser
) -> Result<(bool, DefineForLoopStatementNode), String>{

    let mut parser = parser;
    let mut for_loop_node = DefineForLoopStatementNode::new();

    _move(&mut parser)?;

    /* Define Looping Conditions */
    {
        /* Match Variable */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        _match(&mut parser, vec![
            TokenType::Variable
        ])?;
        for_loop_node.variable = Some(parser.current_token.clone());
        _move(&mut parser)?;

        /* Match In */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        _match(&mut parser, vec![
            TokenType::In
        ])?;
        _move(&mut parser)?;

        /* Match Open Pranthese */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::MultiLineComment,
        ])?;
        _match(&mut parser, vec![
            TokenType::OpenParenthes
        ])?;
        _move(&mut parser)?;

        /* Match Expression */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
        if !_is_matched_with(&mut parser, vec![
            TokenType::Comma
        ]){
            for_loop_node.meta.insert(
                String::from("start-token"),
                Some(parser.current_token.clone()));

            let mut tokens_array: VecDeque<Token> = VecDeque::new();
            match_expression(&mut parser, false, &mut tokens_array)?;
            for_loop_node.start = Some(construct_expression_node(&mut tokens_array));
        }

        /* Match Comma */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
        _match(&mut parser, vec![
            TokenType::Comma
        ])?;
        _move(&mut parser)?;

        /* Match Expression */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
        if !_is_matched_with(&mut parser, vec![
            TokenType::Comma
        ]){
            for_loop_node.meta.insert(
                String::from("stop-token"),
                Some(parser.current_token.clone()));

            let mut tokens_array: VecDeque<Token> = VecDeque::new();
            match_expression(&mut parser, false, &mut tokens_array)?;
            for_loop_node.stop = Some(construct_expression_node(&mut tokens_array));
        }

        /* Match Comma */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
        _match(&mut parser, vec![
            TokenType::Comma
        ])?;
        _move(&mut parser)?;

        /* Match Expression */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine
        ])?;
        if !_is_matched_with(&mut parser, vec![
            TokenType::CloseParenthes
        ]){
            for_loop_node.meta.insert(
                String::from("step-token"),
                Some(parser.current_token.clone()));

            let mut tokens_array: VecDeque<Token> = VecDeque::new();
            match_expression(&mut parser, false, &mut tokens_array)?;
            for_loop_node.step = Some(construct_expression_node(&mut tokens_array));
        }

        /* Match Close Pranthese */
        bypass(&mut parser, vec![
            TokenType::Space,
            TokenType::SingleLineComment,
            TokenType::MultiLineComment,
            TokenType::NewLine,
        ])?;
        _match(&mut parser, vec![
            TokenType::CloseParenthes
        ])?;
        _move(&mut parser)?;
    }

    /* Match Open Bracket */
    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::OpenBracket
    ])?;
    _move(&mut parser)?;

    /* Define Statements */
    for_loop_node.statements = statements(&mut parser, false)?;

    /* Match Close Bracket */
    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::SingleLineComment,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::CloseBracket
    ])?;
    _move(&mut parser)?;

    /* Match New Line */
    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::SingleLineComment,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::NewLine
    ])?;
    _move(&mut parser)?;

    return Ok((false, for_loop_node));
}


fn define_continue_statement(
    parser: &mut Parser
) -> Result<(bool, DefineContinueStatementNode), String>{

    let mut parser = parser;
    let mut node = DefineContinueStatementNode::new();

    node.meta.insert(
        String::from("continue-token"),
        Some(parser.current_token.clone()));

    _move(&mut parser)?;

    /* Match New Line */
    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::SingleLineComment,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::NewLine
    ])?;
    _move(&mut parser)?;

    return Ok((false, node));
}


fn define_break_statement(
    parser: &mut Parser
) -> Result<(bool, DefineBreakStatementNode), String>{

    let mut parser = parser;
    let mut node = DefineBreakStatementNode::new();

    node.meta.insert(
        String::from("break-token"),
        Some(parser.current_token.clone()));

    _move(&mut parser)?;

    /* Match New Line */
    bypass(&mut parser, vec![
        TokenType::Space,
        TokenType::SingleLineComment,
        TokenType::MultiLineComment,
    ])?;
    _match(&mut parser, vec![
        TokenType::NewLine
    ])?;
    _move(&mut parser)?;

    return Ok((false, node));
}
