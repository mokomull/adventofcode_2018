fn power_level(serial: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let starting_power_level = rack_id * y;
    let power_level = starting_power_level + serial;
    let power_level = power_level * rack_id;
    let digit = (power_level / 100) % 10;

    digit as isize - 5
}

fn find_largest(serial: usize, size: usize) -> ((usize, usize), isize) {
    use itertools::Itertools;

    (1..=301 - size)
        .cartesian_product(1..=301 - size)
        .map(|(x, y)| {
            let power = (x..x + size)
                .cartesian_product(y..y + size)
                .map(|(ix, iy)| power_level(serial, ix, iy))
                .sum::<isize>();
            ((x, y), power)
        })
        .max_by_key(|&(_, power)| power)
        .unwrap()
}

fn find_largest_variable(serial: usize) -> (usize, usize, usize) {
    use rayon::prelude::*;

    (1usize..301)
        .into_par_iter()
        .map(|size| {
            let ((x, y), power) = find_largest(serial, size);
            ((x, y, size), power)
        })
        .max_by_key(|&(_, power)| power)
        .unwrap()
        .0
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let serial = input.parse::<usize>().unwrap();
    let (largest, _power) = find_largest(serial, 3);

    println!("The largest total power is at {},{}", largest.0, largest.1);

    let (x, y, size) = find_largest_variable(serial);
    println!(
        "The largest power with variable size is at {},{},{}",
        x, y, size
    );
}

#[test]
fn example() {
    assert_eq!(power_level(8, 3, 5), 4);
    assert_eq!(power_level(57, 122, 79), -5);
    assert_eq!(power_level(39, 217, 196), 0);
    assert_eq!(power_level(71, 101, 153), 4);

    assert_eq!(find_largest(18, 3).0, (33, 45));
    assert_eq!(find_largest(42, 3).0, (21, 61));

    assert_eq!(find_largest_variable(18), (90, 269, 16));
    assert_eq!(find_largest_variable(42), (232, 251, 12));
}
