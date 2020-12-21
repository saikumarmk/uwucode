//! Handles the parsing of tokens that come from the lexer.
use crate::parser::ast::{is_builtin, Expr, Operator, Precedence, Prefix, Statement};
use crate::token::token::Token;

/// The parse function turns a vector of tokens into a vector of statements. This is done by grouping them into one of several categories.
///
/// # Parse categories
/// - Let (define)
/// - Function (define)
/// - Right brace (end body)
/// - Return (return statement)
/// - EOF (end of file)
/// - Expressions (everything else)
pub fn parse(input: &mut Vec<Token>) -> Vec<Statement> {
    let mut statements = vec![];

    // Process each statement here
    loop {
        let top_token = match input.last() {
            Some(token) => token,
            None => panic!("Vec<Token> list found empty."),
        };

        match top_token {
            Token::EOF => break, // We've reached the end of line or file
            Token::LET => parse_let(input, &mut statements), // Define a variable
            Token::FUNCTION => parse_function(input, &mut statements), // Define a function
            Token::RBRA => break, // We've reached the end of an enclosing
            Token::RETURN => parse_return(input, &mut statements), // We've hit a return statement
            _ => statements.push(Statement::Expression(parse_expression(
                input,
                Precedence::Lowest,
            ))), // Deal with an expression
        }

        // Since the subcalls modify the vector of tokens, we should reach a semicolon at the end.
        assert_eq!(input.pop(), Some(Token::SEMICOLON));
    }

    return statements;
}

/// Parses let statement by resolving an identifier name and an expression.
///
/// # Technical information
/// The function effectively transforms a line with a let statement into a let expression.
fn parse_let(input: &mut Vec<Token>, statements: &mut Vec<Statement>) {
    assert_eq!(input.pop(), Some(Token::LET)); // Sanity check

    // wtfs going on here lol, double enum extraction
    let var_name: String = match input.pop() {
        Some(Token::IDENT(var)) => var,
        _ => panic!("String type not found for variable name."),
    };

    // We're at = stage
    assert_eq!(input.pop(), Some(Token::ASSIGN));
    // Now we're at the expression eval stage, leave it to parse expr
    let value = parse_expression(input, Precedence::Lowest);
    statements.push(Statement::Let {
        name: var_name,
        value,
    });
}

fn parse_return(input: &mut Vec<Token>, statements: &mut Vec<Statement>) {
    /*
    Let's have a look at our Return enum now. Return {value:Expr} which means
    we need to just parse the expression.
    */
    assert_eq!(input.pop(), Some(Token::RETURN));
    // Now evaluate the expression
    let value = parse_expression(input, Precedence::Lowest);
    statements.push(Statement::Return { value });
}

/// Parses a function definition, which consists of statements from the other categories.
///
/// # Technical Information
/// The ideal structure of a function takes the following form:
/// DEFINE function_name(args..) {
/// body
/// }
/// This means that each argument and the body have to be individually parsed.
///
fn parse_function(input: &mut Vec<Token>, statements: &mut Vec<Statement>) {
    assert_eq!(input.pop(), Some(Token::FUNCTION));

    // Next thing is the function name, add it in
    let func_name: String = match input.pop() {
        Some(Token::IDENT(name)) => name,
        a => panic!(
            "String not found for the function name, instead found {:?}",
            a
        ),
    };

    // Now we're at args, first thing is the LPAR
    assert_eq!(input.pop(), Some(Token::LPAR));

    // Read arguments
    let mut parameters = vec![];

    loop {
        match input.pop() {
            Some(Token::RPAR) => break,
            Some(Token::IDENT(var)) => {
                // Find an arg, add then proceed
                parameters.push(var); // push to vec list
                                      // Either separate the argument, finish reading or panic.
                match input.pop() {
                    Some(Token::RPAR) => break,
                    Some(Token::COMMA) => continue,
                    _ => panic!("Object after parameter was not comma or bracket"),
                };
            }
            _ => panic!("Object after param was not comma or bracket"),
        }
    }

    // Parse the body
    assert_eq!(input.pop(), Some(Token::LBRA)); // {
    let body = parse(input); // will return code of inside
    assert_eq!(input.pop(), Some(Token::RBRA)); // }

    statements.push(Statement::Define {
        func_name,
        func: Expr::Function { parameters, body },
    });
}

