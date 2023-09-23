mod constants;
mod compiler;
mod interpreter;

use clap::{Arg, Command, ArgMatches};

use crate::compiler::compile;
use crate::interpreter::interpret;


fn parse_args() -> ArgMatches{
    let file_path_arg = Arg::new("file-path");

    let executable_arg = Arg::new("executable")
        .long("executable")
        .short('e')
        .value_parser(clap::value_parser!(bool))
        .default_value("false")
        .default_missing_value("true")
        .num_args(0)
        .required(false)
        .conflicts_with("byte-code");

    let byte_code_arg = Arg::new("byte-code")
        .long("byte-code")
        .short('b')
        .value_parser(clap::value_parser!(bool))
        .default_value("false")
        .default_missing_value("true")
        .num_args(0)
        .required(false);

    let args = Command::new("Engine").args([
        file_path_arg,
        executable_arg,
        byte_code_arg
    ])
        .about("Engine Programming Language.")
        .long_about("Engine Programming Language Ecosystem.");
    return args.get_matches();
}


fn main() {
    let matches = parse_args();

    let generate_byte_code = if
        matches.contains_id("byte-code") &&
        matches.get_one::<bool>("byte-code").unwrap().clone()
    {true} else {false};

    let executable = if
        matches.contains_id("executable") &&
        matches.get_one::<bool>("executable").unwrap().clone()
    {true} else {false};

    let result = if executable || generate_byte_code{compile(generate_byte_code)} else {interpret()};
    if result.is_err(){
        panic!("{}", result.unwrap_err());
    }
}
