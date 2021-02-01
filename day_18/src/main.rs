use nom::bytes::complete::tag;
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

fn acre(input: &str) -> IResult<&str, Acre> {
    nom::branch::alt((
        tag(".").map(|_| Acre::Open),
        tag("|").map(|_| Acre::Trees),
        tag("#").map(|_| Acre::Lumberyard),
    ))(input)
}

fn area(input: &str) -> Vec<Vec<Acre>> {
    separated_list1(nom::character::complete::line_ending, many1(acre))(input)
        .expect("produced a sequence of lines")
        .1
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_acre() {
        assert_eq!(acre("."), Ok(("", Acre::Open)));
        assert!(acre("l").is_err());
    }

    #[test]
    fn parse_area() {
        assert_eq!(
            area(
                ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|."
            ),
            vec![
                vec![
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Lumberyard,
                    Acre::Open
                ],
                vec![
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Trees,
                    Acre::Lumberyard,
                    Acre::Lumberyard,
                    Acre::Trees
                ],
                vec![
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Open
                ],
                vec![
                    Acre::Open,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Lumberyard,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Lumberyard
                ],
                vec![
                    Acre::Lumberyard,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Lumberyard,
                    Acre::Trees,
                    Acre::Lumberyard,
                    Acre::Trees
                ],
                vec![
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open
                ],
                vec![
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open
                ],
                vec![
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Trees
                ],
                vec![
                    Acre::Trees,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open
                ],
                vec![
                    Acre::Open,
                    Acre::Open,
                    Acre::Open,
                    Acre::Lumberyard,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open,
                    Acre::Open,
                    Acre::Trees,
                    Acre::Open
                ],
            ]
        )
    }
}
