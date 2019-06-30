#[macro_use]
extern crate nom;

use nom::types::CompleteByteSlice;
type Reg = u64;

#[derive(Debug, PartialEq)]
enum Line {
    Before([Reg; 4]),
    Opcode([u64; 4]),
    After([Reg; 4]),
    Empty,
}

named!(line(CompleteByteSlice) -> Line,
    value!(unimplemented!())
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
