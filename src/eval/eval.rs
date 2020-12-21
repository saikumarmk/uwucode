use crate::parser::ast::Expr;
use crate::parser::ast::Operator;
use crate::parser::ast::Prefix;
use crate::parser::ast::Statement;

pub use crate::eval::env::Env;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Null,
    Integer(i64),
    String(String),
    Boolean(bool),
    Return(Box<Object>),
    Function {
        parameters: Vec<String>,
        body: Vec<Statement>,
    },
    Terminate,
    Print(Box<Object>),
    While {
        condition: Box<Expr>,
        instruction: Vec<Statement>,
    },
}

/// NOT IMPLEMENTED: Determines what action the interpreter must take. The while loop feature is not implemented as a result, and neither is the input function.
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum InterpreterAction {
    Print(Object),
    Terminate,
    Input(Object),
    None(Object),
    While(Object),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(num) => num.fmt(f),
            Object::String(string) => string.fmt(f),
            Object::Boolean(val) => match val {
                true => "truwu".fmt(f),
                false => "fowose".fmt(f),
            },
            Object::Function {parameters:_,body:_} => "".fmt(f),
            Object::Null => "none".fmt(f),
            Object::Return(obj) => write!(f,"{}",obj),
            Object::Terminate => "Nyaaa!~, t-t-thanks fow u-using uwucode?!?1 Come *boops your nose* again *huggles tightly* soon?!! 🥺".fmt(f),
            Object::Print(obj) => write!(f,"{}",obj),
            Object::While {condition:_,instruction:_} => "".fmt(f)
        }
    }
}
// Change to be an Action
/// Evaluates most expressions recursively, with base cases being recognised primitives.
fn eval_expr(expression: Expr, env: &mut Env) -> Object {
    match expression {
        // Match primitives into their objective form
        Expr::String(string) => Object::String(string),
        Expr::Integer(num) => Object::Integer(num),
        Expr::Boolean(val) => Object::Boolean(val),

        /*
        While {statement,body} => {
            eval_expr(statement) => true,false -> break out and return blank eval
            easy
        }
        */

        /*
        Input needs to call a stdin but print first. logic needs to be handled elsewhere.

        */

        /*
        not sure how to do elif*/
        // Call prefix notation expressions into here
        Expr::Prefix { prefix, value } => eval_prefix(prefix, value, env),

        // Call infix notation here
        Expr::Infix {
            left,
            operator,
            right,
        } => eval_infix(left, operator, right, env),

        Expr::If {
            condition,
            consequence,
            alternative,
        } => {
            if eval_expr(*condition, env) == Object::Boolean(true) {
                eval_statements(consequence, env)
            } else {
                eval_statements(alternative, env)
            }
        }
        Expr::Variable(name) => env.get(&name).expect("Failed to get variable"),
        Expr::Function { parameters, body } => Object::Function { parameters, body },

        // Control flow
        Expr::While {
            condition,
            instruction,
        } => Object::While {
            condition,
            instruction,
        },
        Expr::Builtin {
            function_name,
            arguments,
        } => {
            let mut obj_args: Vec<Object> = vec![];
            for arg in arguments.iter() {
                obj_args.push(eval_expr(arg.clone(), env));
            }
            eval_builtin(function_name, obj_args)
        }

        // Call logic requires setting up function frames
        Expr::Call {
            function,
            arguments,
        } => {
            // Prep args by evaluating and appending to vector
            let mut obj_args: Vec<Object> = vec![];
            for arg in arguments.iter() {
                obj_args.push(eval_expr(arg.clone(), env));
            }

            let (parameters, body) = match *function {
                // Scope here, clean up logic
                Expr::Variable(func_name) => {
                    match env.get(&func_name) {
                        // Found a user-defined function
                        Some(Object::Function { parameters, body }) => (parameters, body),

                        // Try checking against inbuilt functions, not implemented
                        None => {
                            panic!("Function not found!");
                            //return eval_builtin(&func_name, arguments);
                        }
                        _ => panic!("attempted to call non-function"),
                    }
                }
                Expr::Function { parameters, body } => (parameters, body),

                _ => panic!("attempted to call non-function"),
            };

            // run user defined function
            eval_function(body, parameters, obj_args, env)
        }
    }
}

/// Evaluates an entire body of statements.
fn eval_statements(statements: Vec<Statement>, env: &mut Env) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement, env);

        if let &Object::Return(_) = &result {
            return result;
        }
    }
    result
}

/// Evaluates primitive statements which are either function declarations, variable definitions or return statements.
fn eval_statement(statement: Statement, env: &mut Env) -> Object {
    match statement {
        Statement::Define { func_name, func } => {
            let value = eval_expr(func, env);
            env.set(func_name, value.clone());
            value
        }

        Statement::Let { name, value } => {
            let value = eval_expr(value, env);
            env.set(name, value.clone());
            value
        }

        Statement::Expression(expr) => eval_expr(expr, env),

        Statement::Return { value } => Object::Return(Box::new(eval_expr(value, env))),
    }
}

/// Evaluates and unwraps return statements if found.
pub fn eval_return(statements: Vec<Statement>, env: &mut Env) -> Object {
    let result = eval_statements(statements, env);

    match result {
        Object::Return(ret) => *ret,
        _ => result, // Don't unwrap and deref
    }
}

