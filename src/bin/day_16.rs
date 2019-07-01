#[macro_use]
extern crate nom;

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

named!(line(CompleteByteSlice) -> Line,
    alt!(before_or_after | opcode | empty)
);

named!(before_or_after(CompleteByteSlice) -> Line,
    do_parse!(
        kind: alt!(tag!(&b"Before: "[..]) | tag!(&b"After:  "[..])) >>
        tag!(&b"["[..]) >>
        items: ws!(separated_list!(tag!(&b","[..]), nom::digit)) >>
        cond!(items.len() != 4, error_position!(Custom(42))) >>
        (
            unimplemented!()
        )
    )
);

named!(opcode(CompleteByteSlice) -> Line,
    value!(unimplemented!())
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
