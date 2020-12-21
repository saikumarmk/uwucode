//! # What is uwucode?
//! uwucode is an atrocity. In all seriousness, uwucode is a satirical programming language that incorporates egirl slang to make it nearly unreadable. Take for example the following:
//! ```
//! owo hehexd = 1;
//! uwu nya(lel) {nuzzles (lel==hehexd) {sugoi hehexd;} rawr {sugoi lel*nya(lel-hehexd);};};
//! nya(5); // -> 120
//! ```
//! Of course, one may recognize this is a recursive implementation of the factorial function, however it looks like a mess to anyone else.

mod eval;
mod lexer;
mod parser;
mod repl;
mod tests;
mod token;
use std::env;

/// uwucode takes in the following arguments:
/// repl | open
/// # repl
/// Opens a REPl to evaluate uwucode. Takes in no additional arguments.
///
/// # open
/// Takes in one argument, which is the filepath.
fn main() {
    let args: Vec<String> = env::args().collect();
    match args[1].as_ref() {
        "repl" => repl::repl::start(),
        "open" => repl::interpreter::file_interpret(args[2].as_ref()),

        val => panic!("{} is not a recognized argument", val),
    }
}
