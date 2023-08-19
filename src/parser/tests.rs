use super::*;
use crate::lexer::Lexer;
use crate::parser::ast::Statement;

fn setup_test(input: String, expected_statements: Option<usize>) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);

    let program = parser.parse_program();

    // Print parser errors and panic if any
    if !parser.errors.is_empty() {
        println!("parser errors: {:?}", parser.errors);
        panic!("parser has {} errors", parser.errors.len());
    }

    // Expect expected_statements statements
    if let Some(expected_statements) = expected_statements {
        assert_eq!(program.statements.len(), expected_statements);
    }

    program
}

#[test]
fn test_let_statements() {
    let tests = vec![
        ("let x = 5;", "x", "5"),
        ("let y = true;", "y", "true"),
        ("let foobar = y;", "foobar", "y"),
    ];

    for (input, name, value) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];

        assert!(matches!(stmt,
            Statement::Let(
                var_name,
                expr
            ) if var_name == name && expr.to_string() == value
        ));
    }
}

#[test]
fn test_return_statements() {
    let tests = vec![
        ("return 5;", "5"),
        ("return true;", "true"),
        ("return foobar;", "foobar"),
    ];

    for (input, value) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];

        assert!(matches!(stmt,
            Statement::Return(
                expr
            ) if expr.to_string() == value
        ));
    }
}

#[test]
fn test_parser_string() {
    let program = Program {
        statements: vec![Statement::Let(
            String::from("myVar"),
            Expression::Identifier(String::from("anotherVar")),
        )],
    };

    assert_eq!(program.to_string(), "let myVar = anotherVar;");
}

#[test]
fn test_identifier_expression() {
    let input = String::from("foobar;");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];

    assert!(matches!(stmt,
        Statement::Expression(
            Expression::Identifier(value)
        ) if value == "foobar"
    ));
}

#[test]
fn test_integer_literal_expression() {
    let input = String::from("5;");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];

    assert!(matches!(stmt,
        Statement::Expression(
            Expression::Integer(value)
        ) if *value == 5
    ));
}

#[test]
fn test_prefix_expressions() {
    let tests = vec![
        ("!5;", token!(!), "5"),
        ("-15;", token!(-), "15"),
        ("!foobar;", token!(!), "foobar"),
        ("-foobar;", token!(-), "foobar"),
        ("!true;", token!(!), "true"),
        ("!false;", token!(!), "false"),
    ];

    for (input, operator, value) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];
        assert!(matches!(stmt,
            Statement::Expression(
                Expression::Prefix(op, expr)
            ) if *op == operator && expr.to_string() == value
        ));
    }
}

#[test]
fn test_infix_expressions() {
    let tests = vec![
        ("5 + 5;", "5", token!(+), "5"),
        ("5 - 5;", "5", token!(-), "5"),
        ("5 * 5;", "5", token!(*), "5"),
        ("5 / 5;", "5", token!(/), "5"),
        ("5 > 5;", "5", token!(>), "5"),
        ("5 < 5;", "5", token!(<), "5"),
        ("5 == 5;", "5", token!(==), "5"),
        ("5 != 5;", "5", token!(!=), "5"),
        ("foobar + barfoo;", "foobar", token!(+), "barfoo"),
        ("foobar - barfoo;", "foobar", token!(-), "barfoo"),
        ("foobar * barfoo;", "foobar", token!(*), "barfoo"),
        ("foobar / barfoo;", "foobar", token!(/), "barfoo"),
        ("foobar > barfoo;", "foobar", token!(>), "barfoo"),
        ("foobar < barfoo;", "foobar", token!(<), "barfoo"),
        ("foobar == barfoo;", "foobar", token!(==), "barfoo"),
        ("foobar != barfoo;", "foobar", token!(!=), "barfoo"),
        ("true == true", "true", token!(==), "true"),
        ("true != false", "true", token!(!=), "false"),
        ("false == false", "false", token!(==), "false"),
    ];

    for (input, left, operator, right) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];
        assert!(matches!(stmt,
            Statement::Expression(
                Expression::Infix(op, left_expr, right_expr)
            ) if *op == operator &&
                left_expr.to_string() == left &&
                right_expr.to_string() == right
        ));
    }
}

