use std::io::Write;

use crate::shell;
use crate::parsers;
use crate::types::Tokens;
use crate::scripting;


pub fn run(sh: &mut shell::Shell, tokens: &Tokens) -> i32 {
    let args = parsers::parser_line::tokens_to_args(&tokens);

    if args.len() > 2 {
        println_stderr!("cicada: source: too many arguments");
        return 1;
    }
    if args.len() < 2 {
        println_stderr!("cicada: source: no file specified");
        return 1;
    }

    return scripting::run_script(sh, &args);
}
