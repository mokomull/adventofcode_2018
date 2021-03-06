#[macro_use]
extern crate nom;

#[macro_use]
extern crate log;

use std::collections::{BTreeSet, HashMap};
use std::io::Read;

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
                opt!(call!(nom::line_ending)) >>
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
        x => panic!("Cell {}, {} was {:?}, neither Goblin nor Elf", row, col, x),
    }
}

fn next_step(board: &[Vec<Unit>], position: (usize, usize)) -> Option<Direction> {
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
        .min_by_key(|position| predecessors.get(position).expect("just filtered").1)?;

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

    direction
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

    let mut update_predecessor = || -> bool {
        let previous_distance = predecessors
            .get(&(row, col))
            .map(|&(_, d)| d)
            .unwrap_or(std::usize::MAX);
        if distance + 1 < previous_distance {
            predecessors.insert((row, col), (step_direction, distance + 1));
            return true;
        }
        false
    };

    match cell {
        x if std::mem::discriminant(&x) == enemy => {
            update_predecessor();
            reachable_enemies.insert((row, col));
        }
        Goblin(_) | Elf(_) => {}
        Wall => {}
        Empty => {
            if update_predecessor() {
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
    assert_eq!(next_step(&board, (1, 2)), Some(Right));
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
    .min_by_key(|&(unit, _dir)| match unit {
        Goblin(x) | Elf(x) => x,
        _ => panic!("should have filtered out enemies before we get here"),
    });

    if let Some((_, dir)) = attack {
        return Action::Attack(dir);
    }

    if let Some(dir) = next_step(board, (row, col)) {
        return Action::Move(dir);
    }

    Action::Nothing
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

    /* From a failing case demonstrated in test_run():
        #######
        #..G..# 200
        #...EG# 197 197
        #.#.#G# 200
        #..G#E# 200 197
        #.....#
        #######
        4, 3 decided to Nothing

        The goblin at 4, 3 should have been able to move up.
    */
    let input = b"#######
#..G..#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
    let (_remaining, b) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(next_action(&b, (4, 3)), Action::Move(Up));
}

fn attack<T>(
    board: &mut Vec<Vec<Unit>>,
    attack_power: T,
    (row, col): (usize, usize),
    dir: Direction,
    elves: &mut usize,
    goblins: &mut usize,
) -> Option<()>
where
    T: FnOnce(&Unit, &Unit) -> Option<usize>,
{
    let (other_row, other_col) = match dir {
        Left => (row, col - 1),
        Right => (row, col + 1),
        Down => (row + 1, col),
        Up => (row - 1, col),
    };

    let this_attack = attack_power(&board[row][col], &board[other_row][other_col])?;

    let new_unit = match board[other_row][other_col] {
        Goblin(x) => {
            if x <= this_attack {
                *goblins -= 1;
                Empty
            } else {
                Goblin(x - this_attack)
            }
        }
        Elf(x) => {
            if x <= this_attack {
                *elves -= 1;
                Empty
            } else {
                Elf(x - this_attack)
            }
        }
        something_else => panic!("Tried to attack a {:?}", something_else),
    };

    board[other_row][other_col] = new_unit;
    Some(())
}

fn run_with_attack_power<T>(mut board: Vec<Vec<Unit>>, mut attack_power: T) -> Option<usize>
where
    T: FnMut(&Unit, &Unit) -> Option<usize>,
{
    let mut rounds = 0;

    'round: loop {
        debug!("=== Starting round {}", rounds);

        let mut players = Vec::new();
        let mut goblins = 0;
        let mut elves = 0;
        for (row_num, row) in board.iter().enumerate() {
            for (col_num, unit) in row.iter().enumerate() {
                match unit {
                    Goblin(_) => {
                        goblins += 1;
                        players.push((row_num, col_num));
                    }
                    Elf(_) => {
                        elves += 1;
                        players.push((row_num, col_num));
                    }
                    _ => {}
                }
            }
        }

        for (row, col) in players {
            match board[row][col] {
                // it may have been killed by a previous action in the same round
                Empty => {
                    debug!("skipped {}, {}", row, col);
                    continue;
                }
                Goblin(_) | Elf(_) if goblins == 0 || elves == 0 => {
                    // No opponents left, and the round did not complete.
                    break 'round;
                }
                _ => {}
            }

            let action = next_action(&board, (row, col));
            dump_board(&board, (row, col));
            debug!("{}, {} decided to {:?}", row, col, action);

            match action {
                Action::Move(dir) => {
                    let (new_row, new_col) = match dir {
                        Left => (row, col - 1),
                        Right => (row, col + 1),
                        Down => (row + 1, col),
                        Up => (row - 1, col),
                    };
                    let unit = board[row][col];
                    board[row][col] = Empty;
                    board[new_row][new_col] = unit;

                    if let Action::Attack(dir) = next_action(&board, (new_row, new_col)) {
                        debug!("{}, {} would now attack {:?}", new_row, new_col, dir);
                        attack(
                            &mut board,
                            &mut attack_power,
                            (new_row, new_col),
                            dir,
                            &mut elves,
                            &mut goblins,
                        );
                    } else {
                        debug!(
                            "{}, {} still can't attack anything this round",
                            new_row, new_col
                        );
                    }
                }
                Action::Attack(dir) => attack(
                    &mut board,
                    &mut attack_power,
                    (row, col),
                    dir,
                    &mut elves,
                    &mut goblins,
                )?,
                Action::Nothing => {}
            }
            debug!("");
        }

        rounds += 1;
    }

    let sum_hp: usize = board
        .iter()
        .flat_map(|r| {
            r.iter().map(|&unit| match unit {
                Goblin(x) | Elf(x) => x,
                _ => 0,
            })
        })
        .sum();

    debug!("sum is {}, rounds is {}", sum_hp, rounds);

    Some(rounds * sum_hp)
}

fn run(board: Vec<Vec<Unit>>) -> usize {
    run_with_attack_power(board, |_, _| Some(3)).expect("closure never returns None")
}

#[cfg(test)]
static LOG_INITIALIZED: std::sync::Once = std::sync::Once::new();

#[cfg(test)]
fn do_test_run(input: &[u8], expected: usize) {
    LOG_INITIALIZED.call_once(|| {
        env_logger::Builder::from_default_env()
            .is_test(true)
            .default_format_timestamp(false)
            .init();
    });
    let (remaining, b) = board(CompleteByteSlice(&input[..])).unwrap();
    assert_eq!(remaining, CompleteByteSlice(&b""[..]));
    assert_eq!(run(b), expected);
}

#[test]
fn test_run_step_by_step() {
    // This was the case the problem statement describes move-by-move.
    do_test_run(
        b"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######",
        27730,
    );
}

#[test]
fn test_run_2() {
    do_test_run(
        b"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",
        36334,
    );
}

#[test]
fn test_run_3() {
    do_test_run(
        b"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######",
        39514,
    );
}

#[test]
fn test_run_4() {
    do_test_run(
        b"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######",
        27755,
    );
}

#[test]
fn test_run_5() {
    do_test_run(
        b"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######",
        28944,
    );
}

#[test]
fn test_run_6() {
    do_test_run(
        b"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########",
        18740,
    );
}

fn run_without_killing_elf(board: Vec<Vec<Unit>>) -> usize {
    for elf_attack_power in 4.. {
        let attack_power = |_attacker: &Unit, receiver: &Unit| -> Option<usize> {
            match *receiver {
                Elf(x) if x < 3 => None,
                Elf(_) => Some(3),
                Goblin(_) => Some(elf_attack_power),
                wtf => panic!("Tried to attack a {:?}", wtf),
            }
        };

        if let Some(ret) = run_with_attack_power(board.clone(), attack_power) {
            return ret;
        }
    }
    panic!("No flawless Elf victory was ever observed")
}

#[test]
fn test_run_without_killing_elf() {
    let b = board(
        b"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"[..]
            .into(),
    )
    .unwrap()
    .1;
    assert_eq!(run_without_killing_elf(b), 4988);
}

fn dump_board(board: &[Vec<Unit>], highlight_position: (usize, usize)) {
    for (cur_row, row) in board.iter().enumerate() {
        let mut line = String::new();
        for (cur_col, col) in row.iter().enumerate() {
            let c = match *col {
                Wall => '#',
                Empty => '.',
                Goblin(_) => 'G',
                Elf(_) => 'E',
            };
            if (cur_row, cur_col) == highlight_position {
                line.push_str("\x1b[1m");
                line.push(c);
                line.push_str("\x1b[0m");
            } else {
                line.push(c);
            }
        }

        for col in row {
            match *col {
                Goblin(x) | Elf(x) => line.push_str(&format!(" {}", x)),
                _ => {}
            }
        }

        debug!("{}", line);
    }
}

fn main() {
    env_logger::Builder::from_default_env()
        .default_format_timestamp(false)
        .init();

    let mut buf = Vec::new();
    std::io::stdin()
        .lock()
        .read_to_end(&mut buf)
        .expect("stdin read failed");

    let board = crate::board(CompleteByteSlice(&buf)).unwrap().1;
    println!("Outcome of combat is: {}", run(board.clone()));

    println!(
        "Outcome of combat with no Elf deaths: {}",
        run_without_killing_elf(board)
    )
}
