#[macro_use]
extern crate nom;

use std::convert::TryInto;

use nom::types::CompleteByteSlice;
use nom::ErrorKind::Custom;

type Reg = u64;

#[derive(Debug, PartialEq)]
enum Line {
    Before([Reg; 4]),
    Opcode([u64; 4]),
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
    alt!(before_or_after | opcode | empty)
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

named!(opcode(CompleteByteSlice) -> Line,
    do_parse!(
        // no "separator" for four_integers, since it's already wrapped in ws!().
        operations: call!(four_integers::<u64, _>, b"") >>
        (Line::Opcode(operations))
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
        Ok((empty_string, Opcode([9, 2, 1, 2])))
    );

    assert_eq!(
        line(b"After:  [3, 2, 2, 1]"[..].into()),
        Ok((empty_string, After([3, 2, 2, 1])))
    );

    assert_eq!(line(empty_string), Ok((empty_string, Empty)));
}
