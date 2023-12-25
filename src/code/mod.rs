use anyhow::{Error, anyhow};

#[cfg(test)]
mod tests;

pub type Instructions = Vec<u8>;
pub type InstructionsSlice<'a> = &'a[u8];

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Opcode {
    OpConstant,
}

pub struct Definition {
    name: &'static str,
    operand_widths: Vec<usize>,
}

impl Opcode {
    fn lookup(&self) -> Definition {
        match self {
            Opcode::OpConstant => Definition {
                name: "OpConstant",
                operand_widths: vec![2],
            },
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::OpConstant),
            _ => Err(anyhow!("Unknown opcode: {}", value)),
        }
    }
}

pub fn make(op: Opcode, operands: Vec<u64>) -> Instructions {
    let def = op.lookup();

    let instruction_len = def.operand_widths.iter().sum::<usize>() + 1;

    let mut instruction: Instructions = vec![0; instruction_len].into();
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

pub fn instructions_string(instructions: &Instructions) -> String {
    let mut out = String::new();

    let mut i = 0;
    while i < instructions.len() {
        let op = match Opcode::try_from(instructions[i]) {
            Ok(op) => op,
            Err(err) => {
                out.push_str(&format!("ERROR: {}\n", err));
                continue;
            }
        };
        let def = op.lookup();
        let (operands, read) = read_operands(&def, &instructions[i+1..]);
        out.push_str(&format!("{:04} {}\n", i, format_instruction(&def, operands)));
        i += 1 + read;
    }

    return out;
}

pub fn format_instruction(def: &Definition, operands: Vec<u64>) -> String {
    let operand_count = def.operand_widths.len();
    if operands.len() != operand_count {
        return format!(
            "ERROR: operand len {} does not match defined {}\n",
            operands.len(),
            operand_count
        );
    }

    match operand_count {
        1 => {
            return format!(
                "{} {}", def.name, operands[0]
            );
        }
        _ => {
            return format!("ERROR: unhandled operand_count for {}\n", def.name);
        }
    }
}

pub fn read_operands(def: &Definition, instructions: InstructionsSlice) -> (Vec<u64>, usize) {
    let mut operands = vec![];
    let mut offset = 0;

    for width in def.operand_widths.iter() {
        match width {
            2 => {
                let operand = u64::from(read_u16(&instructions[offset..]));
                operands.push(operand);
            }
            _ => unimplemented!(),
        }
        offset += width;
    }

    return (operands, offset);
}

pub fn read_u16(instructions: InstructionsSlice) -> u16 {
    u16::from(instructions[0])<<8 | u16::from(instructions[1])
}

// Macro to easily make instructions
#[macro_export]
macro_rules! make {
    ($opcode:ident, [$($x:expr),*]) => {
        make(Opcode::$opcode, vec![$($x),*])
    };
}
