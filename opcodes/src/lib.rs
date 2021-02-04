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

pub fn eval_one<const N: usize>(
    opcode: Opcode,
    before: [Reg; N],
    instruction: [Op; 4],
) -> [Reg; N] {
    let [_opcode, source_1_idx, source_2_idx, dest_idx] = instruction;

    let source_1 = before.get(source_1_idx).cloned();
    let source_2 = before.get(source_2_idx).cloned();

    let source_1_imm = source_1_idx as Reg;
    let source_2_imm = source_2_idx as Reg;

    let result_value = match opcode {
        Addr => source_1.unwrap() + source_2.unwrap(),
        Addi => source_1.unwrap() + source_2_imm,
        Mulr => source_1.unwrap() * source_2.unwrap(),
        Muli => source_1.unwrap() * source_2_imm,
        Banr => source_1.unwrap() & source_2.unwrap(),
        Bani => source_1.unwrap() & source_2_imm,
        Borr => source_1.unwrap() | source_2.unwrap(),
        Bori => source_1.unwrap() | source_2_imm,
        Setr => source_1.unwrap(),
        Seti => source_1_imm,
        Gtir => {
            if source_1_imm > source_2.unwrap() {
                1
            } else {
                0
            }
        }
        Gtri => {
            if source_1.unwrap() > source_2_imm {
                1
            } else {
                0
            }
        }
        Gtrr => {
            if source_1.unwrap() > source_2.unwrap() {
                1
            } else {
                0
            }
        }
        Eqir => {
            if source_1_imm == source_2.unwrap() {
                1
            } else {
                0
            }
        }
        Eqri => {
            if source_1.unwrap() == source_2_imm {
                1
            } else {
                0
            }
        }
        Eqrr => {
            if source_1.unwrap() == source_2.unwrap() {
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
