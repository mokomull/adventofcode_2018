#[macro_use]
extern crate nom;

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io::BufRead;

use nom::types::CompleteByteSlice;
use nom::ErrorKind::Custom;

type Reg = u64;
type Op = usize;

#[derive(Debug, PartialEq)]
enum Line {
    Before([Reg; 4]),
    Instruction([Op; 4]),
    After([Reg; 4]),
    Empty,
}

fn create_error(input: CompleteByteSlice) -> nom::IResult<CompleteByteSlice, (), u32> {
    Err(nom::Err::Error(error_position!(
        input,
        nom::ErrorKind::Custom(0)
    )))
}

named!(line(CompleteByteSlice) -> Line,
    alt!(before_or_after | instruction | empty)
);

fn four_integers<'a, T, U>(
    input: CompleteByteSlice<'a>,
    separator: &[u8],
) -> nom::IResult<CompleteByteSlice<'a>, [T; 4], u32>
where
    T: std::str::FromStr<Err = U> + Copy,
    U: std::fmt::Debug,
{
    do_parse!(
        input,
        items: ws!(separated_list!(tag!(separator), nom::digit))
            >> cond_with_error!(items.len() != 4, call!(create_error))
            >> (items
                .iter()
                .map(|x| std::str::from_utf8(x)
                    .expect("invalid utf8 somehow")
                    .parse::<T>()
                    .expect("couldn't parse an integer"))
                .collect::<Vec<_>>()[..]
                .try_into()
                .expect("wrong number of registers"))
    )
}

named!(before_or_after(CompleteByteSlice) -> Line,
    do_parse!(
        kind: alt!(tag!(&b"Before: "[..]) | tag!(&b"After:  "[..])) >>
        tag!(&b"["[..]) >>
        items: call!(four_integers::<Reg, _>, b",") >>
        tag!(&b"]"[..]) >>
        (
            match &*kind {
                &b"Before: " => Line::Before(items),
                &b"After:  " => Line::After(items),
                _ => panic!("parser bug: neither before nore after")
            }
        )
    )
);

named!(instruction(CompleteByteSlice) -> Line,
    do_parse!(
        // no "separator" for four_integers, since it's already wrapped in ws!().
        operations: call!(four_integers::<Op, _>, b"") >>
        (Line::Instruction(operations))
    )
);

named!(empty(CompleteByteSlice) -> Line,
    do_parse!(
        eof!() >>
        (Line::Empty)
    )
);

#[test]
fn line_parser() {
    use Line::*;
    let empty_string = b""[..].into();

    assert_eq!(
        line(b"Before: [3, 2, 1, 1]"[..].into()),
        Ok((empty_string, Before([3, 2, 1, 1])))
    );

    assert_eq!(
        line(b"9 2 1 2"[..].into()),
        Ok((empty_string, Instruction([9, 2, 1, 2])))
    );

    assert_eq!(
        line(b"After:  [3, 2, 2, 1]"[..].into()),
        Ok((empty_string, After([3, 2, 2, 1])))
    );

    assert_eq!(line(empty_string), Ok((empty_string, Empty)));
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Opcode {
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

const ALL_OPCODES: &[Opcode] = &[
    Addr, Addi, Mulr, Muli, Banr, Bani, Borr, Bori, Setr, Seti, Gtir, Gtri, Gtrr, Eqir, Eqri, Eqrr,
];

fn which_opcodes(before: [Reg; 4], instruction: [Op; 4], after: [Reg; 4]) -> Vec<Opcode> {
    ALL_OPCODES
        .iter()
        .filter_map(|&opcode| {
            if after == eval(opcode, before, instruction) {
                Some(opcode)
            } else {
                None
            }
        })
        .collect()
}

fn eval(opcode: Opcode, before: [Reg; 4], instruction: [Op; 4]) -> [Reg; 4] {
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

#[test]
fn test_how_many_opcodes() {
    assert_eq!(
        which_opcodes([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]),
        vec![Addi, Mulr, Seti]
    );
}

fn main() {
    let mut before = None;
    let mut instruction = None;
    let mut after = None;
    let mut count = 0;

    let mut possibilities: HashMap<Op, Vec<Vec<Opcode>>> = HashMap::new();

    for line in std::io::stdin().lock().lines() {
        let line = line.expect("stdin read failed");

        match crate::line(line.as_bytes().into()).expect("parse error").1 {
            Line::Before(x) => before = Some(x),
            Line::Instruction(x) => instruction = Some(x),
            Line::After(x) => after = Some(x),
            Line::Empty => {
                // the input is terminated by two empty lines
                if before.is_none() && instruction.is_none() && after.is_none() {
                    break;
                }

                let opcode = instruction.expect("instruction not set")[0];

                // but if otherwise we've been partially set, then panic.
                let opcodes = which_opcodes(
                    before.take().expect("before not set"),
                    instruction.take().expect("instruction not set"),
                    after.take().expect("after not set"),
                );
                if opcodes.len() >= 3 {
                    count += 1;
                }

                possibilities.entry(opcode).or_insert(vec![]).push(opcodes)
            }
        }
    }

    println!("{} instructions can be 3 or more opcodes", count);

    let mut known: HashMap<Opcode, Op> = HashMap::new();

    while !possibilities.is_empty() {
        let mut new_known: Vec<(Opcode, Op)> = Vec::new();

        for (opcode, opcode_sets) in &possibilities {
            let mut valid: HashSet<Opcode> = opcode_sets[0].iter().cloned().collect();
            for other in opcode_sets {
                let other_set: HashSet<Opcode> = other.iter().cloned().collect();
                valid.retain(|&x| other_set.contains(&x));
            }
            valid.retain(|&x| !known.contains_key(&x));

            if valid.len() == 1 {
                new_known.push((*valid.iter().next().unwrap(), *opcode));
            }
        }

        assert!(!new_known.is_empty());

        for (opcode, numeric) in new_known {
            possibilities.remove(&numeric);
            known.insert(opcode, numeric);
        }
    }

    let opcode_map: HashMap<Op, Opcode> = known
        .iter()
        .map(|(&opcode, &numeric)| (numeric, opcode))
        .collect();

    let mut registers = [0; 4];

    for line in std::io::stdin().lock().lines() {
        let line = line.expect("stdin read failed");

        match crate::line(line.as_bytes().into()).expect("parse error").1 {
            Line::Instruction(instruction) => {
                registers = eval(opcode_map[&instruction[0]], registers, instruction);
            }
            Line::Empty => (),
            x => panic!("Should not see anything but instructions here: {:?}", x),
        }
    }

    println!("Register 0 contains {}", registers[0]);
}
