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

macro_rules! make_compiled_function {
    ($instructions:expr) => {
        Object::CompiledFunction(CompiledFunction {
            instructions: concat_instructions($instructions),
            num_locals:  0,
            num_parameters: 0,
        })
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

#[test]
fn test_hash_literals() {
    let tests = vec![
        make_test!(
            "{}";
            ;
            make!(OpHash, [0]),
            make!(OpPop)
        ),
        make_test!(
            "{1: 2, 3: 4, 5: 6}";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(4),
            Object::Integer(5),
            Object::Integer(6);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpConstant, [3]),
            make!(OpConstant, [4]),
            make!(OpConstant, [5]),
            make!(OpHash, [6]),
            make!(OpPop)
        ),
        make_test!(
            "{1: 2 + 3, 4: 5 * 6}";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(4),
            Object::Integer(5),
            Object::Integer(6);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpAdd),
            make!(OpConstant, [3]),
            make!(OpConstant, [4]),
            make!(OpConstant, [5]),
            make!(OpMul),
            make!(OpHash, [4]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_index_expressions() {
    let tests = vec![
        make_test!(
            "[1, 2, 3][1 + 1]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(1),
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpArray, [3]),
            make!(OpConstant, [3]),
            make!(OpConstant, [4]),
            make!(OpAdd),
            make!(OpIndex),
            make!(OpPop)
        ),
        make_test!(
            "{1: 2}[2 - 1]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(2),
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpHash, [2]),
            make!(OpConstant, [2]),
            make!(OpConstant, [3]),
            make!(OpSub),
            make!(OpIndex),
            make!(OpPop)
        ),
        // String indexing, not supported in the book
        make_test!(
            "\"monkey\"[1]";
            Object::String(String::from("monkey")),
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpIndex),
            make!(OpPop)
        ),
        // Slice indexing, not supported in the book
        make_test!(
            "[1, 2, 3][1:2]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpArray, [3]),
            make!(OpConstant, [3]),
            make!(OpConstant, [4]),
            make!(OpSliceIndex),
            make!(OpPop)
        ),
        make_test!(
            "[1, 2, 3][1:]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(1);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpArray, [3]),
            make!(OpConstant, [3]),
            make!(OpNull),
            make!(OpSliceIndex),
            make!(OpPop)
        ),
        make_test!(
            "[1, 2, 3][:2]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpArray, [3]),
            make!(OpNull),
            make!(OpConstant, [3]),
            make!(OpSliceIndex),
            make!(OpPop)
        ),
        make_test!(
            "[1, 2, 3][:]";
            Object::Integer(1),
            Object::Integer(2),
            Object::Integer(3);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpArray, [3]),
            make!(OpNull),
            make!(OpNull),
            make!(OpSliceIndex),
            make!(OpPop)
        ),
        // String slicing, not supported in the book
        make_test!(
            "\"monkey\"[1:2]";
            Object::String(String::from("monkey")),
            Object::Integer(1),
            Object::Integer(2);
            make!(OpConstant, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpSliceIndex),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_functions() {
    let tests = vec![
        make_test!(
            "fn() { return 5 + 10 }";
            Object::Integer(5),
            Object::Integer(10),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpConstant, [1]),
                make!(OpAdd),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [2]),
            make!(OpPop)
        ),
        make_test!(
            "fn() { 5 + 10 }";
            Object::Integer(5),
            Object::Integer(10),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpConstant, [1]),
                make!(OpAdd),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [2]),
            make!(OpPop)
        ),
        make_test!(
            "fn() { 1; 2 }";
            Object::Integer(1),
            Object::Integer(2),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpPop),
                make!(OpConstant, [1]),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [2]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_compiler_scopes() {
    let mut compiler = Compiler::new();
    let global_sym = compiler.symbol_table.clone();

    assert_eq!(compiler.scope_index, 0);

    compiler.emit(Opcode::OpMul, vec![]);

    compiler.enter_scope();
    assert_eq!(compiler.scope_index, 1);

    compiler.emit(Opcode::OpSub, vec![]);
    assert_eq!(compiler.scopes[compiler.scope_index].instructions.len(), 1);
    assert_eq!(
        compiler.scopes[compiler.scope_index]
            .last_instruction
            .clone()
            .unwrap()
            .opcode,
        Opcode::OpSub
    );
    assert_eq!(compiler.symbol_table.clone().borrow().outer, Some(global_sym.clone()));

    compiler.leave_scope();
    assert_eq!(compiler.scope_index, 0);
    assert_eq!(compiler.symbol_table.clone(), global_sym);
    assert_eq!(compiler.symbol_table.clone().borrow().outer, None);

    compiler.emit(Opcode::OpAdd, vec![]);

    assert_eq!(compiler.scopes[compiler.scope_index].instructions.len(), 2);
    assert_eq!(
        compiler.scopes[compiler.scope_index]
            .last_instruction
            .clone()
            .unwrap()
            .opcode,
        Opcode::OpAdd
    );
    assert_eq!(
        compiler.scopes[compiler.scope_index]
            .previous_instruction
            .clone()
            .unwrap()
            .opcode,
        Opcode::OpMul
    );
}

#[test]
fn test_functions_without_return_value() {
    let tests = vec![make_test!(
        "fn() { }";
        make_compiled_function!(vec![
            make!(OpReturn),
        ]);
        make!(OpConstant, [0]),
        make!(OpPop)
    )];

    run_compiler_tests(tests);
}

#[test]
fn test_function_calls() {
    let tests = vec![
        make_test!(
            "fn() { 24 }()";
            Object::Integer(24),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [1]),
            make!(OpCall, [0]),
            make!(OpPop)
        ),
        make_test!(
            "let noArg = fn() { 24 }; noArg();";
            Object::Integer(24),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [1]),
            make!(OpSetGlobal, [0]),
            make!(OpGetGlobal, [0]),
            make!(OpCall, [0]),
            make!(OpPop)
        ),
        make_test!(
            "let oneArg = fn(a) { a }; oneArg(24);";
            make_compiled_function!(vec![
                make!(OpGetLocal, [0]),
                make!(OpReturnValue),
            ]),
            Object::Integer(24);
            make!(OpConstant, [0]),
            make!(OpSetGlobal, [0]),
            make!(OpGetGlobal, [0]),
            make!(OpConstant, [1]),
            make!(OpCall, [1]),
            make!(OpPop)
        ),
        make_test!(
            "let manyArg = fn(a, b, c) { a; b; c }; manyArg(24, 25, 26);";
            make_compiled_function!(vec![
                make!(OpGetLocal, [0]),
                make!(OpPop),
                make!(OpGetLocal, [1]),
                make!(OpPop),
                make!(OpGetLocal, [2]),
                make!(OpReturnValue),
            ]),
            Object::Integer(24),
            Object::Integer(25),
            Object::Integer(26);
            make!(OpConstant, [0]),
            make!(OpSetGlobal, [0]),
            make!(OpGetGlobal, [0]),
            make!(OpConstant, [1]),
            make!(OpConstant, [2]),
            make!(OpConstant, [3]),
            make!(OpCall, [3]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_let_statement_scopes() {
    let tests = vec![
        make_test!(
            "let num = 55; fn() { num }";
            Object::Integer(55),
            make_compiled_function!(vec![
                make!(OpGetGlobal, [0]),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [0]),
            make!(OpSetGlobal, [0]),
            make!(OpConstant, [1]),
            make!(OpPop)
        ),
        make_test!(
            "fn() { let num = 55; num }";
            Object::Integer(55),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpSetLocal, [0]),
                make!(OpGetLocal, [0]),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [1]),
            make!(OpPop)
        ),
        make_test!(
            "fn() { let a = 55; let b = 77; a + b }";
            Object::Integer(55),
            Object::Integer(77),
            make_compiled_function!(vec![
                make!(OpConstant, [0]),
                make!(OpSetLocal, [0]),
                make!(OpConstant, [1]),
                make!(OpSetLocal, [1]),
                make!(OpGetLocal, [0]),
                make!(OpGetLocal, [1]),
                make!(OpAdd),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [2]),
            make!(OpPop)
        ),
    ];

    run_compiler_tests(tests);
}

#[test]
fn test_builtins() {
    let tests = vec![
        make_test!(
            "len([]); push([], 1);";
            Object::Integer(1);
            make!(OpGetBuiltin, [0]),
            make!(OpArray, [0]),
            make!(OpCall, [1]),
            make!(OpPop),
            make!(OpGetBuiltin, [5]),
            make!(OpArray, [0]),
            make!(OpConstant, [0]),
            make!(OpCall, [2]),
            make!(OpPop)
        ),
        make_test!(
            "fn() { len([]) }";
            make_compiled_function!(vec![
                make!(OpGetBuiltin, [0]),
                make!(OpArray, [0]),
                make!(OpCall, [1]),
                make!(OpReturnValue),
            ]);
            make!(OpConstant, [0]),
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
    for (i, obj) in expected.iter().enumerate() {
        match obj {
            Object::CompiledFunction(expected) => {
                let actual = match &actual[i] {
                    Object::CompiledFunction(actual) => actual,
                    _ => panic!("object is not a CompiledFunction: {:?}", &actual[i]),
                };
                assert_eq!(expected.instructions, actual.instructions);
            }
            _ => assert_eq!(obj, &actual[i]),
        }
    }
}
