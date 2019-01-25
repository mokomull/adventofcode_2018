#[macro_use]
extern crate nom;

use nom::types::CompleteByteSlice;

#[derive(Debug, PartialEq)]
enum Unit {
    Elf,
    Goblin,
    Wall,
    Empty,
}

use self::Unit::*;

named!(unit(CompleteByteSlice) -> Unit,
    alt!(
        do_parse!(tag!(&b"#"[..]) >> (Wall)) |
        do_parse!(tag!(&b"E"[..]) >> (Elf)) |
        do_parse!(tag!(&b"G"[..]) >> (Goblin)) |
        do_parse!(tag!(&b"."[..]) >> (Empty))
    )
);

named!(board(CompleteByteSlice) -> Vec<Vec<Unit>>,
    do_parse!(
        rows: many1!(
            do_parse!(
                row: many1!(unit) >>
                opt!(tag!(&b"\n"[..])) >>
                (row)
            )
        ) >>
        (rows)
    )
);

#[test]
fn test_parser() {
    let input = b"#######
#E..G.#
#...#.#
#.G.#G#
#######";
    let (remaining, board) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(remaining, CompleteByteSlice(&b""[..]));
    assert_eq!(
        board,
        vec![
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Elf, Empty, Empty, Goblin, Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Wall],
            vec![Wall, Empty, Goblin, Empty, Wall, Goblin, Wall],
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall],
        ]
    );
}