/// Parses most expressions that involve primitives, operators or basic conditionals.
///
/// # Technical Information
///
fn parse_expression(input: &mut Vec<Token>, precedence: Precedence) -> Expr {
    let mut left_expr = match input.pop() {
        Some(val) => {
            match val {
                // This is a Token type
                // Primitives
                Token::INT(value) => Expr::Integer(value),
                Token::TRUE => Expr::Boolean(true),
                Token::FALSE => Expr::Boolean(false),
                Token::IDENT(value) => {
                    // for implementing builtin, do a logic check here
                    if input.last() == Some(&Token::LPAR) {
                        input.pop();
                        let mut args = vec![];

                        loop {
                            match input.last() {
                                Some(Token::RPAR) => {
                                    input.pop();
                                    break;
                                }
                                None => panic!("weird stuff happened"),
                                _ => args.push(parse_expression(input, Precedence::Lowest)),
                            }

                            match input.pop() {
                                Some(Token::RPAR) => break,
                                Some(Token::COMMA) => continue,
                                _ => panic!("Unexpected parameter"),
                            }
                        }
                        if is_builtin(&value as &str) {
                            Expr::Builtin {
                                function_name: value,
                                arguments: args,
                            }
                        } else {
                            Expr::Call {
                                function: Box::new(Expr::Variable(value)),
                                arguments: args,
                            }
                        }
                    } else {
                        Expr::Variable(value)
                    }
                }
                Token::STRING(value) => Expr::String(value),

                // Prefix types [A B]
                Token::BANG => Expr::Prefix {
                    prefix: Prefix::Bang,
                    value: Box::new(parse_expression(input, Precedence::Prefix)),
                },

                Token::MINUS => Expr::Prefix {
                    prefix: Prefix::Minus,
                    value: Box::new(parse_expression(input, Precedence::Prefix)),
                },

                // conditional
                Token::IF => {
                    assert_eq!(Some(Token::LPAR), input.pop());
                    let condition = parse_expression(input, Precedence::Lowest);
                    assert_eq!(Some(Token::RPAR), input.pop());

                    // Parse body
                    assert_eq!(Some(Token::LBRA), input.pop());
                    let consequence = parse(input);
                    assert_eq!(Some(Token::RBRA), input.pop());

                    let alternative = if input.last() == Some(&Token::ELSE) {
                        // ELSE CONDITION
                        input.pop();
                        assert_eq!(Some(Token::LBRA), input.pop());
                        let alternative = parse(input);
                        assert_eq!(Some(Token::RBRA), input.pop());
                        alternative
                    } else {
                        // No alternative
                        Vec::new()
                    };

                    Expr::If {
                        condition: Box::new(condition),
                        consequence,
                        alternative,
                    }
                }

                // while, control flow 1
                Token::WHILE => {
                    // While (THING) { do thing}
                    assert_eq!(Some(Token::LPAR), input.pop());
                    let condition = parse_expression(input, Precedence::Lowest);
                    assert_eq!(Some(Token::RPAR), input.pop());

                    // Parse body
                    assert_eq!(Some(Token::LBRA), input.pop());
                    let instruction = parse(input);
                    assert_eq!(Some(Token::RBRA), input.pop());

                    Expr::While {
                        condition: Box::new(condition),
                        instruction,
                    }
                }

                // Error
                _ => panic!("Parser error: Not recognized!"),
            }
        }
        _ => panic!("The vector is empty"),
    };

    // Depending on whether we have a prefix/infix expression, we need to modify evaluation order.
    while precedence < input.last().unwrap().priority() {
        left_expr = parse_infix(left_expr, input);
    }

    left_expr
}

/// Parses expressions involving an operator in the middle, for instance a OP b.
///
/// # Technical Information
/// The left token is passed in, then the right token is popped, which results in a new infix expression, with pointers to the left and right expressions.
fn parse_infix(left: Expr, input: &mut Vec<Token>) -> Expr {
    let next_token = match input.pop() {
        Some(value) => value,
        None => panic!("Empty list..."),
    };

    let operator = match next_token {
        Token::PLUS => Operator::Plus,
        Token::MINUS => Operator::Minus,
        Token::SLASH => Operator::Divide,
        Token::MOD => Operator::Modulo,
        Token::ASTERISK => Operator::Multiply,
        Token::LEQ => Operator::LessThanEqual,
        Token::LE => Operator::LessThan,
        Token::GEQ => Operator::GreaterThanEqual,
        Token::GR => Operator::GreaterThan,
        Token::EQ => Operator::Equals,
        Token::NEQ => Operator::NotEquals,
        _ => panic!("parse infix called on invalid operator"),
    };

    Expr::Infix {
        left: Box::new(left),
        operator,
        right: Box::new(parse_expression(input, next_token.priority())),
    }
}
