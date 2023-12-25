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
            expected: vec![Opcode::OpConstant as u8, 255, 254].into(),
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
        make!(OpConstant, [1]),
        make!(OpConstant, [2]),
        make!(OpConstant, [65535]),
    ];

    let expected = String::from(
        "0000 OpConstant 1\n\
         0003 OpConstant 2\n\
         0006 OpConstant 65535\n",
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
    ];

    for test in tests {
        let instruction = make(test.op.clone(), test.operands.clone());
        let def = test.op.lookup();
        let (operands_read, n) = read_operands(&def, &instruction[1..]);
        assert_eq!(test.bytes_read, n);
        assert_eq!(test.operands, operands_read);
    }
}