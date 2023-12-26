use std::collections::HashMap;

use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::object::HashKey;
use crate::parser::ast::{Node, Program};
use crate::parser::Parser;

use super::*;

macro_rules! make_test_int {
    ($input:expr, $expected:expr) => {
        VMTestCase {
            input: String::from($input),
            expected: Object::Integer($expected),
        }
    };
}

macro_rules! make_test_bool {
    ($input:expr, $expected:expr) => {
        VMTestCase {
            input: String::from($input),
            expected: Object::Boolean($expected),
        }
    };
}

macro_rules! make_test {
    ($input:expr, $expected:expr) => {
        VMTestCase {
            input: String::from($input),
            expected: $expected,
        }
    };
}

struct VMTestCase {
    input: String,
    expected: Object,
}

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        make_test_int!("1", 1),
        make_test_int!("2", 2),
        make_test_int!("1 + 2", 3),
        make_test_int!("1 - 2", -1),
        make_test_int!("1 * 2", 2),
        make_test_int!("4 / 2", 2),
        make_test_int!("50 / 2 * 2 + 10 - 5", 55),
        make_test_int!("5 + 5 + 5 + 5 - 10", 10),
        make_test_int!("2 * 2 * 2 * 2 * 2", 32),
        make_test_int!("5 * 2 + 10", 20),
        make_test_int!("5 + 2 * 10", 25),
        make_test_int!("5 * (2 + 10)", 60),
        make_test_int!("-5", -5),
        make_test_int!("-10", -10),
        make_test_int!("-50 + 100 + -50", 0),
        make_test_int!("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_boolean_expressions() {
    let tests = vec![
        make_test_bool!("true", true),
        make_test_bool!("false", false),
        make_test_bool!("1 < 2", true),
        make_test_bool!("1 > 2", false),
        make_test_bool!("1 < 1", false),
        make_test_bool!("1 > 1", false),
        make_test_bool!("1 == 1", true),
        make_test_bool!("1 != 1", false),
        make_test_bool!("1 == 2", false),
        make_test_bool!("1 != 2", true),
        make_test_bool!("true == true", true),
        make_test_bool!("false == false", true),
        make_test_bool!("true == false", false),
        make_test_bool!("true != false", true),
        make_test_bool!("false != true", true),
        make_test_bool!("(1 < 2) == true", true),
        make_test_bool!("(1 < 2) == false", false),
        make_test_bool!("(1 > 2) == true", false),
        make_test_bool!("(1 > 2) == false", true),
        make_test_bool!("!true", false),
        make_test_bool!("!false", true),
        make_test_bool!("!5", false),
        make_test_bool!("!!true", true),
        make_test_bool!("!!false", false),
        make_test_bool!("!!5", true),
        make_test_bool!("!(if (false) { 5; })", true),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![
        make_test_int!("if (true) { 10 }", 10),
        make_test_int!("if (true) { 10 } else { 20 }", 10),
        make_test_int!("if (false) { 10 } else { 20 }", 20),
        make_test_int!("if (1) { 10 }", 10),
        make_test_int!("if (1 < 2) { 10 }", 10),
        make_test_int!("if (1 < 2) { 10 } else { 20 }", 10),
        make_test_int!("if (1 > 2) { 10 } else { 20 }", 20),
        make_test!("if (1 > 2) { 10 }", Object::Null),
        make_test!("if (false) { 10 }", Object::Null),
        make_test_int!("if ((if (false) { 10 })) { 10 } else { 20 }", 20),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_global_let_statements() {
    let tests = vec![
        make_test_int!("let one = 1; one", 1),
        make_test_int!("let one = 1; let two = 2; one + two", 3),
        make_test_int!("let one = 1; let two = one + one; one + two", 3),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_string_expressions() {
    let tests = vec![
        make_test!("\"monkey\"", Object::String(String::from("monkey"))),
        make_test!("\"mon\" + \"key\"", Object::String(String::from("monkey"))),
        make_test!(
            "\"mon\" + \"key\" + \"banana\"",
            Object::String(String::from("monkeybanana"))
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_array_literals() {
    let tests = vec![
        make_test!("[]", Object::Array(vec![])),
        make_test!(
            "[1, 2, 3]",
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
            ])
        ),
        make_test!(
            "[1 + 2, 3 * 4, 5 + 6]",
            Object::Array(vec![
                Object::Integer(3),
                Object::Integer(12),
                Object::Integer(11),
            ])
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_hash_literals() {
    let tests = vec![
        make_test!("{}", Object::Hash(HashMap::new())),
        make_test!(
            "{1: 2, 2: 3}",
            Object::Hash(HashMap::from([
                (HashKey::Integer(1), Object::Integer(2)),
                (HashKey::Integer(2), Object::Integer(3)),
            ]))
        ),
        make_test!(
            "{1 + 1: 2 * 2, 3 + 3: 4 * 4}",
            Object::Hash(
                HashMap::from([
                    (HashKey::Integer(2), Object::Integer(4)),
                    (HashKey::Integer(6), Object::Integer(16)),
                ])
            )
        ),
    ];

    run_vm_tests(tests);
}

fn parse(input: String) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

fn run_vm_tests(tests: Vec<VMTestCase>) {
    for test in tests {
        let program = parse(test.input);
        let mut compiler = Compiler::new();
        compiler.compile_node(&Node::Program(&program)).unwrap();
        let bytecode = compiler.bytecode();

        let mut vm = VM::new(bytecode);
        vm.run().unwrap();

        let stack_elem = vm.last_popped_stack_elem();

        assert_eq!(stack_elem, test.expected);
    }
}
