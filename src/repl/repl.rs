use crate::eval::eval::{eval_return, Env, Object};
use crate::lexer::lexer::Lexer;
use crate::parser::parser::parse;
use ::std::io::Write;
use std::io;
use std::process;

use colored::*;

const PROMPT: &str = "( ᴜ ω ᴜ )⭜";

pub fn start() {
    println!("uwu *nuzzles* wewcome to uwucode! Is for me..? 🥺👉👈");
    let mut env = Env::new();
    loop {
        print!("{}  ", PROMPT.truecolor(255, 69, 0));
        std::io::stdout().flush().expect("Flushing failed");
        let mut user_in: String = String::new();
        io::stdin().read_line(&mut user_in).expect("Could not read");
        let mut lexer = Lexer::new(&user_in as &str);
        let mut token_vec = lexer.lex();

        let parsed = parse(&mut token_vec);

        let evaluated = match eval_return(parsed, &mut env) {
            Object::Terminate => {
                println!("{}", Object::Terminate);
                process::exit(69);
            }
            val => val,
        };
        println!("{}", evaluated);
    }
}
