use crate::code::{instructions_string, Opcode};
use crate::make;

use super::*;
use crate::code::make;
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;

macro_rules! make_test {
    ($input:expr; $($exp_constant:expr),*; $($exp_instruction:expr),*) => {
        CompilerTestCase {
            input: String::from($input),
            expected_constants: vec![$($exp_constant),*],
            expected_instructions: vec![$($exp_instruction),*],
        }
    };
}

struct CompilerTestCase {
    input: String,
    expected_constants: Vec<Object>,
    expected_instructions: Vec<Instructions>,
}

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        make_test!(
            "1";
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpPop)
        ),
        make_test!(
            "2";
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpPop)
        ),
        make_test!(
            "1 + 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpAdd),
            make!(OpPop)
        ),
        make_test!(
            "1; 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpPop),
            make!(OpConstant, [1]),
            make!(OpPop)
        ),
        make_test!(
            "1 - 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpSub),
            make!(OpPop)
        ),
        make_test!(
            "1 * 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpMul),
            make!(OpPop)
        ),
        make_test!(
            "4 / 2";
            Object::Integer(4),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpDiv),
            make!(OpPop)
        ),
        make_test!(
            "-1";
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpMinus),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_boolean_expressions() {
    let tests = vec![
        make_test!(
            "true";
            ;
            make!(OpTrue),
            make!(OpPop)
        ),
        make_test!(
            "false";
            ;
            make!(OpFalse),
            make!(OpPop)
        ),
        make_test!(
            "1 > 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpGreaterThan),
            make!(OpPop)
        ),
        make_test!(
            "1 < 2";
            Object::Integer(2),
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpGreaterThan),
            make!(OpPop)
        ),
        make_test!(
            "1 == 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpEqual),
            make!(OpPop)
        ),
        make_test!(
            "1 != 2";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpNotEqual),
            make!(OpPop)
        ),
        make_test!(
            "true == false";
            ;
            make!(OpTrue),
            make!(OpFalse),
            make!(OpEqual),
            make!(OpPop)
        ),
        make_test!(
            "true != false";
            ;
            make!(OpTrue),
            make!(OpFalse),
            make!(OpNotEqual),
            make!(OpPop)
        ),
        make_test!(
            "!true";
            ;
            make!(OpTrue),
            make!(OpBang),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_conditionals() {
    let tests = vec![
        make_test!(
            "if (true) { 10 }; 3333;";
            Object::Integer(10),
            Object::Integer(3333);
            make!(OpTrue),
            make!(OpJumpNotTruthy, [10]),
            make!(OpConstant, [0]),
            make!(OpJump, [11]),
            make!(OpNull),
            make!(OpPop),
            make!(OpConstant, [1]),
            make!(OpPop)
        ),
        make_test!(
            "if (true) { 10 } else { 20 }; 3333;";
            Object::Integer(10),
            Object::Integer(20),
            Object::Integer(3333);
            make!(OpTrue),
            make!(OpJumpNotTruthy, [10]),
            make!(OpConstant, [0]),
            make!(OpJump, [13]),
            make!(OpConstant, [1]),
            make!(OpPop),
            make!(OpConstant, [2]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_global_let_statements() {
    let tests = vec![
        make_test!(
            "let one = 1; let two = 2;";
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpSetGlobal, [0]),
            make!(OpConstant, [1]),
            make!(OpSetGlobal, [1])
        ),
        make_test!(
            "let one = 1; one;";
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpSetGlobal, [0]),
            make!(OpGetGlobal, [0]),
            make!(OpPop)
        ),
        make_test!(
            "let one = 1; let two = one; two;";
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpSetGlobal, [0]),
            make!(OpGetGlobal, [0]),
            make!(OpSetGlobal, [1]),
            make!(OpGetGlobal, [1]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_string_expressions() {
    let tests = vec![
        make_test!(
            "\"monkey\"";
            Object::String(String::from("monkey"));
            make!(OpConstant, [0]),
            make!(OpPop)
        ),
        make_test!(
            "\"mon\" + \"key\"";
            Object::String(String::from("mon")),
            Object::String(String::from("key"));
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpAdd),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_array_literals() {
    let tests = vec![
        make_test!(
            "[]";
            ;
            make!(OpArray, [0]),
            make!(OpPop)
        ),
        make_test!(
            "[1, 2, 3]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpArray, [3]),
            make!(OpPop)
        ),
        make_test!(
            "[1 + 2, 3 - 4, 5 * 6]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(4),
            Object::Integer(5),
            Object::Integer(6);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpAdd),
            make!(OpConstant, [2]),
            make!(OpConstant, [3]),
            make!(OpSub),
            make!(OpConstant, [4]),
            make!(OpConstant, [5]),
            make!(OpMul),
            make!(OpArray, [3]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

fn parse(input: String) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}

fn run_compiler_tests(tests: Vec<CompilerTestCase>) {
    for test in tests {
        let program = parse(test.input);
        let mut compiler = Compiler::new();
        compiler.compile(&program).unwrap();
        let bytecode = compiler.bytecode();

        test_instructions(test.expected_instructions, bytecode.instructions);
        test_constants(test.expected_constants, bytecode.constants);
    }
}

fn test_instructions(expected: Vec<Instructions>, actual: Instructions) {
    let concatted = concat_instructions(expected);

    assert_eq!(
        concatted,
        actual,
        "wrong instructions: expected={:?}, actual={:?}",
        instructions_string(&concatted),
        instructions_string(&actual)
    );
}

fn concat_instructions(instructions: Vec<Instructions>) -> Instructions {
    let mut out = Instructions::new();
    for instruction in instructions {
        out.extend(instruction);
    }
    out
}

fn test_constants(expected: Vec<Object>, actual: Vec<Object>) {
    assert_eq!(expected, actual);
}
