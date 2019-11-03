use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::multi::separated_list;
use nom::sequence::tuple;
use nom::IResult;
use nom::ParseTo;
use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq)]
enum Vein {
    XRange { y: usize, x: RangeInclusive<usize> },
    YRange { x: usize, y: RangeInclusive<usize> },
}

fn integer<T: std::str::FromStr>(input: &[u8]) -> IResult<&[u8], T> {
    let (rest, digits) = digit1(input)?;
    let result = digits.parse_to().unwrap();
    Ok((rest, result))
}

fn xrange(input: &[u8]) -> IResult<&[u8], Vein> {
    let (rest, (_, y, _, startx, _, endx)) = tuple((
        tag(b"y="),
        integer,
        tag(b", x="),
        integer,
        tag(b".."),
        integer,
    ))(input)?;
    Ok((
        rest,
        Vein::XRange {
            y,
            x: startx..=endx,
        },
    ))
}

fn yrange(input: &[u8]) -> IResult<&[u8], Vein> {
    let (rest, (_, x, _, starty, _, endy)) = tuple((
        tag(b"x="),
        integer,
        tag(b", y="),
        integer,
        tag(b".."),
        integer,
    ))(input)?;
    Ok((
        rest,
        Vein::YRange {
            x,
            y: starty..=endy,
        },
    ))
}

fn vein(input: &[u8]) -> IResult<&[u8], Vein> {
    alt((xrange, yrange))(input)
}

fn veins(input: &[u8]) -> IResult<&[u8], Vec<Vein>> {
    separated_list(newline, vein)(input)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Square {
    Empty,
    Clay,
    Sand,
    WaterResting,
    WaterThrough,
}

fn parse_scan(input: &[u8]) -> ((usize, usize), Vec<Vec<Square>>) {
    let veins = veins(input).unwrap().1;
    let (spring_x, spring_y) = (500, 0);

    let xs = veins
        .iter()
        .flat_map(|vein| match vein {
            Vein::XRange { ref x, .. } => x.clone(),
            Vein::YRange { ref x, .. } => *x..=*x,
        })
        /* Option<integer type> was an easy cop-out for an iterator that yields a single value; I
        tried using the slightly more obvious [spring_x], but for some reason that becomes a
        temporary whose into_iter() borrows against it instead. */
        .chain(Some(spring_x));
    let ys = veins
        .iter()
        .flat_map(|vein| match vein {
            Vein::XRange { ref y, .. } => *y..=*y,
            Vein::YRange { ref y, .. } => y.clone(),
        })
        .chain(Some(spring_y));
    let min_x = xs.clone().min().unwrap() - 1;
    let max_x = xs.max().unwrap() + 1;
    let min_y = ys.clone().min().unwrap();
    let max_y = ys.max().unwrap();

    let mut result = vec![vec![Square::Sand; max_x - min_x + 1]; max_y - min_y + 1];

    for vein in veins {
        match vein {
            Vein::XRange { x: xs, y } => {
                for x in xs {
                    result[y - min_y][x - min_x] = Square::Clay;
                }
            }
            Vein::YRange { x, y: ys } => {
                for y in ys {
                    result[y - min_y][x - min_x] = Square::Clay;
                }
            }
        }
    }

    ((spring_x - min_x, spring_y - min_y), result)
}

fn count_reachable(spring: (usize, usize), ground: &Vec<Vec<Square>>) -> usize {
    use std::collections::{HashSet, VecDeque};
    use Square::Sand;

    let mut reached: HashSet<(usize, usize)> = HashSet::new();
    let mut to_visit = VecDeque::from(vec![spring]);

    while !to_visit.is_empty() {
        let (x, y) = to_visit.pop_front().unwrap();
        if !reached.insert((x, y)) {
            continue;
        }
        println!("visiting {:?} for the first time", (x, y));

        if let Some(Sand) = ground.get(y + 1).and_then(|row| row.get(x)) {
            to_visit.push_back((x, y + 1));
        }
        if let Some(Sand) = ground[y].get(x.wrapping_sub(1)) {
            to_visit.push_back((x - 1, y));
        }
        if let Some(Sand) = ground[y].get(x + 1) {
            to_visit.push_back((x + 1, y));
        }
    }

    reached.len()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &[u8] = b"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test_single_entries() {
        assert_eq!(
            vein(b"y=7, x=495..501"),
            Ok((&b""[..], Vein::XRange { y: 7, x: 495..=501 }))
        );
        assert_eq!(
            vein(b"x=498, y=2..4"),
            Ok((&b""[..], Vein::YRange { x: 498, y: 2..=4 }))
        );
    }

    #[test]
    fn test_veins() {
        assert_eq!(
            veins(EXAMPLE_INPUT),
            Ok((
                &b""[..],
                vec![
                    Vein::YRange { x: 495, y: 2..=7 },
                    Vein::XRange { y: 7, x: 495..=501 },
                    Vein::YRange { x: 501, y: 3..=7 },
                    Vein::YRange { x: 498, y: 2..=4 },
                    Vein::YRange { x: 506, y: 1..=2 },
                    Vein::YRange { x: 498, y: 10..=13 },
                    Vein::YRange { x: 504, y: 10..=13 },
                    Vein::XRange {
                        y: 13,
                        x: 498..=504
                    },
                ]
            ))
        )
    }

    #[test]
    fn test_parse_scan() {
        use super::Square::{Clay as C, Sand as S};
        assert_eq!(
            parse_scan(EXAMPLE_INPUT),
            (
                ((500 - 494), 0),
                vec![
                    vec![S, S, S, S, S, S, S, S, S, S, S, S, S, S],
                    vec![S, S, S, S, S, S, S, S, S, S, S, S, C, S],
                    vec![S, C, S, S, C, S, S, S, S, S, S, S, C, S],
                    vec![S, C, S, S, C, S, S, C, S, S, S, S, S, S],
                    vec![S, C, S, S, C, S, S, C, S, S, S, S, S, S],
                    vec![S, C, S, S, S, S, S, C, S, S, S, S, S, S],
                    vec![S, C, S, S, S, S, S, C, S, S, S, S, S, S],
                    vec![S, C, C, C, C, C, C, C, S, S, S, S, S, S],
                    vec![S, S, S, S, S, S, S, S, S, S, S, S, S, S],
                    vec![S, S, S, S, S, S, S, S, S, S, S, S, S, S],
                    vec![S, S, S, S, C, S, S, S, S, S, C, S, S, S],
                    vec![S, S, S, S, C, S, S, S, S, S, C, S, S, S],
                    vec![S, S, S, S, C, S, S, S, S, S, C, S, S, S],
                    vec![S, S, S, S, C, C, C, C, C, C, C, S, S, S],
                ]
            )
        )
    }

    #[test]
    fn test_count() {
        let (spring, ground) = parse_scan(EXAMPLE_INPUT);
        assert_eq!(count_reachable(spring, &ground), 57);
    }
}
