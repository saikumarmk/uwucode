use crate::token::token::{lookup_ident, Token};
use ::std::iter::Peekable;
use std::str;
use std::str::Chars;

pub struct Lexer<'a> {
    /*
    Takes in a string then stores its iterator.
    Info: <'a> indicates a speciifed lifetime.
    */
    pub chr_iter: Peekable<Chars<'a>>,
}

/// Initializes an instance of a lexer which returns a vector of tokens on a string.
///
/// # Examples
///
/// Initialize a lexer as follows:
/// ```
/// let mut lexer = Lexer::new("owo five = 5;");
/// ```
impl<'a> Lexer<'a> {
    /// Calls next on the char iterator.
    pub fn read_char(&mut self) -> Option<char> {
        // Returns the next item in the iterator.
        self.chr_iter.next()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.chr_iter.peek()
    }

    /// Instantiates a new lexer instance with a char iterator.
    pub fn new(file_string: &'a str) -> Lexer<'a> {
        Lexer {
            chr_iter: file_string.chars().peekable(),
        }
    }

    /// Consumes all whitespace characters by peaking, making the language independent of whitespace.
    pub fn skip_whitespace(&mut self) {
        // While not perfect, it's better to use Some over unwrap to avoid None issues.
        while let Some(&chr) = self.peek_char() {
            if chr.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    /// Reads in keywords, variables and function names.
    pub fn read_identifier(&mut self, first_letter: char) -> String {
        let mut expression: String = String::from(first_letter);

        // Peek at top element, determine if alphabetic then add.
        while let Some(&chr) = self.peek_char() {
            if chr.is_alphabetic() || chr == '_' {
                expression.push(self.read_char().unwrap());
            } else {
                break;
            }
        }
        expression
    }

    /// Reads in a string which has a specific terminator.
    pub fn read_string(&mut self) -> String {
        let mut expression: String = String::new();

        // Peek at top element, determine if alphabetic then add.
        while let Some(&chr) = self.peek_char() {
            if chr == '"' {
                self.read_char();
                break;
            } else {
                expression.push(self.read_char().unwrap());
            }
        }
        expression
    }

    /// Reads in a sequence of integers and returns an integer.
    pub fn read_number(&mut self, first_chr: char) -> i64 {
        // TODO: Prefix notation, i.e 0x, 0b, 0o, 0f
        let mut expression: String = String::from(first_chr);

        while let Some(&chr) = self.peek_char() {
            if char::is_digit(chr, 10) {
                expression.push(self.read_char().unwrap());
            } else {
                break;
            }
        }

        expression.parse().unwrap()
    }

    /// Reads from the iterator to create the next token.
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        /*
        Matching process:
        For single character tokens, immediately process.
        However for multi-leter tokens, use peak to determine if the next character links up.
        */
        match self.read_char() {
            // = first, into ASSIGN and EQUAL
            Some('=') => {
                // Comparison operator
                if self.peek_char().unwrap() == &'=' {
                    self.read_char();
                    Token::EQ
                } else {
                    Token::ASSIGN
                }
            }
            Some(',') => Token::COMMA,
            Some(';') => Token::SEMICOLON,

            // Alternative (or soon to be default) line end :3
            Some(':') => {
                if self.peek_char().unwrap() == &'3' {
                    self.read_char();
                    Token::SEMICOLON
                } else {
                    Token::ILLEGAL(String::from(":"))
                }
            }

            Some('+') => Token::PLUS,
            Some('-') => Token::MINUS,
            Some('%') => Token::MOD,

            // TODO: implement power
            Some('*') => Token::ASTERISK,
            // TODO: implement integer division, comments
            Some('/') => {
                if self.peek_char().unwrap() == &'*' {
                    // comments
                    self.read_char();
                    loop {
                        if self.peek_char().unwrap() == &'*' {
                            self.read_char();
                            if self.peek_char().unwrap() == &'/' {
                                self.read_char();
                                return self.next_token();
                            }
                        }
                        self.read_char();
                    }
                } else {
                    Token::SLASH
                }
            }

            Some('(') => Token::LPAR,
            Some(')') => Token::RPAR,
            Some('{') => Token::LBRA,
            Some('}') => Token::RBRA,

            Some('>') => {
                if self.peek_char().unwrap() == &'=' {
                    self.read_char();
                    Token::GEQ
                } else {
                    Token::GR
                }
            }

            Some('<') => {
                if self.peek_char().unwrap() == &'=' {
                    self.read_char();
                    Token::LEQ
                } else {
                    Token::LE
                }
            }

            Some('!') => {
                if self.peek_char().unwrap() == &'=' {
                    self.read_char();
                    Token::NEQ
                } else {
                    Token::BANG
                }
            }

            None => Token::EOF,

            Some('"') => Token::STRING(self.read_string()),

            // Deal with expressions and primitives
            Some(chr) => {
                if chr.is_alphabetic() {
                    // Converts to either keyword or identifier
                    let token = self.read_identifier(chr);
                    lookup_ident(&token as &str)
                }
                // Could be an integer
                else if char::is_digit(chr, 10) {
                    Token::INT(self.read_number(chr))
                }
                // Nothing recognized, spit illegal.
                else {
                    Token::ILLEGAL(String::from(chr))
                }
            }
        }
    }

    /// Returns the vector of tokens from an input.
    pub fn lex(&mut self) -> Vec<Token> {
        let mut token_vec: Vec<Token> = Vec::new();

        loop {
            match self.chr_iter.peek() {
                None => break,
                _ => token_vec.push(self.next_token()),
            }
        }
        if token_vec.last() != Some(&Token::EOF) {
            token_vec.push(Token::EOF);
        }
        // Important, I'm reversing the list because implementation from the back is much better in terms of time complexity. Pop better than remove.
        // Potential TODO: Change to stack?
        token_vec.reverse();
        token_vec
    }
}