/// Evaluates a single line.
pub fn eval_return_single(statement: &Statement, env: &mut Env) -> Object {
    let result = eval_statement(statement.clone(), env);

    match result {
        Object::Return(ret) => *ret,
        _ => result,
    }
}

/// Binds any arguments passed in to the function scope created, returning the scope of the function.
fn bind_local_vars(args: Vec<String>, parameters: Vec<Object>, env: &mut Env) -> Env {
    // for i in .... env set in the newest env
    assert_eq!(parameters.len(),args.len(),
        "Function called with incorrect number of arguments, {} arguments required instead {} were found.",parameters.len(),args.len()
    );

    let mut closed_env = Env::new_enclosing(env.clone());
    for (param, arg) in parameters.iter().zip(args.iter()) {
        closed_env.set(arg.clone(), param.clone());
    }
    closed_env
}

/// Sets up a function frame, binds local variables and execeutes the function.
fn eval_function(
    func_body: Vec<Statement>,
    args: Vec<String>,
    parameters: Vec<Object>,
    env: &mut Env,
) -> Object {
    let mut func_env = bind_local_vars(args, parameters, env);
    eval_return(func_body, &mut func_env)
}

/// Handles unary operations such as negation or turning a number negative.
fn eval_prefix(prefix: Prefix, value: Box<Expr>, env: &mut Env) -> Object {
    match prefix {
        // Negative numbers
        Prefix::Minus => match eval_expr(*value, env) {
            Object::Integer(val) => Object::Integer(-val),
            _ => panic!("non numeric type found, - does not support this operation"),
        },
        // Logical negation
        Prefix::Bang => match eval_expr(*value, env) {
            Object::Boolean(val) => Object::Boolean(!val),
            _ => panic!("Logical negation op performed on non boolean type"),
        },
        //_ => panic!("Prefix type not recognized"),
    }
}

/// Evaluates binary expressions via infix notation. This can include basic arithmetic or comparisons.
fn eval_infix(left: Box<Expr>, operator: Operator, right: Box<Expr>, env: &mut Env) -> Object {
    match operator {
        // Arithmetic group
        Operator::Plus
        | Operator::Minus
        | Operator::Multiply
        | Operator::Divide
        | Operator::Modulo => eval_infix_op(left, operator, right, env),
        // Comparison group
        Operator::LessThan
        | Operator::GreaterThan
        | Operator::Equals
        | Operator::LessThanEqual
        | Operator::GreaterThanEqual => eval_infix_comp(left, operator, right, env),
        _ => panic!("Operator not recognized!"),
    }
}

/// Evaluates arithmetic operations that are of the infix notation.
fn eval_infix_op(left: Box<Expr>, operator: Operator, right: Box<Expr>, env: &mut Env) -> Object {
    match operator {
        // Inner workings, eval left side and right side then check if both are numbers (or similar type)
        Operator::Plus => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left + right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::Minus => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left - right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::Multiply => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left * right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::Divide => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left / right),
            _ => panic!("Unsupported operation between two expressions"),
        },

        Operator::Modulo => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Integer(left % right),
            _ => panic!("Unsupported operation between two expressions"),
        },

        _ => panic!("Unsupported arithmetic operator found "),
    }
}

/// Evaluates comparisons that are of the infix notation.
fn eval_infix_comp(left: Box<Expr>, operator: Operator, right: Box<Expr>, env: &mut Env) -> Object {
    match operator {
        Operator::Equals => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left == right),
            (Object::String(left), Object::String(right)) => Object::Boolean(left == right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::LessThan => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left < right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::GreaterThan => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left > right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::GreaterThanEqual => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left >= right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::LessThanEqual => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left <= right),
            _ => panic!("Unsupported operation between two expressions"),
        },
        Operator::NotEquals => match (eval_expr(*left, env), eval_expr(*right, env)) {
            (Object::Integer(left), Object::Integer(right)) => Object::Boolean(left != right),
            (Object::String(left), Object::String(right)) => Object::Boolean(left != right),
            _ => panic!("Unsupported operation between two expressions"),
        },

        _ => panic!("Unsupported comparison operator encountered"),
    }
}

/// Evaluates builtins that come from the Builtin call.
///
/// # Current Builtins
/// - len (prints the length of strings)
/// - quwuit (terminates the program)
/// - dprint (prints a statement)
fn eval_builtin(func_name: String, args: Vec<Object>) -> Object {
    match &func_name as &str {
        "len" => len(args),
        "quwuit" => Object::Terminate,
        "dprint" => dprint(args),
        _ => panic!("inbuilt not found!"),
    }
}

/// BUILTIN - len
fn len(args: Vec<Object>) -> Object {
    match args.as_slice() {
        [Object::String(string)] => Object::Integer(string.len() as i64),
        _ => panic!("Non measurable type given"),
    }
}

/// BUILTIN - dprint
fn dprint(args: Vec<Object>) -> Object {
    match args.as_slice() {
        [obj] => Object::Print(Box::new(obj.clone())),
        _ => panic!("Incorrect args supplied"),
    }
}
