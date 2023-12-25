use crate::compiler::Compiler;
use crate::parser::ast::{Node, Program};
use crate::lexer::Lexer;
use crate::parser::Parser;

use super::*;

fn parse(input: String) -> Program {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_program()
}


struct VMTestCase {
    input: String,
    expected: Object,
}

fn run_vm_tests(tests: Vec<VMTestCase>) {
    for test in tests {
        let program = parse(test.input);
        let mut compiler = Compiler::new();
        compiler.compile(&Node::Program(&program)).unwrap();
        let bytecode = compiler.bytecode();

        let mut vm = VM::new(bytecode);
        vm.run().unwrap();

        let stack_elem = vm.stack_top();

        assert_eq!(stack_elem, test.expected);
    }
}

#[test]
fn test_integer_object() {
    let tests = vec![
        VMTestCase {
            input: String::from("1"),
            expected: Object::Integer(1),
        },
        VMTestCase {
            input: String::from("2"),
            expected: Object::Integer(2),
        },
        VMTestCase {
            input: String::from("1 + 2"),
            expected: Object::Integer(3), // TODO: fix this
        },
    ];

    run_vm_tests(tests);
}