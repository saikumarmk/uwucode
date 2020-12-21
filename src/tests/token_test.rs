#[cfg(test)]
mod tests {
    use crate::token::token::*;

    #[test]
    fn test_lookup_ident() {
        struct TestingStruct {
            token_vec: Vec<Token>,
        }
        let inputs: Vec<&str> = vec![
            "uwu", "owo", "nuzzles", "dab", "rawr", "sugoi", "truwu", "fowose", "Hello!",
        ];

        let tests: TestingStruct = TestingStruct {
            token_vec: vec![
                Token::FUNCTION,
                Token::LET,
                Token::IF,
                Token::ELIF,
                Token::ELSE,
                Token::RETURN,
                Token::TRUE,
                Token::FALSE,
                Token::IDENT(String::from("Hello!")),
            ],
        };
        // You may ask, what the hell is the resultant iterator here?? Tbh, I'm not sure as well...
        // In all seriousness,unlike Python, zip is a method for tying iterators, where the argument (not self!) is the second element.
        // Diagram: A.iter().zip(B.iter()) -> iter(a,b).
        for (input, test) in inputs.iter().zip(tests.token_vec.iter()) {
            assert_eq!(&lookup_ident(input), test);
        }
    }
}
