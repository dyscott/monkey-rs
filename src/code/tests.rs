use super::*;

#[test]
fn test_make() {
    struct Test {
        op: Opcode,
        operands: Vec<u64>,
        expected: Instructions,
    }
    let tests = vec![
        Test {
            op: Opcode::Constant,
            operands: vec![65534],
            expected: vec![Opcode::Constant as u8, 255, 254],
        },
    ];

    for tt in tests {
        let instruction = make(tt.op, tt.operands);
        assert_eq!(instruction, tt.expected);
    }
}