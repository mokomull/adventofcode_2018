use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::tuple;
use nom::IResult;
use nom::ParseTo;
use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq)]
enum ClayEntry {
    XRange { y: usize, x: RangeInclusive<usize> },
    YRange { x: usize, y: RangeInclusive<usize> },
}

fn integer<T: std::str::FromStr>(input: &[u8]) -> IResult<&[u8], T> {
    let (rest, digits) = digit1(input)?;
    let result = digits.parse_to().unwrap();
    Ok((rest, result))
}

fn xrange(input: &[u8]) -> IResult<&[u8], ClayEntry> {
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
        ClayEntry::XRange {
            y,
            x: startx..=endx,
        },
    ))
}

fn yrange(input: &[u8]) -> IResult<&[u8], ClayEntry> {
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
        ClayEntry::YRange {
            x,
            y: starty..=endy,
        },
    ))
}

fn clay_entry(input: &[u8]) -> IResult<&[u8], ClayEntry> {
    alt((xrange, yrange))(input)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_single_entries() {
        assert_eq!(
            clay_entry(b"y=7, x=495..501"),
            Ok((&b""[..], ClayEntry::XRange { y: 7, x: 495..=501 }))
        );
        assert_eq!(
            clay_entry(b"x=498, y=2..4"),
            Ok((&b""[..], ClayEntry::YRange { x: 498, y: 2..=4 }))
        );
    }
}
