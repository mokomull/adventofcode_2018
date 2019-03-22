#[macro_use]
extern crate nom;

use std::collections::{BTreeSet, HashMap, HashSet};

use nom::types::CompleteByteSlice;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Unit {
    Elf(usize),
    Goblin(usize),
    Wall,
    Empty,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

use self::Direction::*;
use self::Unit::*;

named!(unit(CompleteByteSlice) -> Unit,
    alt!(
        do_parse!(tag!(&b"#"[..]) >> (Wall)) |
        do_parse!(tag!(&b"E"[..]) >> (Elf(200))) |
        do_parse!(tag!(&b"G"[..]) >> (Goblin(200))) |
        do_parse!(tag!(&b"."[..]) >> (Empty))
    )
);

named!(board(CompleteByteSlice) -> Vec<Vec<Unit>>,
    do_parse!(
        rows: many1!(
            do_parse!(
                row: many1!(unit) >>
                opt!(tag!(&b"\n"[..])) >>
                (row)
            )
        ) >>
        (rows)
    )
);

#[test]
fn test_parser() {
    let input = b"#######
#E..G.#
#...#.#
#.G.#G#
#######";
    let (remaining, board) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(remaining, CompleteByteSlice(&b""[..]));
    assert_eq!(
        board,
        vec![
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall],
            vec![Wall, Elf(200), Empty, Empty, Goblin(200), Empty, Wall],
            vec![Wall, Empty, Empty, Empty, Wall, Empty, Wall],
            vec![Wall, Empty, Goblin(200), Empty, Wall, Goblin(200), Wall],
            vec![Wall, Wall, Wall, Wall, Wall, Wall, Wall],
        ]
    );
}

fn get_enemy(board: &[Vec<Unit>], (row, col): (usize, usize)) -> std::mem::Discriminant<Unit> {
    match board.get(row).and_then(|r| r.get(col)) {
        Some(Goblin(_)) => std::mem::discriminant(&Elf(0)),
        Some(Elf(_)) => std::mem::discriminant(&Goblin(0)),
        _ => panic!("Cell {}, {} was neither Goblin nor Elf", row, col),
    }
}

fn next_step(board: &[Vec<Unit>], position: (usize, usize)) -> Direction {
    let enemy = get_enemy(board, position);

    // We want to move to the target that is first in the reading order, so we'll scan for the
    // reachable squares in reading order too.  Note that an enemy's "up" square is always going to
    // be first in "reading order".
    let mut reachable_enemies = BTreeSet::new();
    let mut predecessors = HashMap::<(usize, usize), (Direction, usize)>::new();
    let mut to_visit = BTreeSet::new();
    to_visit.insert((position, 0));

    while !to_visit.is_empty() {
        let entry = *to_visit.iter().next().expect("to_visit is not empty");
        let ((row, col), distance) = entry;
        to_visit.remove(&entry);

        next_step_visit(
            board,
            row.wrapping_sub(1),
            col,
            distance,
            enemy,
            &mut to_visit,
            &mut predecessors,
            Up,
            &mut reachable_enemies,
        );
        next_step_visit(
            board,
            row,
            col.wrapping_sub(1),
            distance,
            enemy,
            &mut to_visit,
            &mut predecessors,
            Left,
            &mut reachable_enemies,
        );
        next_step_visit(
            board,
            row,
            col + 1,
            distance,
            enemy,
            &mut to_visit,
            &mut predecessors,
            Right,
            &mut reachable_enemies,
        );
        next_step_visit(
            board,
            row + 1,
            col,
            distance,
            enemy,
            &mut to_visit,
            &mut predecessors,
            Down,
            &mut reachable_enemies,
        );
    }

    let adjacent_to_enemy = reachable_enemies
        .iter()
        .flat_map(|&(row, col)| {
            vec![
                (row.wrapping_sub(1), col),
                (row, col.wrapping_sub(1)),
                (row, col + 1),
                (row + 1, col),
            ]
        })
        .filter(|position| predecessors.contains_key(position))
        // TODO: I might need to actually emit _all_ of the ones of the same distance, rather than
        // the first one I come to.
        .min_by_key(|position| predecessors.get(position).expect("just filtered").1)
        .expect("at least one enemy should be reachable");

    // Walk the path to determine the original step
    let (mut row, mut col) = adjacent_to_enemy;
    let mut direction = None;
    while (row, col) != position {
        direction = Some(predecessors.get(&(row, col)).unwrap().0);
        let predecessor_position = match direction.unwrap() {
            Up => (row + 1, col),
            Left => (row, col + 1),
            Right => (row, col.wrapping_sub(1)),
            Down => (row.wrapping_sub(1), col),
        };

        row = predecessor_position.0;
        col = predecessor_position.1;
    }

    direction.expect("there is a next step")
}

