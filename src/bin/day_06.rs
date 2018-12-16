use std::cmp::max;

fn place(input: &[(usize, usize)]) -> Vec<Vec<Option<usize>>> {
    let (max_x, max_y) = input
        .iter()
        .fold((0, 0), |(old_max_x, old_max_y), &(x, y)| {
            (max(x, old_max_x), max(y, old_max_y))
        });

    let mut board: Vec<_> = (0..=max_y).map(|_| vec![None; max_x + 1]).collect();

    for x in 0..=max_x {
        for y in 0..=max_y {
            let mut distances: Vec<_> = input
                .iter()
                .map(|&(other_x, other_y)| {
                    (other_x as isize - x as isize).abs() + (other_y as isize - y as isize).abs()
                })
                .enumerate()
                .collect();
            distances.sort_by_key(|&(_, distance)| distance);

            if distances.len() > 1 && distances[0].1 == distances[1].1 {
                continue;
            }

            if distances.len() > 0 {
                board[y][x] = Some(distances[0].0);
            }
        }
    }

    board
}

fn largest_area(placed: &Vec<Vec<Option<usize>>>) -> usize {
    let mut areas = std::collections::HashMap::new();

    for col in placed {
        for cell in col {
            *areas.entry(cell).or_insert(0) += 1;
        }
    }

    // Remove all areas that are on the edges of the board -- these are the ones that extend to infinity.
    for cell in placed.first().unwrap_or(&vec![]) {
        areas.remove(cell);
    }

    for cell in placed.last().unwrap_or(&vec![]) {
        areas.remove(cell);
    }

    for col in placed {
        if let Some(cell) = col.first() {
            areas.remove(cell);
        }
        if let Some(cell) = col.last() {
            areas.remove(cell);
        }
    }

    *areas.values().max().unwrap_or(&0)
}

fn within_limit(input: &[(usize, usize)], limit: usize) -> usize {
    let (max_x, max_y) = input
        .iter()
        .fold((0, 0), |(old_max_x, old_max_y), &(x, y)| {
            (max(x, old_max_x), max(y, old_max_y))
        });

    let mut count = 0;

    for x in 0..=max_x {
        for y in 0..=max_y {
            let total: isize = input
                .iter()
                .map(|&(other_x, other_y)| {
                    (other_x as isize - x as isize).abs() + (other_y as isize - y as isize).abs()
                })
                .sum();
            if (total as usize) < limit {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let input: Vec<(usize, usize)> = lock
        .lines()
        .map(|line| {
            let l = line.unwrap();
            (
                l.split(",").nth(0).unwrap().parse::<usize>().unwrap(),
                l.split(" ").nth(1).unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let placed = place(&input);
    println!("Area: {}", largest_area(&placed));
}

#[test]
fn example_place() {
    let input = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
    let placed = place(&input);
    assert_eq!(
        placed,
        vec![
            vec![
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                None,
                Some(2),
                Some(2),
                Some(2)
            ],
            vec![
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                Some(0),
                None,
                Some(2),
                Some(2),
                Some(2)
            ],
            vec![
                Some(0),
                Some(0),
                Some(0),
                Some(3),
                Some(3),
                Some(4),
                Some(2),
                Some(2),
                Some(2)
            ],
            vec![
                Some(0),
                Some(0),
                Some(3),
                Some(3),
                Some(3),
                Some(4),
                Some(2),
                Some(2),
                Some(2)
            ],
            vec![
                None,
                None,
                Some(3),
                Some(3),
                Some(3),
                Some(4),
                Some(4),
                Some(2),
                Some(2)
            ],
            vec![
                Some(1),
                Some(1),
                None,
                Some(3),
                Some(4),
                Some(4),
                Some(4),
                Some(4),
                Some(2)
            ],
            vec![
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(4),
                Some(4),
                Some(4),
                Some(4),
                None
            ],
            vec![
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(4),
                Some(4),
                Some(4),
                Some(5),
                Some(5)
            ],
            vec![
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(4),
                Some(4),
                Some(5),
                Some(5),
                Some(5)
            ],
            vec![
                Some(1),
                Some(1),
                Some(1),
                None,
                Some(5),
                Some(5),
                Some(5),
                Some(5),
                Some(5)
            ]
        ]
    );

    assert_eq!(largest_area(&placed), 17);

    assert_eq!(within_limit(&input, 32), 16);
}
