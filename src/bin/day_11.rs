fn power_level(serial: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let starting_power_level = rack_id * y;
    let power_level = starting_power_level + serial;
    let power_level = power_level * rack_id;
    let digit = (power_level / 100) % 10;

    digit as isize - 5
}

fn find_largest(serial: usize) -> (usize, usize) {
    use itertools::Itertools;

    (1..=297)
        .cartesian_product(1..=297)
        .map(|(x, y)| {
            let power = (x..x + 3)
                .cartesian_product(y..y + 3)
                .map(|(ix, iy)| power_level(serial, ix, iy))
                .sum::<isize>();
            ((x, y), power)
        })
        .max_by_key(|&(_, power)| power)
        .unwrap()
        .0
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let serial = input.parse::<usize>().unwrap();
    let largest = find_largest(serial);

    println!("The largest total power is at {},{}", largest.0, largest.1);
}

#[test]
fn example() {
    assert_eq!(power_level(8, 3, 5), 4);
    assert_eq!(power_level(57, 122, 79), -5);
    assert_eq!(power_level(39, 217, 196), 0);
    assert_eq!(power_level(71, 101, 153), 4);

    assert_eq!(find_largest(18), (33, 45));
    assert_eq!(find_largest(42), (21, 61));
}
