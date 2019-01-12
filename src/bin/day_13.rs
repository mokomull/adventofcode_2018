#[macro_use]
extern crate nom;

#[derive(Debug, PartialEq)]
enum Segment {
    Vertical,
    Horizontal,
    CurveUpRight,
    CurveDownRight,
    CurveDownLeft,
    CurveUpLeft,
    Intersection,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[derive(Debug, PartialEq)]
enum ParsedSegment {
    Vertical,
    Horizontal,
    CurveUpLeftOrDownRight,
    CurveDownLeftOrUpRight,
    Cart(Direction),
    Intersection,
    Empty,
}

use nom::types::CompleteByteSlice as ParserInput;

named!(parse_segment(ParserInput) -> ParsedSegment,
    alt!(
        do_parse!(tag!(&b"-"[..]) >> (ParsedSegment::Horizontal)) |
        do_parse!(tag!(&b"|"[..]) >> (ParsedSegment::Vertical)) |
        do_parse!(tag!(&b"\\"[..]) >> (ParsedSegment::CurveDownLeftOrUpRight)) |
        do_parse!(tag!(&b"/"[..]) >> (ParsedSegment::CurveUpLeftOrDownRight)) |
        do_parse!(tag!(&b">"[..]) >> (ParsedSegment::Cart(Direction::Right))) |
        do_parse!(tag!(&b"v"[..]) >> (ParsedSegment::Cart(Direction::Down))) |
        do_parse!(tag!(&b"<"[..]) >> (ParsedSegment::Cart(Direction::Left))) |
        do_parse!(tag!(&b"^"[..]) >> (ParsedSegment::Cart(Direction::Up))) |
        do_parse!(tag!(&b"+"[..]) >> (ParsedSegment::Intersection)) |
        do_parse!(tag!(&b" "[..]) >> (ParsedSegment::Empty))
    )
);

named!(parse_row(ParserInput) -> Vec<ParsedSegment>,
    many0!(parse_segment)
);

type Map = Vec<Vec<Segment>>;
type Carts = Vec<(Direction, (usize, usize))>;

fn parse_map(input: &[u8]) -> (Map, Carts) {
    let input = ParserInput(input);
    #[rustfmt::skip]
    let (_rest, mut rows) = do_parse!(
        input,
        foo: many1!(
            do_parse!(
                row: parse_row >>
                opt!(tag!(&b"\n"[..])) >>
                (row)
            )
        ) >>
        (foo)
    ).unwrap();

    // Remove carts to leave the underlying tracks
    let mut carts = vec![];

    for (row, row_data) in rows.iter_mut().enumerate() {
        for (col, cell) in row_data.iter_mut().enumerate() {
            match *cell {
                ParsedSegment::Cart(d) => {
                    carts.push((d, (col, row)));

                    match d {
                        Direction::Left | Direction::Right => {
                            *cell = ParsedSegment::Horizontal;
                        }
                        Direction::Up | Direction::Down => {
                            *cell = ParsedSegment::Vertical;
                        }
                    }
                }
                _ => {}
            }
        }
    }

    // Resolve the curves
    let mut result = vec![];
    for (row, row_data) in rows.iter().enumerate() {
        let mut result_row = vec![];
        for (col, cell) in row_data.iter().enumerate() {
            let real_cell = match *cell {
                ParsedSegment::Vertical => Segment::Vertical,
                ParsedSegment::Horizontal => Segment::Horizontal,
                ParsedSegment::Empty => Segment::Empty,
                ParsedSegment::Intersection => Segment::Intersection,
                ParsedSegment::Cart(_) => panic!("Carts should have been filtered out"),
                ParsedSegment::CurveUpLeftOrDownRight => {
                    let left = is_horizontal(&rows, row, col.wrapping_sub(1));
                    let right = is_horizontal(&rows, row, col + 1);
                    let up = is_vertical(&rows, row.wrapping_sub(1), col);
                    let down = is_vertical(&rows, row + 1, col);

                    if left && up {
                        Segment::CurveUpLeft
                    } else if down && right {
                        Segment::CurveDownRight
                    } else {
                        panic!("neither upleft or downright at row {} col {}", row, col)
                    }
                }
                ParsedSegment::CurveDownLeftOrUpRight => {
                    let left = is_horizontal(&rows, row, col.wrapping_sub(1));
                    let right = is_horizontal(&rows, row, col + 1);
                    let up = is_vertical(&rows, row.wrapping_sub(1), col);
                    let down = is_vertical(&rows, row + 1, col);

                    if down && left {
                        Segment::CurveDownLeft
                    } else if up && right {
                        Segment::CurveUpRight
                    } else {
                        panic!("neither downleft or upright at row {} col {}", row, col)
                    }
                }
            };
            result_row.push(real_cell);
        }
        result.push(result_row);
    }

    (result, carts)
}

fn is_horizontal(data: &Vec<Vec<ParsedSegment>>, row: usize, col: usize) -> bool {
    let cell = data.get(row).and_then(|r| r.get(col));
    if cell == Some(&ParsedSegment::Horizontal) || cell == Some(&ParsedSegment::Intersection) {
        return true;
    }
    false
}

fn is_vertical(data: &Vec<Vec<ParsedSegment>>, row: usize, col: usize) -> bool {
    let cell = data.get(row).and_then(|r| r.get(col));
    if cell == Some(&ParsedSegment::Vertical) || cell == Some(&ParsedSegment::Intersection) {
        return true;
    }
    false
}

#[test]
fn test_parser() {
    use self::Direction::*;
    use self::Segment::*;

    assert_eq!(
        parse_segment(ParserInput(&b">"[..])).unwrap(),
        (ParserInput(&b""[..]), ParsedSegment::Cart(Direction::Right))
    );

    let (map, carts) = parse_map(
        b"/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ",
    );

    assert_eq!(
        map[0..2],
        vec![
            vec![
                CurveDownRight,
                Horizontal,
                Horizontal,
                Horizontal,
                CurveDownLeft,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty
            ],
            vec![
                Vertical,
                Empty,
                Empty,
                Empty,
                Vertical,
                Empty,
                Empty,
                CurveDownRight,
                Horizontal,
                Horizontal,
                Horizontal,
                Horizontal,
                CurveDownLeft
            ],
        ][..]
    );

    assert_eq!(carts, vec![(Right, (2, 0)), (Down, (9, 3)),]);
}