#[test]
fn test_operator_precedence() {
    let tests = vec![
        ("-a * b", "((-a) * b)"),
        ("!-a", "(!(-a))"),
        ("a + b + c", "((a + b) + c)"),
        ("a + b - c", "((a + b) - c)"),
        ("a * b * c", "((a * b) * c)"),
        ("a * b / c", "((a * b) / c)"),
        ("a + b / c", "(a + (b / c))"),
        ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
        ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
        ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
        ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        (
            "3 + 4 * 5 == 3 * 1 + 4 * 5",
            "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
        ),
        ("true", "true"),
        ("false", "false"),
        ("3 > 5 == false", "((3 > 5) == false)"),
        ("3 < 5 == true", "((3 < 5) == true)"),
        ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
        ("(5 + 5) * 2", "((5 + 5) * 2)"),
        ("2 / (5 + 5)", "(2 / (5 + 5))"),
        ("(5 + 5) * 2 * (5 + 5)", "(((5 + 5) * 2) * (5 + 5))"),
        ("-(5 + 5)", "(-(5 + 5))"),
        ("!(true == true)", "(!(true == true))"),
        ("a + add(b * c) + d", "((a + add((b * c))) + d)"),
        (
            "add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))",
            "add(a, b, 1, (2 * 3), (4 + 5), add(6, (7 * 8)))",
        ),
        (
            "add(a + b + c * d / f + g)",
            "add((((a + b) + ((c * d) / f)) + g))",
        ),
        (
            "a * [1, 2, 3, 4][b * c] * d",
            "((a * ([1, 2, 3, 4][(b * c)])) * d)",
        ),
        (
            "add(a * b[2], b[1], 2 * [1, 2][1])",
            "add((a * (b[2])), (b[1]), (2 * ([1, 2][1])))",
        ),
    ];

    for (input, expected) in tests {
        let program = setup_test(String::from(input), None);

        assert_eq!(program.to_string(), expected);
    }
}

#[test]
fn test_boolean_expression() {
    let tests = vec![("true;", true), ("false;", false)];

    for (input, value) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];
        assert!(matches!(stmt,
            Statement::Expression(
                Expression::Boolean(val)
            ) if *val == value
        ));
    }
}

#[test]
fn test_if_expression() {
    let input = String::from("if (x < y) { x }");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::If(
                condition,
                consequence,
                None
            )
        ) if condition.to_string() == "(x < y)" &&
            consequence.to_string() == "x"
    ));
}

#[test]
fn test_if_else_expression() {
    let input = String::from("if (x < y) { x } else { y }");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::If(
                condition,
                consequence,
                Some(alternative)
            )
        ) if condition.to_string() == "(x < y)" &&
            consequence.to_string() == "x" &&
            alternative.to_string() == "y"
    ));
}

#[test]
fn test_fn_literal_parsing() {
    let input = String::from("fn(x, y) { x + y; }");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::Function(
                parameters,
                body
            )
        ) if parameters.len() == 2 &&
            parameters[0].to_string() == "x" &&
            parameters[1].to_string() == "y" &&
            matches!(&**body,
                Statement::Block(
                    statements
                ) if statements.len() == 1 &&
                    statements[0].to_string() == "(x + y)"
            )
    ));
}

#[test]
fn test_fn_parameter_parsing() {
    let tests = vec![
        ("fn() {};", vec![]),
        ("fn(x) {};", vec!["x"]),
        ("fn(x, y, z) {};", vec!["x", "y", "z"]),
    ];

    for (input, args) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];
        assert!(matches!(stmt,
            Statement::Expression(
                Expression::Function(
                    parameters,
                    _
                )
            ) if parameters.len() == args.len() &&
                parameters.iter().zip(args.iter()).all(|(a, b)| a.to_string() == *b)
        ));
    }
}

#[test]
fn test_call_expression_parsing() {
    let input = String::from("add(1, 2 * 3, 4 + 5);");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::Call(
                function,
                arguments
            )
        ) if function.to_string() == "add" &&
            arguments.len() == 3 &&
            arguments[0].to_string() == "1" &&
            arguments[1].to_string() == "(2 * 3)" &&
            arguments[2].to_string() == "(4 + 5)"
    ));
}

#[test]
fn test_call_expression_parameter_parsing() {
    let tests = vec![
        ("add();", vec![]),
        ("add(1);", vec!["1"]),
        ("add(1, 2 * 3, 4 + 5);", vec!["1", "(2 * 3)", "(4 + 5)"]),
    ];

    for (input, args) in tests {
        let program = setup_test(String::from(input), Some(1));

        let stmt = &program.statements[0];
        assert!(matches!(stmt,
            Statement::Expression(
                Expression::Call(
                    _,
                    arguments
                )
            ) if arguments.len() == args.len() &&
                arguments.iter().zip(args.iter()).all(|(a, b)| a.to_string() == *b)
        ));
    }
}

#[test]
fn test_string_literal_expression() {
    let input = String::from("\"hello world\";");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::String(value)
        ) if value == "hello world"
    ));
}

#[test]
fn test_array_literal_expression() {
    let input = String::from("[1, 2 * 2, 3 + 3]");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::Array(values)
        ) if values.len() == 3 &&
            values[0].to_string() == "1" &&
            values[1].to_string() == "(2 * 2)" &&
            values[2].to_string() == "(3 + 3)"
    ));
}

#[test]
fn test_array_index_expression() {
    let input = String::from("myArray[1 + 1]");

    let program = setup_test(input, Some(1));

    let stmt = &program.statements[0];
    assert!(matches!(stmt,
        Statement::Expression(
            Expression::Index(
                array,
                index
            )
        ) if array.to_string() == "myArray" &&
            index.to_string() == "(1 + 1)"
    ));
}
