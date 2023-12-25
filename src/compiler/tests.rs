use crate::code::{Opcode, instructions_string};
use crate::make;

use super::*;
use crate::code::make;
use crate::compiler::Compiler;
use crate::lexer::Lexer;
use crate::parser::Parser;

struct CompilerTestCase {
    input: String,
    expected_constants: Vec<Object>,
    expected_instructions: Vec<Instructions>,
}

#[test]
fn test_integer_arithmetic() {
    let tests = vec![
        CompilerTestCase {
            input: String::from("1 + 2"),
            expected_constants: vec![Object::Integer(1), Object::Integer(2)],
            expected_instructions: vec![
                make!(OpConstant, [0]),
                make!(OpConstant, [1]),
                make!(OpAdd),
            ],
        }
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
        compiler.compile(&Node::Program(&program)).unwrap();
        let bytecode = compiler.bytecode();

        test_instructions(test.expected_instructions, bytecode.instructions);
        test_constants(test.expected_constants, bytecode.constants);
    }
}

fn test_instructions(expected: Vec<Instructions>, actual: Instructions) {
    let concatted = concat_instructions(expected);

    assert_eq!(concatted, actual, "wrong instructions: expected={:?}, actual={:?}",
               instructions_string(&concatted), instructions_string(&actual));
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


