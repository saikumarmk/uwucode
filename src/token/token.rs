///
/// Maps an identifier to a keyword or a variable/function.
/// # Examples
///
/// Various keywords are mapped to a respective action.
/// ```
/// let result = lookup_ident("uwu"); // Returns Token::FUNCTION
/// assert_eq!(Token::FUNCTION,result);
/// ```
///
/// If the keyword is not recognized, it gets mapped to an identifier.
/// ```
/// let result = lookup_ident("random_thing");
/// assert_eq!(Token::IDENT(String::from("random_thing")),result);
/// ```
///
///
pub fn lookup_ident(ident: &str) -> Token {
    match ident {
        "uwu" => Token::FUNCTION,
        "owo" => Token::LET,
        "nuzzles" => Token::IF,
        "dab" => Token::ELIF,
        "rawr" => Token::ELSE,
        "sugoi" => Token::RETURN,
        "truwu" => Token::TRUE,
        "fowose" => Token::FALSE,
        "nyaa" => Token::WHILE,
        _ => Token::IDENT(String::from(ident)),
    }
}

/// The Token enum is used in the parsing process. It could be called the atomic element of a programming language.
///
#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    ILLEGAL(String),
    EOF,

    IDENT(String),

    ASSIGN,
    PLUS,
    MINUS,
    ASTERISK,
    SLASH,
    MOD,

    EQ,
    LEQ,
    LE,
    GEQ,
    GR,
    NEQ,

    COMMA,
    SEMICOLON,

    LPAR,
    RPAR,
    LBRA,
    RBRA,

    BANG,

    FUNCTION,
    LET,
    RETURN,

    IF,
    ELIF,
    ELSE,

    WHILE,

    INT(i64),
    STRING(String),
    TRUE,
    FALSE,
}
