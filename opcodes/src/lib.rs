pub type Reg = u64;
pub type Op = usize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

use Opcode::*;

pub const ALL_OPCODES: &[Opcode] = &[
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

pub fn eval_one(opcode: Opcode, before: [Reg; 4], instruction: [Op; 4]) -> [Reg; 4] {
    let [_opcode, source_1_idx, source_2_idx, dest_idx] = instruction;

    let source_1 = before[source_1_idx];
    let source_2 = before[source_2_idx];

    let source_1_imm = source_1_idx as Reg;
    let source_2_imm = source_2_idx as Reg;

    let result_value = match opcode {
        Addr => source_1 + source_2,
        Addi => source_1 + source_2_imm,
        Mulr => source_1 * source_2,
        Muli => source_1 * source_2_imm,
        Banr => source_1 & source_2,
        Bani => source_1 & source_2_imm,
        Borr => source_1 | source_2,
        Bori => source_1 | source_2_imm,
        Setr => source_1,
        Seti => source_1_imm,
        Gtir => {
            if source_1_imm > source_2 {
                1
            } else {
                0
            }
        }
        Gtri => {
            if source_1 > source_2_imm {
                1
            } else {
                0
            }
        }
        Gtrr => {
            if source_1 > source_2 {
                1
            } else {
                0
            }
        }
        Eqir => {
            if source_1_imm == source_2 {
                1
            } else {
                0
            }
        }
        Eqri => {
            if source_1 == source_2_imm {
                1
            } else {
                0
            }
        }
        Eqrr => {
            if source_1 == source_2 {
                1
            } else {
                0
            }
        }
    };

    let mut result = before;
    result[dest_idx] = result_value;
    result
}
