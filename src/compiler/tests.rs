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
        )
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
