mod constants;
mod compiler;

use clap::{Arg, Command, ArgMatches};

use crate::compiler::compile;


fn parse_args() -> ArgMatches{
    let file_path_arg = Arg::new("file-path");

    // let executable_arg = Arg::new("executable")
    //     .long("executable")
    //     .short('e')
    //     .value_parser(clap::value_parser!(bool))
    //     .default_value("false")
    //     .default_missing_value("true")
    //     .num_args(0)
    //     .required(false)
    //     .conflicts_with("byte-code");

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
        // executable_arg,
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

    let result = compile(generate_byte_code);
    if result.is_err(){
        panic!("{}", result.unwrap_err());
    }
}
