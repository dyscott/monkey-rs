use std::collections::HashMap;

use crate::code::instructions_string;
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
            expected: Ok(Object::Integer($expected)),
        }
    };
}

macro_rules! make_test_bool {
    ($input:expr, $expected:expr) => {
        VMTestCase {
            input: String::from($input),
            expected: Ok(Object::Boolean($expected)),
        }
    };
}

macro_rules! make_test_ok {
    ($input:expr, $expected:expr) => {
        VMTestCase {
            input: String::from($input),
            expected: Ok($expected),
        }
    };
}

macro_rules! make_test_err {
    ($input:expr, $expected:expr) => {
        VMTestCase {
            input: String::from($input),
            expected: Err(anyhow!($expected)),
        }
    };
}

struct VMTestCase {
    input: String,
    expected: Result<Object>,
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
        make_test_ok!("if (1 > 2) { 10 }", Object::Null),
        make_test_ok!("if (false) { 10 }", Object::Null),
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
        make_test_ok!("\"monkey\"", Object::String(String::from("monkey"))),
        make_test_ok!("\"mon\" + \"key\"", Object::String(String::from("monkey"))),
        make_test_ok!(
            "\"mon\" + \"key\" + \"banana\"",
            Object::String(String::from("monkeybanana"))
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_array_literals() {
    let tests = vec![
        make_test_ok!("[]", Object::Array(vec![])),
        make_test_ok!(
            "[1, 2, 3]",
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
            ])
        ),
        make_test_ok!(
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
        make_test_ok!("{}", Object::Hash(HashMap::new())),
        make_test_ok!(
            "{1: 2, 2: 3}",
            Object::Hash(HashMap::from([
                (HashKey::Integer(1), Object::Integer(2)),
                (HashKey::Integer(2), Object::Integer(3)),
            ]))
        ),
        make_test_ok!(
            "{1 + 1: 2 * 2, 3 + 3: 4 * 4}",
            Object::Hash(HashMap::from([
                (HashKey::Integer(2), Object::Integer(4)),
                (HashKey::Integer(6), Object::Integer(16)),
            ]))
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_index_expressions() {
    let tests = vec![
        make_test_int!("[1, 2, 3][1]", 2),
        make_test_int!("[1, 2, 3][0 + 2]", 3),
        make_test_int!("[[1, 1, 1]][0][0]", 1),
        make_test_ok!("[][0]", Object::Null),
        make_test_ok!("[1, 2, 3][99]", Object::Null),
        make_test_int!("[1][-1]", 1),
        make_test_int!("{1: 1, 2: 2}[1]", 1),
        make_test_int!("{1: 1, 2: 2}[2]", 2),
        make_test_ok!("{1: 1}[0]", Object::Null),
        make_test_ok!("{}[0]", Object::Null),
        make_test_ok!(
            "[1, 2, 3][:]",
            Object::Array(vec![
                Object::Integer(1),
                Object::Integer(2),
                Object::Integer(3),
            ])
        ),
        make_test_ok!(
            "[1, 2, 3][1:]",
            Object::Array(vec![Object::Integer(2), Object::Integer(3)])
        ),
        make_test_ok!("[1, 2, 3][:1]", Object::Array(vec![Object::Integer(1)])),
        make_test_ok!("[1, 2, 3][-1:]", Object::Array(vec![Object::Integer(3)])),
        make_test_ok!(
            "[1, 2, 3][:-1]",
            Object::Array(vec![Object::Integer(1), Object::Integer(2)])
        ),
        make_test_ok!("[1, 2, 3][1:2]", Object::Array(vec![Object::Integer(2)])),
        make_test_ok!(
            "[1, 2, 3][1:3]",
            Object::Array(vec![Object::Integer(2), Object::Integer(3)])
        ),
        make_test_ok!(
            "[1, 2, 3][1:4]",
            Object::Array(vec![Object::Integer(2), Object::Integer(3)])
        ),
        make_test_ok!("[1, 2, 3][4:5]", Object::Array(vec![])),
        make_test_ok!("\"Hello\"[1]", Object::String("e".to_string())),
        make_test_ok!("\"Hello\"[2]", Object::String("l".to_string())),
        make_test_ok!("\"Hello\"[0]", Object::String("H".to_string())),
        make_test_ok!("\"Hello\"[3]", Object::String("l".to_string())),
        make_test_ok!("\"Hello\"[4]", Object::String("o".to_string())),
        make_test_ok!("\"Hello\"[5]", Object::Null),
        make_test_ok!("\"Hello\"[-1]", Object::String("o".to_string())),
        make_test_ok!("\"Hello\"[1:]", Object::String("ello".to_string())),
        make_test_ok!("\"Hello\"[:1]", Object::String("H".to_string())),
        make_test_ok!("\"Hello\"[-1:]", Object::String("o".to_string())),
        make_test_ok!("\"Hello\"[:-1]", Object::String("Hell".to_string())),
        make_test_ok!("\"Hello\"[:]", Object::String("Hello".to_string())),
        make_test_ok!("\"Hello\"[1:2]", Object::String("e".to_string())),
        make_test_ok!("\"Hello\"[1:3]", Object::String("el".to_string())),
        make_test_ok!("\"Hello\"[1:4]", Object::String("ell".to_string())),
        make_test_ok!("\"Hello\"[1:5]", Object::String("ello".to_string())),
        make_test_ok!("\"Hello\"[1:6]", Object::String("ello".to_string())),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_calling_functions_without_arguments() {
    let tests = vec![
        make_test_int!("let fivePlusTen = fn() { 5 + 10; }; fivePlusTen();", 15),
        make_test_int!(
            "let one = fn() { 1; }; let two = fn() { 2; }; one() + two()",
            3
        ),
        make_test_int!(
            "let a = fn() { 1 }; let b = fn() { a() + 1 }; let c = fn() { b() + 1 }; c();",
            3
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_functions_with_return_statement() {
    let tests = vec![
        make_test_int!("let earlyExit = fn() { return 99; 100; }; earlyExit();", 99),
        make_test_int!(
            "let earlyExit = fn() { return 99; return 100; }; earlyExit();",
            99
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_functions_without_return_value() {
    let tests = vec![
        make_test_ok!("let noReturn = fn() { }; noReturn();", Object::Null),
        make_test_ok!("let noReturn = fn() { }; let noReturnTwo = fn() { noReturn(); }; noReturn(); noReturnTwo();", Object::Null),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_first_class_functions() {
    let tests = vec![
        make_test_int!("let returnsOne = fn() { 1; }; let returnsOneReturner = fn() { returnsOne; }; returnsOneReturner()();", 1),
        make_test_int!("let returnsOneReturner = fn() { let returnsOne = fn() { 1; }; returnsOne; }; returnsOneReturner()();", 1),
        make_test_int!(r#"
            let returnsOneReturner = fn() {
                let returnsOne = fn() { 1; };
                returnsOne;
            };
            returnsOneReturner()();
            "#,
            1
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_calling_functions_with_bindings() {
    let tests = vec![
        make_test_int!(
            "let one = fn() { let one = 1; one }; one();",
            1
        ),
        make_test_int!(
            "let oneAndTwo = fn() { let one = 1; let two = 2; one + two; }; oneAndTwo();",
            3
        ),
        make_test_int!(r#"
            let oneAndTwo = fn() { let one = 1; let two = 2; one + two; };
            let threeAndFour = fn() { let three = 3; let four = 4; three + four; };
            oneAndTwo() + threeAndFour();
            "#,
            10
        ),
        make_test_int!(r#"
            let firstFoobar = fn() { let foobar = 50; foobar; };
            let secondFoobar = fn() { let foobar = 100; foobar; };
            firstFoobar() + secondFoobar();
            "#,
            150
        ),
        make_test_int!(r#"
            let globalSeed = 50;
            let minusOne = fn() {
            let num = 1;
            globalSeed - num;
            }
            let minusTwo = fn() {
            let num = 2;
            globalSeed - num;
            }
            minusOne() + minusTwo();
            "#,
            97
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_calling_functions_with_arguments_and_bindings() {
    let tests = vec![
        make_test_int!("let identity = fn(a) { a; }; identity(4);", 4),
        make_test_int!("let sum = fn(a, b) { a + b; }; sum(1, 2);", 3),
        make_test_int!("let sum = fn(a, b) { let c = a + b; c; }; sum(1, 2);", 3),
        make_test_int!("let sum = fn(a, b) { let c = a + b; c; }; sum(1, 2) + sum(3, 4);", 10),
        make_test_int!(r#"
            let sum = fn(a, b) {
                let c = a + b;
                c;
            };
            let outer = fn() {
                sum(1, 2) + sum(3, 4);
            };
            outer();
            "#,
            10
        ),
        make_test_int!(r#"
            let globalNum = 10;
            let sum = fn(a, b) {
                let c = a + b;
                c + globalNum;
            };
            let outer = fn() {
                sum(1, 2) + sum(3, 4) + globalNum;
            };
            outer() + globalNum;
            "#,
            50
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_calling_functions_with_wrong_arguments() {
    let tests = vec![
        make_test_err!("fn() { 1; }(1);", "wrong number of arguments: want=0, got=1"),
        make_test_err!("fn(a) { a; }();", "wrong number of arguments: want=1, got=0"),
        make_test_err!("fn(a, b) { a + b; }(1);", "wrong number of arguments: want=2, got=1"),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_builtin_functions() {
    let tests = vec![
        make_test_ok!(r#"len("")"#, Object::Integer(0)),
        make_test_ok!(r#"len("four")"#, Object::Integer(4)),
        make_test_ok!(r#"len("hello world")"#, Object::Integer(11)),
        make_test_err!(r#"len(1)"#, "argument to `len` not supported, got INTEGER"),
        make_test_err!(r#"len("one", "two")"#, "wrong number of arguments. got=2, want=1"),
        make_test_ok!(r#"len([1, 2, 3])"#, Object::Integer(3)),
        make_test_ok!(r#"len([])"#, Object::Integer(0)),
        make_test_ok!(r#"puts("hello", "world!")"#, Object::Null),
        make_test_ok!(r#"first([1, 2, 3])"#, Object::Integer(1)),
        make_test_ok!(r#"first([])"#, Object::Null),
        make_test_err!(r#"first(1)"#, "argument to `first` must be ARRAY, got INTEGER"),
        make_test_ok!(r#"last([1, 2, 3])"#, Object::Integer(3)),
        make_test_ok!(r#"last([])"#, Object::Null),
        make_test_err!(r#"last(1)"#, "argument to `last` must be ARRAY, got INTEGER"),
        make_test_ok!(r#"rest([1, 2, 3])"#, Object::Array(vec![Object::Integer(2), Object::Integer(3)])),
        make_test_ok!(r#"rest([])"#, Object::Null),
        make_test_ok!(r#"push([], 1)"#, Object::Array(vec![Object::Integer(1)])),
        make_test_err!(r#"push(1, 1)"#, "argument to `push` must be ARRAY, got INTEGER"),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_closures() {
    let tests = vec![
        make_test_int!(r#"
            let newClosure = fn(a) {
                fn() { a; };
            };
            let closure = newClosure(99);
            closure();
            "#,
            99
        ),
        make_test_int!(r#"
            let newAdder = fn(a, b) {
                fn(c) { a + b + c };
            };
            let adder = newAdder(1, 2);
            adder(8);
            "#,
            11
        ),
        make_test_int!(r#"
            let newAdder = fn(a, b) {
                let c = a + b;
                fn(d) { c + d };
            };
            let adder = newAdder(1, 2);
            adder(8);
            "#,
            11
        ),
        make_test_int!(r#"
            let newAdderOuter = fn(a, b) {
                let c = a + b;
                fn(d) {
                    let e = d + c;
                    fn(f) { e + f; };
                };
            };
            let newAdderInner = newAdderOuter(1, 2)
            let adder = newAdderInner(3);
            adder(8);
            "#,
            14
        ),
        make_test_int!(r#"
            let a = 1;
            let newAdderOuter = fn(b) {
                fn(c) {
                    fn(d) { a + b + c + d };
                };
            };
            let newAdderInner = newAdderOuter(2)
            let adder = newAdderInner(3);
            adder(8);
            "#,
            14
        ),
        make_test_int!(r#"
            let newClosure = fn(a, b) {
                let one = fn() { a; };
                let two = fn() { b; };
                fn() { one() + two(); };
            };
            let closure = newClosure(9, 90);
            closure();            
            "#,
            99
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_recursive_functions() {
    let tests = vec![
        make_test_int!(r#"
            let countDown = fn(x) {
                if (x == 0) {
                    return 0;
                } else {
                    countDown(x - 1);
                }
            };
            countDown(1);
            "#,
            0
        ),
        make_test_int!(r#"
            let countDown = fn(x) {
                if (x == 0) {
                    return 0;
                } else {
                    countDown(x - 1);
                }
            };
            let wrapper = fn() {
                countDown(1);
            };
            wrapper();
            "#,
            0
        ),
        make_test_int!(r#"
            let wrapper = fn() {
                let countDown = fn(x) {
                    if (x == 0) {
                        return 0;
                    } else {
                        countDown(x - 1);
                    }
                };
                countDown(1);
            };
            wrapper();
            "#,
            0
        ),
    ];

    run_vm_tests(tests);
}

#[test]
fn test_recursive_fibonacci() {
    let tests = vec![
        make_test_int!(r#"
            let fibonacci = fn(x) {
                if (x == 0) {
                    return 0;
                } else {
                    if (x == 1) {
                        return 1;
                    } else {
                        fibonacci(x - 1) + fibonacci(x - 2);
                    }
                }
            };
            fibonacci(15);
            "#,
            610
        )
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

        // Print disassembled bytecode
        println!("{}", instructions_string(&bytecode.instructions));

        let mut vm = VM::new(bytecode);
        match test.expected {
            Ok(expected) => {
                vm.run().unwrap();
                let stack_elem = vm.last_popped_stack_elem();
                assert_eq!(stack_elem, expected);
            }
            Err(expected) => {
                let err = vm.run().unwrap_err();
                assert_eq!(err.to_string(), expected.to_string());
            }
        }
    }
}
