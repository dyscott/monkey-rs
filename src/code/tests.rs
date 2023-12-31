use super::*;
use crate::make;

#[test]
fn test_make() {
    struct Test {
        op: Opcode,
        operands: Vec<u64>,
        expected: Instructions,
    }
    let tests = vec![
        Test {
            op: Opcode::OpConstant,
            operands: vec![65534],
            expected: vec![Opcode::OpConstant as u8, 255, 254],
        },
        Test {
            op: Opcode::OpAdd,
            operands: vec![],
            expected: vec![Opcode::OpAdd as u8],
        },
        Test {
            op: Opcode::OpGetLocal,
            operands: vec![255],
            expected: vec![Opcode::OpGetLocal as u8, 255],
        },
        Test {
            op: Opcode::OpClosure,
            operands: vec![65534, 255],
            expected: vec![Opcode::OpClosure as u8, 255, 254, 255],
        },
    ];

    for test in tests {
        let instruction = make(test.op, test.operands);
        assert_eq!(instruction, test.expected);
    }
}

#[test]
fn test_instructions_string() {
    let instructions = vec![
        make!(OpAdd),
        make!(OpGetLocal, [1]),
        make!(OpConstant, [2]),
        make!(OpConstant, [65535]),
        make!(OpClosure, [65535, 255]),
    ];

    let expected = String::from(
        "0000 OpAdd\n\
         0001 OpGetLocal 1\n\
         0003 OpConstant 2\n\
         0006 OpConstant 65535\n\
         0009 OpClosure 65535 255\n",
    );

    let mut concatted = Instructions::new();
    for instruction in instructions {
        concatted.extend(instruction);
    }

    assert_eq!(expected, instructions_string(&concatted));
}

#[test]
fn test_read_operands() {
    struct Test {
        op: Opcode,
        operands: Vec<u64>,
        bytes_read: usize,
    }
    let tests = vec![
        Test {
            op: Opcode::OpConstant,
            operands: vec![65535],
            bytes_read: 2,
        },
        Test {
            op: Opcode::OpGetLocal,
            operands: vec![255],
            bytes_read: 1,
        },
        Test {
            op: Opcode::OpClosure,
            operands: vec![65535, 255],
            bytes_read: 3,
        },
    ];

    for test in tests {
        let instruction = make(test.op.clone(), test.operands.clone());
        let def = test.op.lookup();
        let (operands_read, n) = read_operands(&def, &instruction[1..]);
        assert_eq!(test.bytes_read, n);
        assert_eq!(test.operands, operands_read);
    }
}