fn next_step_visit(
    board: &[Vec<Unit>],
    row: usize,
    col: usize,
    distance: usize,
    enemy: std::mem::Discriminant<Unit>,
    to_visit: &mut BTreeSet<((usize, usize), usize)>,
    predecessors: &mut HashMap<(usize, usize), (Direction, usize)>,
    step_direction: Direction,
    reachable_enemies: &mut BTreeSet<(usize, usize)>,
) {
    let cell = *board.get(row).and_then(|r| r.get(col)).unwrap_or(&Wall);
    match cell {
        x if std::mem::discriminant(&x) == enemy => {
            reachable_enemies.insert((row - 1, col));
        }
        Goblin(_) | Elf(_) => {}
        Wall => {}
        Empty => {
            let previous_distance = predecessors
                .get(&(row, col))
                .map(|&(_, d)| d)
                .unwrap_or(std::usize::MAX);
            if distance + 1 < previous_distance {
                predecessors.insert((row, col), (step_direction, distance + 1));
                to_visit.insert(((row, col), distance + 1));
            }
        }
    }
}

#[test]
fn test_next_step() {
    let input = b"#######
#.E...#
#.....#
#...G.#
#######";
    let (_remaining, board) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(next_step(&board, (1, 2)), Right);
}

#[derive(Debug, PartialEq)]
enum Action {
    Move(Direction),
    Attack(Direction),
    Nothing,
}

fn next_action(board: &[Vec<Unit>], (row, col): (usize, usize)) -> Action {
    let enemy = get_enemy(board, (row, col));

    let attack = [
        (row.wrapping_sub(1), col, Up),
        (row, col.wrapping_sub(1), Left),
        (row, col + 1, Right),
        (row + 1, col, Down),
    ]
    .iter()
    .cloned()
    .filter_map(|(other_row, other_col, dir)| {
        board
            .get(other_row)
            .and_then(|r| r.get(other_col))
            .filter(|&unit| std::mem::discriminant(unit) == enemy)
            .map(|&unit| (unit, dir))
    })
    .min_by_key(|&(unit, _dir)| {
        match unit {
            Goblin(x) | Elf(x) => x,
            _ => panic!("should have filtered out enemies before we get here")
        }
    });

    if let Some((_, dir)) = attack {
        return Action::Attack(dir);
    }

    Action::Move(next_step(board, (row, col)))
}

#[test]
fn test_next_action() {
    let input = b"#######
#.E...#
#.....#
#...G.#
#######";
    let (_remaining, b) = board(CompleteByteSlice(&input[..])).unwrap();

    assert_eq!(next_action(&b, (1, 2)), Action::Move(Right));

    let input = b"#########
#.......#
#..GGG..#
#..GEG..#
#G..G...#
#......G#
#.......#
#.......#
#########";
    let (_remaining, b) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(next_action(&b, (3, 3)), Action::Attack(Right));

    let input = b"#########
#.......#
#..GGG..#
#..G.G..#
#G..G...#
#......G#
#.......#
#.......#
#########";
    let (_remaining, b) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(next_action(&b, (3, 3)), Action::Nothing);
}
