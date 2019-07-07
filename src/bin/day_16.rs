#[macro_use]
extern crate nom;

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

#[derive(Clone, Copy, Debug, PartialEq)]
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

fn which_opcodes(before: [Reg; 4], instruction: [Op; 4], after: [Reg; 4]) -> Vec<Opcode> {
    let [_opcode, source_1_idx, source_2_idx, dest_idx] = instruction;

    let source_1 = before[source_1_idx];
    let source_2 = before[source_2_idx];
    let dest = after[dest_idx];

    let source_1_imm = source_1_idx as Reg;
    let source_2_imm = source_2_idx as Reg;

    let mismatch_non_dest = before
        .iter()
        .zip(&after)
        .enumerate()
        .filter(|&(i, _)| i != dest_idx)
        .any(|(_, (b, a))| b != a);

    if mismatch_non_dest {
        return vec![];
    }

    use Opcode::*;

    let results_by_opcode = [
        (Addr, source_1 + source_2),
        (Addi, source_1 + source_2_imm),
        (Mulr, source_1 * source_2),
        (Muli, source_1 * source_2_imm),
        (Banr, source_1 & source_2),
        (Bani, source_1 & source_2_imm),
        (Borr, source_1 | source_2),
        (Bori, source_1 | source_2_imm),
        (Setr, source_1),
        (Seti, source_1_imm),
        (Gtir, if source_1_imm > source_2 { 1 } else { 0 }),
        (Gtri, if source_1 > source_2_imm { 1 } else { 0 }),
        (Gtrr, if source_1 > source_2 { 1 } else { 0 }),
        (Eqir, if source_1_imm == source_2 { 1 } else { 0 }),
        (Eqri, if source_1 == source_2_imm { 1 } else { 0 }),
        (Eqrr, if source_1 == source_2 { 1 } else { 0 }),
    ];

    results_by_opcode
        .iter()
        .filter_map(|&(opcode, result)| if dest == result { Some(opcode) } else { None })
        .collect()
}

fn how_many_opcodes(before: [Reg; 4], instruction: [Op; 4], after: [Reg; 4]) -> usize {
    which_opcodes(before, instruction, after).len()
}

#[test]
fn test_how_many_opcodes() {
    use Opcode::*;

    assert_eq!(
        how_many_opcodes([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]),
        3
    );

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

                // but if otherwise we've been partially set, then panic.
                let this_count = how_many_opcodes(
                    before.take().expect("before not set"),
                    instruction.take().expect("instruction not set"),
                    after.take().expect("after not set"),
                );
                if this_count >= 3 {
                    count += 1;
                }
            }
        }
    }

    println!("{} instructions can be 3 or more opcodes", count);
}
