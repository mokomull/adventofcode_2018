#[macro_use]
extern crate nom;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum Turn {
    Right,
    Left,
    Straight,
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cart {
    dir: Direction,
    next_intersection: Turn,
    position: (usize, usize),
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

type Carts = Vec<Cart>;

fn parse_map(input: &[u8]) -> (Vec<Vec<Segment>>, Carts) {
    let input = ParserInput(input);
    #[rustfmt::skip]
    let (_rest, mut rows) = do_parse!(
        input,
        rows: many1!(
            do_parse!(
                row: parse_row >>
                opt!(call!(nom::line_ending)) >>
                (row)
            )
        ) >>
        (rows)
    ).unwrap();

    // Remove carts to leave the underlying tracks
    let mut carts: Carts = vec![];

    for (row, row_data) in rows.iter_mut().enumerate() {
        for (col, cell) in row_data.iter_mut().enumerate() {
            if let ParsedSegment::Cart(d) = *cell {
                carts.push(Cart {
                    position: (col, row),
                    dir: d,
                    next_intersection: Turn::Left,
                });

                match d {
                    Direction::Left | Direction::Right => {
                        *cell = ParsedSegment::Horizontal;
                    }
                    Direction::Up | Direction::Down => {
                        *cell = ParsedSegment::Vertical;
                    }
                }
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

fn is_horizontal(data: &[Vec<ParsedSegment>], row: usize, col: usize) -> bool {
    let cell = data.get(row).and_then(|r| r.get(col));
    if cell == Some(&ParsedSegment::Horizontal) || cell == Some(&ParsedSegment::Intersection) {
        return true;
    }
    false
}

fn is_vertical(data: &[Vec<ParsedSegment>], row: usize, col: usize) -> bool {
    let cell = data.get(row).and_then(|r| r.get(col));
    if cell == Some(&ParsedSegment::Vertical) || cell == Some(&ParsedSegment::Intersection) {
        return true;
    }
    false
}

fn step<T, F>(map: &[Vec<Segment>], mut carts: Carts, on_collision: F) -> Result<Carts, T>
where
    // Ok(i: usize) means to continue iterating through the carts at i
    // Err(T) means to stop iterating altogether, and return Err(T) to the
    // caller (i.e. trains have irreparably collided)
    F: Fn(usize, (usize, usize), &mut Carts) -> Result<usize, T>,
{
    // because positions are (col, row) but we want to iterate through carts in row order
    carts.sort_by_key(|&c| (c.position.1, c.position.0));

    let mut i = 0;

    while i < carts.len() {
        let mut cart: Cart = carts[i];
        let (col, row) = cart.position;

        let (next_col, next_row) = match cart.dir {
            Direction::Right => (col + 1, row),
            Direction::Down => (col, row + 1),
            Direction::Left => (col - 1, row),
            Direction::Up => (col, row - 1),
        };

        let next_dir = match (cart.dir, map[next_row][next_col]) {
            (Direction::Right, Segment::CurveUpLeft) => Direction::Up,
            (Direction::Right, Segment::CurveDownLeft) => Direction::Down,
            (Direction::Down, Segment::CurveUpLeft) => Direction::Left,
            (Direction::Down, Segment::CurveUpRight) => Direction::Right,
            (Direction::Left, Segment::CurveUpRight) => Direction::Up,
            (Direction::Left, Segment::CurveDownRight) => Direction::Down,
            (Direction::Up, Segment::CurveDownLeft) => Direction::Left,
            (Direction::Up, Segment::CurveDownRight) => Direction::Right,
            (_, Segment::Intersection) => match cart.next_intersection {
                Turn::Left => {
                    cart.next_intersection = Turn::Straight;
                    match cart.dir {
                        Direction::Right => Direction::Up,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Down,
                        Direction::Up => Direction::Left,
                    }
                }
                Turn::Straight => {
                    cart.next_intersection = Turn::Right;
                    cart.dir
                }
                Turn::Right => {
                    cart.next_intersection = Turn::Left;
                    match cart.dir {
                        Direction::Right => Direction::Down,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Up => Direction::Right,
                    }
                }
            },
            _ => cart.dir,
        };

        carts[i] = Cart {
            position: (next_col, next_row),
            dir: next_dir,
            ..cart
        };

        if carts
            .iter()
            .enumerate()
            .any(|(j, &c)| i != j && c.position == (next_col, next_row))
        {
            i = on_collision(i, (next_col, next_row), &mut carts)?;
        } else {
            i += 1;
        }
    }

    Ok(carts)
}

fn collide(map: &[Vec<Segment>], carts: Carts) -> (usize, usize) {
    let mut res = Ok(carts);

    fn on_collision(
        _: usize,
        position: (usize, usize),
        _: &mut Carts,
    ) -> Result<usize, (usize, usize)> {
        Err(position)
    }

    while let Ok(carts) = res {
        res = step(map, carts, on_collision);
    }

    res.unwrap_err()
}

fn last_standing(map: &[Vec<Segment>], mut carts: Carts) -> (usize, usize) {
    fn on_collision(
        mut i: usize,
        position: (usize, usize),
        carts: &mut Carts,
    ) -> Result<usize, ()> {
        let mut j = 0;
        while j < carts.len() {
            if carts[j].position == position {
                carts.remove(j);
                if j < i {
                    i -= 1;
                }
            } else {
                j += 1;
            }
        }
        Ok(i)
    }

    while carts.len() > 1 {
        carts = step(map, carts, on_collision).unwrap();
    }

    carts[0].position
}

fn main() {
    use std::io::Read;

    let stdin = std::io::stdin();
    let mut lock = stdin.lock();
    let mut input = vec![];
    lock.read_to_end(&mut input).unwrap();

    let (map, carts) = parse_map(&input);
    println!("First collision at {:?}", collide(&map, carts.clone()));
    println!(
        "Last one standing is {:?}",
        last_standing(&map, carts.clone())
    );
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

    assert_eq!(
        carts,
        vec![
            Cart {
                position: (2, 0),
                dir: Right,
                next_intersection: Turn::Left
            },
            Cart {
                position: (9, 3),
                dir: Down,
                next_intersection: Turn::Left
            },
        ]
    );
}

#[test]
fn test_collision() {
    let (map, carts) = parse_map(
        b"/->-\\        
|   |  /----\\
| /-+--+-\\  |
| | |  | v  |
\\-+-/  \\-+--/
  \\------/   ",
    );

    assert_eq!(collide(&map, carts), (7, 3));

    let (map, carts) = parse_map(
        b"/>-<\\  
|   |  
| /<+-\\
| | | v
\\>+</ |
  |   ^
  \\<->/",
    );

    assert_eq!(last_standing(&map, carts), (6, 4));
}
