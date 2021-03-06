use nom::bytes::complete::tag;
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};
use std::io::Read;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
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

fn advance(source: &Vec<Vec<Acre>>) -> Vec<Vec<Acre>> {
    let mut res = source.clone();

    for x in 0..source.len() {
        for y in 0..source[0].len() {
            match source[x][y] {
                Acre::Open => {
                    if count_adjacent(source, x, y, Acre::Trees) >= 3 {
                        res[x][y] = Acre::Trees;
                    }
                }
                Acre::Trees => {
                    if count_adjacent(source, x, y, Acre::Lumberyard) >= 3 {
                        res[x][y] = Acre::Lumberyard;
                    }
                }
                Acre::Lumberyard => {
                    if count_adjacent(source, x, y, Acre::Trees) == 0
                        || count_adjacent(source, x, y, Acre::Lumberyard) == 0
                    {
                        res[x][y] = Acre::Open;
                    }
                }
            }
        }
    }

    res
}

fn count_adjacent(source: &Vec<Vec<Acre>>, x: usize, y: usize, needle: Acre) -> usize {
    use itertools::Itertools;

    ((x as isize - 1)..=(x as isize + 1))
        .cartesian_product((y as isize - 1)..=(y as isize + 1))
        .filter(|&loc| loc != (x as isize, y as isize))
        .map(|(x1, y1)| source.get(x1 as usize).and_then(|row| row.get(y1 as usize)))
        .filter(|&haystack| haystack == Some(&needle))
        .count()
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("could not read stdin");

    let north_pole = area(&input);

    let part1 = iterate(&north_pole, 10);
    dbg!(part1);

    let part2 = iterate(&north_pole, 1000000000);
    dbg!(part2);
}

fn iterate(input: &Vec<Vec<Acre>>, count: usize) -> usize {
    let mut north_pole = input.clone();

    let mut seen = std::collections::HashMap::new();

    for i in 0..count {
        if let Some(cycle_start) = seen.insert(north_pole.clone(), i) {
            let cycle_len = i - cycle_start;
            let smaller_i = cycle_start + (count - cycle_start) % cycle_len;

            north_pole = seen
                .iter()
                .find(|&(_, found_index)| found_index == &smaller_i)
                .expect("should have found the index")
                .0
                .clone();
            break;
        }

        north_pole = advance(&north_pole);
    }

    let trees = north_pole
        .iter()
        .flatten()
        .filter(|&acre| acre == &Acre::Trees)
        .count();
    let lumberyards = north_pole
        .iter()
        .flatten()
        .filter(|&acre| acre == &Acre::Lumberyard)
        .count();
    trees * lumberyards
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
    fn advance() {
        assert_eq!(
            super::advance(&area(
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
            )),
            area(
                ".......##.
......|###
.|..|...#.
..|#||...#
..##||.|#|
...#||||..
||...|||..
|||||.||.|
||||||||||
....||..|."
            )
        );
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
