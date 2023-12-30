use std::fmt::{Display, Formatter};

use anyhow::{anyhow, Error};

#[cfg(test)]
mod tests;

pub type Instructions = Vec<u8>;
pub type InstructionsSlice<'a> = &'a [u8];

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    OpConstant,
    OpPop,
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpTrue,
    OpFalse,
    OpEqual,
    OpNotEqual,
    OpGreaterThan,
    OpMinus,
    OpBang,
    OpJumpNotTruthy,
    OpJump,
    OpNull,
    OpGetGlobal,
    OpSetGlobal,
    OpArray,
    OpHash,
    OpIndex,
    OpSliceIndex,
    OpCall,
    OpReturnValue,
    OpReturn,
    OpGetLocal,
    OpSetLocal,
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
            Opcode::OpPop => Definition {
                name: "OpPop",
                operand_widths: vec![],
            },
            Opcode::OpAdd => Definition {
                name: "OpAdd",
                operand_widths: vec![],
            },
            Opcode::OpSub => Definition {
                name: "OpSub",
                operand_widths: vec![],
            },
            Opcode::OpMul => Definition {
                name: "OpMul",
                operand_widths: vec![],
            },
            Opcode::OpDiv => Definition {
                name: "OpDiv",
                operand_widths: vec![],
            },
            Opcode::OpTrue => Definition {
                name: "OpTrue",
                operand_widths: vec![],
            },
            Opcode::OpFalse => Definition {
                name: "OpFalse",
                operand_widths: vec![],
            },
            Opcode::OpEqual => Definition {
                name: "OpEqual",
                operand_widths: vec![],
            },
            Opcode::OpNotEqual => Definition {
                name: "OpNotEqual",
                operand_widths: vec![],
            },
            Opcode::OpGreaterThan => Definition {
                name: "OpGreaterThan",
                operand_widths: vec![],
            },
            Opcode::OpMinus => Definition {
                name: "OpMinus",
                operand_widths: vec![],
            },
            Opcode::OpBang => Definition {
                name: "OpBang",
                operand_widths: vec![],
            },
            Opcode::OpJumpNotTruthy => Definition {
                name: "OpJumpNotTruthy",
                operand_widths: vec![2],
            },
            Opcode::OpJump => Definition {
                name: "OpJump",
                operand_widths: vec![2],
            },
            Opcode::OpNull => Definition {
                name: "OpNull",
                operand_widths: vec![],
            },
            Opcode::OpGetGlobal => Definition {
                name: "OpGetGlobal",
                operand_widths: vec![2],
            },
            Opcode::OpSetGlobal => Definition {
                name: "OpSetGlobal",
                operand_widths: vec![2],
            },
            Opcode::OpArray => Definition {
                name: "OpArray",
                operand_widths: vec![2],
            },
            Opcode::OpHash => Definition {
                name: "OpHash",
                operand_widths: vec![2],
            },
            Opcode::OpIndex => Definition {
                name: "OpIndex",
                operand_widths: vec![]
            },
            Opcode::OpSliceIndex => Definition {
                name: "OpSliceIndex",
                operand_widths: vec![]
            },
            Opcode::OpCall => Definition {
                name: "OpCall",
                operand_widths: vec![1]
            },
            Opcode::OpReturnValue => Definition {
                name: "OpReturnValue",
                operand_widths: vec![]
            },
            Opcode::OpReturn => Definition {
                name: "OpReturn",
                operand_widths: vec![]
            },
            Opcode::OpGetLocal => Definition {
                name: "OpGetLocal",
                operand_widths: vec![1]
            },
            Opcode::OpSetLocal => Definition {
                name: "OpSetLocal",
                operand_widths: vec![1]
            },
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.lookup().name)
    }
}

impl TryFrom<u8> for Opcode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= Opcode::OpConstant as u8 && value <= Opcode::OpSetLocal as u8 {
            // Sadly, this is unsafe, but using a match would be verbose / slow
            return Ok(unsafe { std::mem::transmute(value) });
        } else {
            return Err(anyhow!("Invalid opcode: {}", value));
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
            1 => {
                instruction[offset] = (*operand & 0xff) as u8;
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
        let (operands, read) = read_operands(&def, &instructions[i + 1..]);
        out.push_str(&format!(
            "{:04} {}\n",
            i,
            format_instruction(&def, operands)
        ));
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
        0 => {
            return def.name.to_string();
        }
        1 => {
            return format!("{} {}", def.name, operands[0]);
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
            },
            1 => {
                let operand = u64::from(instructions[offset]);
                operands.push(operand);
            },
            _ => panic!("Unhandled operand width: {}", width)
        }
        offset += width;
    }

    return (operands, offset);
}

pub fn read_u16(instructions: InstructionsSlice) -> u16 {
    u16::from(instructions[0]) << 8 | u16::from(instructions[1])
}

// Macro to easily make instructions
#[macro_export]
macro_rules! make {
    ($opcode:ident, [$($x:expr),*]) => {
        make(Opcode::$opcode, vec![$($x),*])
    };
    ($opcode:ident) => {
        make(Opcode::$opcode, vec![])
    };
}
