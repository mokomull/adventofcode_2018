#[macro_use]
extern crate nom;

use std::convert::TryInto;

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

fn how_many_opcodes(before: [Reg; 4], instruction: [Op; 4], after: [Reg; 4]) -> usize {
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
        return 0;
    }

    let operations = [
        dest == source_1 + source_2,                          // addr
        dest == source_1 + source_2_imm,                      // addi
        dest == source_1 * source_2,                          // mulr
        dest == source_1 * source_2_imm,                      // muli
        dest == source_1 & source_2,                          // banr
        dest == source_1 & source_2_imm,                      // bani
        dest == source_1 | source_2,                          // borr
        dest == source_1 | source_2_imm,                      // bori
        dest == source_1,                                     // setr
        dest == source_1_imm,                                 // seti
        dest == if source_1_imm > source_2 { 1 } else { 0 },  // gtir
        dest == if source_1 > source_2_imm { 1 } else { 0 },  // gtri
        dest == if source_1 > source_2 { 1 } else { 0 },      // gtrr
        dest == if source_1_imm == source_2 { 1 } else { 0 }, // eqir
        dest == if source_1 == source_2_imm { 1 } else { 0 }, // eqri
        dest == if source_1 == source_2 { 1 } else { 0 },     // eqrr
    ];

    operations.iter().filter(|&&x| x).count()
}

#[test]
fn test_how_many_opcodes() {
    assert_eq!(
        how_many_opcodes([3, 2, 1, 1], [9, 2, 1, 2], [3, 2, 2, 1]),
        3
    )
}
