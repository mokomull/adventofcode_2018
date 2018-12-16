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
            println!("{}, {} => {:?}", x, y, distances);

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

#[test]
fn example_place() {
    let input = vec![(1, 1), (1, 6), (8, 3), (3, 4), (5, 5), (8, 9)];
    assert_eq!(
        place(&input),
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
    )
}
