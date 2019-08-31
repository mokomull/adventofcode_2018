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

#[cfg(test)]
mod test {
    use super::*;
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
            veins(
                b"x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504"
            ),
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
}
