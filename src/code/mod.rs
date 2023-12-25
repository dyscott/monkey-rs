#[cfg(test)]
mod tests;

pub type Instructions = Vec<u8>;

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Opcode {
    Constant,
}

struct Definition {
    name: &'static str,
    operand_widths: Vec<usize>,
}

impl Opcode {
    fn lookup(&self) -> Definition {
        match self {
            Opcode::Constant => Definition {
                name: "OpConstant",
                operand_widths: vec![2],
            },
        }
    }
}

pub fn make(op: Opcode, operands: Vec<u64>) -> Instructions {
    let def = op.lookup();

    let instruction_len = def.operand_widths.iter().sum::<usize>() + 1;

    let mut instruction: Instructions = vec![0; instruction_len];
    instruction[0] = op as u8;

    let mut offset = 1;
    for (i, operand) in operands.iter().enumerate() {
        let width = def.operand_widths[i];
        match width {
            2 => {
                instruction[offset] = ((*operand >> 8) & 0xff) as u8;
                instruction[offset + 1] = (*operand & 0xff) as u8;
            }
            _ => unimplemented!(),
        }
        offset += width;
    }

    return instruction;
}
