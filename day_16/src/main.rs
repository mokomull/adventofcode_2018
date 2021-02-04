#[macro_use]
extern crate nom;

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::io::BufRead;

use nom::types::CompleteByteSlice;

use opcodes::Opcode::{self};
use opcodes::{eval_one, Op, Reg};

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
            match *kind {
                b"Before: " => Line::Before(items),
                b"After:  " => Line::After(items),
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

fn which_opcodes(before: [Reg; 4], instruction: [Op; 4], after: [Reg; 4]) -> Vec<Opcode> {
    opcodes::ALL_OPCODES
        .iter()
        .filter_map(|&opcode| {
            if after == eval_one(opcode, before, instruction) {
                Some(opcode)
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_how_many_opcodes() {
    use opcodes::Opcode::*;

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

                possibilities
                    .entry(opcode)
                    .or_insert_with(|| vec![])
                    .push(opcodes)
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
                registers = eval_one(opcode_map[&instruction[0]], registers, instruction);
            }
            Line::Empty => (),
            x => panic!("Should not see anything but instructions here: {:?}", x),
        }
    }

    println!("Register 0 contains {}", registers[0]);
}
