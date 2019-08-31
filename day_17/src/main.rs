use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
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
}
