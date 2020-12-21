use crate::eval::eval::{eval_return_single, Env, Object};
use crate::lexer::lexer::Lexer;
use crate::parser::parser::parse;
use std::fs;
use std::process;

pub fn file_interpret(file_name: &str) {
    println!("uwu *nuzzles* wewcome to uwucode! Is for me..? 🥺👉👈");
    let mut env = Env::new();

    let file_str = match fs::read_to_string(file_name) {
        Ok(val) => val,
        Err(_) => panic!("File not found, or unreadable"),
    };

    let mut lexer = Lexer::new(&file_str);
    let mut token_vec = lexer.lex();

    let parsed = parse(&mut token_vec);
    /*
    Match action, i.e Print, Terminate, or Exit
    */
    for parsed_expr in parsed.iter() {
        match eval_return_single(parsed_expr, &mut env) {
            Object::Terminate => {
                println!("{}", Object::Terminate);
                process::exit(69);
            }
            Object::Print(value) => {
                println!("{}", value);
            }
            _ => print!(""),
        };
    }
}
