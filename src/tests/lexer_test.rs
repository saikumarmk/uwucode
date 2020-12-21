#[cfg(test)]
mod tests {
    use crate::lexer;
    use crate::token::token::Token;
    struct TestingStruct {
        token_vec: Vec<Token>,
    }
    #[test]
    fn test_token_basic() {
        let input: &str = "=+(){},;";

        let mut test_lexer = lexer::lexer::Lexer::new(input);

        let tests: TestingStruct = TestingStruct {
            token_vec: vec![
                Token::ASSIGN,
                Token::PLUS,
                Token::LPAR,
                Token::RPAR,
                Token::LBRA,
                Token::RBRA,
                Token::COMMA,
                Token::SEMICOLON,
            ],
        };
        for test in tests.token_vec.iter() {
            let current_token = test_lexer.next_token();
            assert_eq!(test, &current_token);
        }
    }
    // Note: The structure below need not follow the syntax of uwucode.
    #[test]
    fn test_token_strings() {
        let input: &str = "owo five = 5;
        owo ten = 10;
        owo add = uwu(x,y) {
            x+y;
        };
        ";

        let mut test_lexer = lexer::lexer::Lexer::new(input);

        let tests: TestingStruct = TestingStruct {
            token_vec: vec![
                Token::LET,
                Token::IDENT(String::from("five")),
                Token::ASSIGN,
                Token::INT(5),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT(String::from("ten")),
                Token::ASSIGN,
                Token::INT(10),
                Token::SEMICOLON,
                Token::LET,
                Token::IDENT(String::from("add")),
                Token::ASSIGN,
                Token::FUNCTION,
                Token::LPAR,
                Token::IDENT(String::from("x")),
                Token::COMMA,
                Token::IDENT(String::from("y")),
                Token::RPAR,
                Token::LBRA,
                Token::IDENT(String::from("x")),
                Token::PLUS,
                Token::IDENT(String::from("y")),
                Token::SEMICOLON,
                Token::RBRA,
                Token::SEMICOLON,
            ],
        };
        for test in tests.token_vec.iter() {
            let current_token = test_lexer.next_token();
            assert_eq!(test, &current_token);
        }
    }

    #[test]
    fn test_keywords() {
        let input: &str = "owo uwu nuzzles dab rawr sugoi truwu fowose";

        let mut test_lexer = lexer::lexer::Lexer::new(input);
        let mut tests: TestingStruct = TestingStruct {
            token_vec: vec![
                Token::LET,
                Token::FUNCTION,
                Token::IF,
                Token::ELIF,
                Token::ELSE,
                Token::RETURN,
                Token::TRUE,
                Token::FALSE,
                Token::EOF,
            ],
        };
        tests.token_vec.reverse();

        for (test, token) in tests.token_vec.iter().zip(test_lexer.lex().iter()) {
            assert_eq!(test, token);
        }
    }
}
