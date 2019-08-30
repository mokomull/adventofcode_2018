#![allow(clippy::needless_range_loop)]

#[macro_use]
extern crate nom;

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

#[allow(clippy::cognitive_complexity)]
fn parse_claim(input: &str) -> Option<Claim> {
    use nom::digit;
    let parsed = ws!(
        nom::types::CompleteStr(input),
        do_parse!(
            tag!("#")
                >> id: digit
                >> tag!("@")
                >> left: digit
                >> tag!(",")
                >> top: digit
                >> tag!(":")
                >> width: digit
                >> tag!("x")
                >> height: digit
                >> (Claim {
                    id: id.parse::<usize>().expect("parse digits"),
                    left: left.parse::<usize>().expect("parse digits"),
                    top: top.parse::<usize>().expect("parse digits"),
                    width: width.parse::<usize>().expect("parse digits"),
                    height: height.parse::<usize>().expect("parse digits")
                })
        )
    );
    parsed.ok().map(|(_rest, result)| result)
}

fn count_claims<T: AsRef<[Claim]>>(input: &T) -> Vec<Vec<usize>> {
    use std::cmp::max;
    let (width, height) = input.as_ref().iter().fold((0, 0), |(w, h), claim| {
        (
            max(w, claim.left + claim.width),
            max(h, claim.top + claim.height),
        )
    });

    let mut counts = vec![vec![0; width]; height];

    for claim in input.as_ref() {
        for i in claim.left..claim.left + claim.width {
            for j in claim.top..claim.top + claim.height {
                counts[i][j] += 1;
            }
        }
    }

    counts
}

fn count_overlapping(counts: &[Vec<usize>]) -> usize {
    let mut count = 0;
    for i in 0..counts.len() {
        for j in 0..counts[0].len() {
            if counts[i][j] > 1 {
                count += 1;
            }
        }
    }

    count
}

fn find_nonoverlapping<T: AsRef<[Claim]>>(input: &T, counts: &[Vec<usize>]) -> Option<usize> {
    'candidate: for i in input.as_ref() {
        for x in i.left..i.left + i.width {
            for y in i.top..i.top + i.height {
                if counts[x][y] > 1 {
                    continue 'candidate;
                }
            }
        }
        return Some(i.id);
    }
    None
}

fn main() {
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let claims: Vec<_> = lock
        .lines()
        .map(|l| parse_claim(&l.unwrap()).expect("input was well-formed"))
        .collect();

    let counts = count_claims(&claims);
    println!("Overlapping squares: {}", count_overlapping(&counts));
    if let Some(i) = find_nonoverlapping(&claims, &counts) {
        println!("Nonoverlapping id: {}", i);
    }
}

#[test]
fn test_parse() {
    assert_eq!(
        parse_claim("#1 @ 1,3: 4x4 "),
        Some(Claim {
            id: 1,
            left: 1,
            top: 3,
            width: 4,
            height: 4
        })
    );
    assert_eq!(
        parse_claim("#2 @ 3,1: 4x4"),
        Some(Claim {
            id: 2,
            left: 3,
            top: 1,
            width: 4,
            height: 4
        })
    );
    assert_eq!(
        parse_claim("#3 @ 5,5: 2x2"),
        Some(Claim {
            id: 3,
            left: 5,
            top: 5,
            width: 2,
            height: 2
        })
    );
}

#[test]
fn example() {
    let claims = vec![
        parse_claim("#1 @ 1,3: 4x4").unwrap(),
        parse_claim("#2 @ 3,1: 4x4").unwrap(),
        parse_claim("#3 @ 5,5: 2x2").unwrap(),
    ];

    let counts = count_claims(&claims);
    assert_eq!(count_overlapping(&counts), 4);
    assert_eq!(find_nonoverlapping(&claims, &counts), Some(3));
}
