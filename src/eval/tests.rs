use crate::lexer::Lexer;
use crate::parser::Parser;
use anyhow::Result;

use super::*;

fn eval_test(input: String) -> Result<Object> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program();
    let mut evaluator = Evaluator::default();

    evaluator.eval(&program)
}

#[test]
fn test_eval_int_expression() {
    let tests = vec![
        ("10", 10),
        ("5", 5),
        ("-10", -10),
        ("-5", -5),
        ("5 + 5 + 5 + 5 - 10", 10),
        ("2 * 2 * 2 * 2 * 2", 32),
        ("-50 + 100 + -50", 0),
        ("5 * 2 + 10", 20),
        ("5 + 2 * 10", 25),
        ("20 + 2 * -10", 0),
        ("50 / 2 * 2 + 10", 60),
        ("2 * (5 + 10)", 30),
        ("3 * 3 * 3 + 10", 37),
        ("3 * (3 * 3) + 10", 37),
        ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Integer(expected));
    }
}

#[test]
fn test_eval_bool_expression() {
    let tests = vec![
        ("true", true),
        ("false", false),
        ("1 < 2", true),
        ("1 > 2", false),
        ("1 < 1", false),
        ("1 > 1", false),
        ("1 == 1", true),
        ("1 != 1", false),
        ("1 == 2", false),
        ("1 != 2", true),
        ("true == true", true),
        ("false == false", true),
        ("true == false", false),
        ("true != false", true),
        ("false != true", true),
        ("(1 < 2) == true", true),
        ("(1 < 2) == false", false),
        ("(1 > 2) == true", false),
        ("(1 > 2) == false", true),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Boolean(expected));
    }
}

#[test]
fn test_eval_bang_operator() {
    let tests = vec![
        ("!true", false),
        ("!false", true),
        ("!5", false),
        ("!!true", true),
        ("!!false", false),
        ("!!5", true),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Boolean(expected));
    }
}

#[test]
fn test_if_else_expressions() {
    let tests = vec![
        ("if (true) { 10 }", "10"),
        ("if (false) { 10 }", "null"),
        ("if (1) { 10 }", "10"),
        ("if (1 < 2) { 10 }", "10"),
        ("if (1 > 2) { 10 }", "null"),
        ("if (1 > 2) { 10 } else { 20 }", "20"),
        ("if (1 < 2) { 10 } else { 20 }", "10"),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated.to_string(), expected);
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![
        ("return 10;", 10),
        ("return 10; 9;", 10),
        ("return 2 * 5; 9;", 10),
        ("9; return 2 * 5; 9;", 10),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Integer(expected));
    }
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let a = 5; a;", 5),
        ("let a = 5 * 5; a;", 25),
        ("let a = 5; let b = a; b;", 5),
        ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Integer(expected));
    }
}

#[test]
fn test_error_handling() {
    let tests = vec![
        ("5 + true;", "type mismatch: INTEGER + BOOLEAN"),
        ("5 + true; 5;", "type mismatch: INTEGER + BOOLEAN"),
        ("-true", "unknown operator: -BOOLEAN"),
        ("true + false;", "unknown operator: BOOLEAN + BOOLEAN"),
        ("5; true + false; 5", "unknown operator: BOOLEAN + BOOLEAN"),
        (
            "if (10 > 1) { true + false; }",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        (
            "
                if (10 > 1) {
                    if (10 > 1) {
                        return true + false;
                    }
                    return 1;
                }
            ",
            "unknown operator: BOOLEAN + BOOLEAN",
        ),
        ("foobar", "identifier not found: foobar"),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string());
        assert!(matches!(evaluated, Err(err) if err.to_string() == expected));
    }
}

#[test]
fn test_function_object() {
    let input = String::from("fn(x) { x + 2; };");

    let evaluated = eval_test(input).unwrap();

    println!("{}", evaluated);

    assert!(matches!(evaluated,
        Object::Function(params, body, _)
            if params == vec!["x".to_string()]
            && body.to_string() == "(x + 2)"
    ));
}

#[test]
fn test_function_application() {
    let tests = vec![
        ("let identity = fn(x) { x; }; identity(5);", 5),
        ("let identity = fn(x) { return x; }; identity(5);", 5),
        ("let double = fn(x) { x * 2; }; double(5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        ("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        ("fn(x) { x; }(5)", 5),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Integer(expected));
    }
}

#[test]
fn test_closures() {
    let input = String::from(
        "
        let newAdder = fn(x) {
            fn(y) { x + y };
        };
        let addTwo = newAdder(2);
        addTwo(2);
        ",
    );

    let evaluated = eval_test(input).unwrap();
    assert_eq!(evaluated, Object::Integer(4));
}
