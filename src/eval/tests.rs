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
        ("\"Hello\" - \"World\"", "unknown operator: STRING - STRING"),
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

#[test]
fn test_string_literal() {
    let input = String::from("\"Hello World!\"");

    let evaluated = eval_test(input).unwrap();
    assert_eq!(evaluated, Object::String("Hello World!".to_string()));
}

#[test]
fn test_string_concatenation() {
    let input = String::from("\"Hello\" + \" \" + \"World!\"");

    let evaluated = eval_test(input).unwrap();
    assert_eq!(evaluated, Object::String("Hello World!".to_string()));
}

#[test]
fn test_string_comparison() {
    let tests = vec![
        ("\"Hello\" == \"Hello\"", true),
        ("\"Hello\" != \"Hello\"", false),
        ("\"Hello\" == \"World\"", false),
        ("\"Hello\" != \"World\"", true),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, Object::Boolean(expected));
    }
}

#[test]
fn test_builtin_functions() {
    let tests = vec![
        ("len(\"\")", Ok(Object::Integer(0))),
        ("len(\"four\")", Ok(Object::Integer(4))),
        ("len(\"hello world\")", Ok(Object::Integer(11))),
        (
            "len(1)",
            Err(anyhow!("argument to `len` not supported, got INTEGER")),
        ),
        (
            "len(\"one\", \"two\")",
            Err(anyhow!("wrong number of arguments. got=2, want=1")),
        ),
        ("len([1, 2, 3])", Ok(Object::Integer(3))),
        ("len([])", Ok(Object::Integer(0))),
        ("first([1, 2, 3])", Ok(Object::Integer(1))),
        ("first([])", Ok(Object::Null)),
		("first(1)", Err(anyhow!("argument to `first` must be ARRAY, got INTEGER"))),
		("last([1, 2, 3])", Ok(Object::Integer(3))),
		("last([])", Ok(Object::Null)),
		("last(1)", Err(anyhow!("argument to `last` must be ARRAY, got INTEGER"))),
		("rest([1, 2, 3])", Ok(Object::Array(vec![Object::Integer(2), Object::Integer(3)]))),
		("rest([])", Ok(Object::Null)),
		("push([], 1)", Ok(Object::Array(vec![Object::Integer(1)]))),
		("push(1, 1)", Err(anyhow!("argument to `push` must be ARRAY, got INTEGER"))),
    ];

    for (input, expected) in tests {
        println!("{}", input);
        let evaluated = eval_test(input.to_string());
        match expected {
            Ok(expected) => assert_eq!(evaluated.unwrap(), expected),
            Err(expected) => {
                assert!(matches!(evaluated, Err(err) if err.to_string() == expected.to_string()))
            }
        }
    }
}

#[test]
fn test_array_literals() {
    let input = "[1, 2 * 2, 3 + 3]";

    let evaluated = eval_test(input.to_string()).unwrap();

    assert_eq!(
        evaluated,
        Object::Array(vec![
            Object::Integer(1),
            Object::Integer(4),
            Object::Integer(6),
        ])
    );
}

#[test]
fn test_array_indexing() {
    let tests = vec![
        ("[1, 2, 3][0]", Object::Integer(1)),
        ("[1, 2, 3][1]", Object::Integer(2)),
        ("[1, 2, 3][2]", Object::Integer(3)),
        ("let i = 0; [1][i];", Object::Integer(1)),
        ("[1, 2, 3][1 + 1];", Object::Integer(3)),
        ("let myArray = [1, 2, 3]; myArray[2];", Object::Integer(3)),
        (
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            Object::Integer(6),
        ),
        (
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
            Object::Integer(2),
        ),
        ("[1, 2, 3][3]", Object::Null),
        ("[1, 2, 3][-1]", Object::Integer(3)),
        (
            "[1, 2, 3][:]",
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
            ]),
        ),
        (
            "[1, 2, 3][1:]",
            Object::Array(vec![Object::Integer(2), Object::Integer(3)]),
        ),
        ("[1, 2, 3][:1]", Object::Array(vec![Object::Integer(1)])),
        ("[1, 2, 3][-1:]", Object::Array(vec![Object::Integer(3)])),
        (
            "[1, 2, 3][:-1]",
            Object::Array(vec![Object::Integer(1), Object::Integer(2)]),
        ),
        ("[1, 2, 3][1:2]", Object::Array(vec![Object::Integer(2)])),
        (
            "[1, 2, 3][1:3]",
            Object::Array(vec![Object::Integer(2), Object::Integer(3)]),
        ),
        (
            "[1, 2, 3][1:4]",
            Object::Array(vec![Object::Integer(2), Object::Integer(3)]),
        ),
        ("[1, 2, 3][4:5]", Object::Array(vec![])),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, expected);
    }
}

#[test]
fn test_string_indexing() {
    let tests = vec![
        ("\"Hello\"[0]", Object::String("H".to_string())),
        ("\"Hello\"[1]", Object::String("e".to_string())),
        ("\"Hello\"[2]", Object::String("l".to_string())),
        ("\"Hello\"[3]", Object::String("l".to_string())),
        ("\"Hello\"[4]", Object::String("o".to_string())),
        ("\"Hello\"[5]", Object::Null),
        ("\"Hello\"[-1]", Object::String("o".to_string())),
        ("\"Hello\"[1:]", Object::String("ello".to_string())),
        ("\"Hello\"[:1]", Object::String("H".to_string())),
        ("\"Hello\"[-1:]", Object::String("o".to_string())),
        ("\"Hello\"[:-1]", Object::String("Hell".to_string())),
        ("\"Hello\"[:]", Object::String("Hello".to_string())),
        ("\"Hello\"[1:2]", Object::String("e".to_string())),
        ("\"Hello\"[1:3]", Object::String("el".to_string())),
        ("\"Hello\"[1:4]", Object::String("ell".to_string())),
        ("\"Hello\"[1:5]", Object::String("ello".to_string())),
        ("\"Hello\"[1:6]", Object::String("ello".to_string())),
    ];

    for (input, expected) in tests {
        let evaluated = eval_test(input.to_string()).unwrap();
        assert_eq!(evaluated, expected);
    }
}
