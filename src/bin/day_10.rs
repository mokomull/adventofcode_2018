use std::cmp::{max, min};

#[macro_use]
extern crate nom;
use nom::{digit, space};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point(isize, isize);

impl std::ops::Add for Point {
    type Output = Point;
    fn add(self, Point(x2, y2): Point) -> Point {
        Point(self.0 + x2, self.1 + y2)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Star {
    position: Point,
    velocity: Point,
}

named!(point(nom::types::CompleteStr) -> Point,
    do_parse!(
        tag!("<") >>
        opt!(space) >>
        x_minus: opt!(tag!("-")) >>
        x: digit >>
        tag!(",") >>
        opt!(space) >>
        y_minus: opt!(tag!("-")) >>
        y: digit >>
        tag!(">") >>
        (Point(
            if x_minus.is_some() {
                -1
            } else {
                1
            } * x.parse::<isize>().unwrap(),
            if y_minus.is_some() {
                -1
            } else {
                1
            } * y.parse::<isize>().unwrap(),
        ))
    )
);

fn parse(input: &str) -> Star {
    do_parse!(
        nom::types::CompleteStr(input),
        tag!("position=")
            >> position: point
            >> tag!(" velocity=")
            >> velocity: point
            >> (Star { position, velocity })
    )
    .unwrap()
    .1
}

fn extrema(input: &[Star]) -> (isize, isize, isize, isize) {
    input.iter().fold(
        (
            std::isize::MAX,
            std::isize::MIN,
            std::isize::MAX,
            std::isize::MIN,
        ),
        |(old_min_x, old_max_x, old_min_y, old_max_y), star| {
            (
                min(old_min_x, star.position.0),
                max(old_max_x, star.position.0),
                min(old_min_y, star.position.1),
                max(old_max_y, star.position.1),
            )
        },
    )
}

fn advance(input: &mut [Star]) {
    for x in input {
        x.position = x.position + x.velocity;
    }
}

fn render(input: &[Star]) {
    let mut sky = vec![vec![false; 80]; 20];
    let (min_x, max_x, min_y, max_y) = extrema(input);

    let &scale_factor = [1, (max_x - min_x) / 80, (max_y - min_y) / 20]
        .iter()
        .max()
        .unwrap();

    for star in input {
        let Point(x, y) = star.position;
        let row = (y - min_y) / scale_factor;
        let column = (x - min_x) / scale_factor;
        if row < 0 || row > 19 || column < 0 || column > 79 {
            continue;
        }
        sky[row as usize][column as usize] = true;
    }

    for row in &sky {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let option = std::env::args().nth(1).unwrap();

    let mut input: Vec<Star> = if "example" == option {
        EXAMPLE.iter().map(|&l| parse(l)).collect()
    } else {
        let input = std::fs::File::open(option).unwrap();
        let reader = std::io::BufReader::new(input);
        std::io::BufRead::lines(reader)
            .map(|l| parse(&l.unwrap()))
            .collect()
    };

    let mut closest = input.to_vec();
    let mut perimiter = std::isize::MAX;
    for _ in 0..1_000_000 {
        let (min_x, max_x, min_y, max_y) = extrema(&input);
        let this = max_x - min_x + max_y - min_y;
        if this < perimiter {
            perimiter = this;
            closest = input.to_vec();
        }
        advance(&mut input);
    }

    render(&closest);
}

const EXAMPLE: &[&str] = &[
    "position=< 9,  1> velocity=< 0,  2>",
    "position=< 7,  0> velocity=<-1,  0>",
    "position=< 3, -2> velocity=<-1,  1>",
    "position=< 6, 10> velocity=<-2, -1>",
    "position=< 2, -4> velocity=< 2,  2>",
    "position=<-6, 10> velocity=< 2, -2>",
    "position=< 1,  8> velocity=< 1, -1>",
    "position=< 1,  7> velocity=< 1,  0>",
    "position=<-3, 11> velocity=< 1, -2>",
    "position=< 7,  6> velocity=<-1, -1>",
    "position=<-2,  3> velocity=< 1,  0>",
    "position=<-4,  3> velocity=< 2,  0>",
    "position=<10, -3> velocity=<-1,  1>",
    "position=< 5, 11> velocity=< 1, -2>",
    "position=< 4,  7> velocity=< 0, -1>",
    "position=< 8, -2> velocity=< 0,  1>",
    "position=<15,  0> velocity=<-2,  0>",
    "position=< 1,  6> velocity=< 1,  0>",
    "position=< 8,  9> velocity=< 0, -1>",
    "position=< 3,  3> velocity=<-1,  1>",
    "position=< 0,  5> velocity=< 0, -1>",
    "position=<-2,  2> velocity=< 2,  0>",
    "position=< 5, -2> velocity=< 1,  2>",
    "position=< 1,  4> velocity=< 2,  1>",
    "position=<-2,  7> velocity=< 2, -2>",
    "position=< 3,  6> velocity=<-1, -1>",
    "position=< 5,  0> velocity=< 1,  0>",
    "position=<-6,  0> velocity=< 2,  0>",
    "position=< 5,  9> velocity=< 1, -2>",
    "position=<14,  7> velocity=<-2,  0>",
    "position=<-3,  6> velocity=< 2, -1>",
];

#[test]
fn test_parser() {
    let parsed: Vec<Star> = EXAMPLE.iter().map(|&x| parse(x)).collect();
    assert_eq!(
        parsed,
        vec![
            Star {
                position: Point(9, 1),
                velocity: Point(0, 2)
            },
            Star {
                position: Point(7, 0),
                velocity: Point(-1, 0)
            },
            Star {
                position: Point(3, -2),
                velocity: Point(-1, 1)
            },
            Star {
                position: Point(6, 10),
                velocity: Point(-2, -1)
            },
            Star {
                position: Point(2, -4),
                velocity: Point(2, 2)
            },
            Star {
                position: Point(-6, 10),
                velocity: Point(2, -2)
            },
            Star {
                position: Point(1, 8),
                velocity: Point(1, -1)
            },
            Star {
                position: Point(1, 7),
                velocity: Point(1, 0)
            },
            Star {
                position: Point(-3, 11),
                velocity: Point(1, -2)
            },
            Star {
                position: Point(7, 6),
                velocity: Point(-1, -1)
            },
            Star {
                position: Point(-2, 3),
                velocity: Point(1, 0)
            },
            Star {
                position: Point(-4, 3),
                velocity: Point(2, 0)
            },
            Star {
                position: Point(10, -3),
                velocity: Point(-1, 1)
            },
            Star {
                position: Point(5, 11),
                velocity: Point(1, -2)
            },
            Star {
                position: Point(4, 7),
                velocity: Point(0, -1)
            },
            Star {
                position: Point(8, -2),
                velocity: Point(0, 1)
            },
            Star {
                position: Point(15, 0),
                velocity: Point(-2, 0)
            },
            Star {
                position: Point(1, 6),
                velocity: Point(1, 0)
            },
            Star {
                position: Point(8, 9),
                velocity: Point(0, -1)
            },
            Star {
                position: Point(3, 3),
                velocity: Point(-1, 1)
            },
            Star {
                position: Point(0, 5),
                velocity: Point(0, -1)
            },
            Star {
                position: Point(-2, 2),
                velocity: Point(2, 0)
            },
            Star {
                position: Point(5, -2),
                velocity: Point(1, 2)
            },
            Star {
                position: Point(1, 4),
                velocity: Point(2, 1)
            },
            Star {
                position: Point(-2, 7),
                velocity: Point(2, -2)
            },
            Star {
                position: Point(3, 6),
                velocity: Point(-1, -1)
            },
            Star {
                position: Point(5, 0),
                velocity: Point(1, 0)
            },
            Star {
                position: Point(-6, 0),
                velocity: Point(2, 0)
            },
            Star {
                position: Point(5, 9),
                velocity: Point(1, -2)
            },
            Star {
                position: Point(14, 7),
                velocity: Point(-2, 0)
            },
            Star {
                position: Point(-3, 6),
                velocity: Point(2, -1)
            },
        ]
    );
    assert_eq!(Point(1, 2) + Point(3, 4), Point(4, 6));
}
