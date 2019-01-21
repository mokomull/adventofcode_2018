fn power_level(serial: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let starting_power_level = rack_id * y;
    let power_level = starting_power_level + serial;
    let power_level = power_level * rack_id;
    let digit = (power_level / 100) % 10;

    digit as isize - 5
}

fn find_largest(partial_sum: &[Vec<isize>], size: usize) -> ((usize, usize), isize) {
    use itertools::Itertools;

    (0..=300 - size)
        .cartesian_product(0..=300 - size)
        .map(|(x, y)| {
            let power = partial_sum[x + size - 1][y + size - 1] // bottom right corner
                - partial_sum[x + size - 1]
                    .get(y.wrapping_sub(1))
                    .unwrap_or(&0) // just above the top right corner
                + partial_sum
                    .get(x.wrapping_sub(1))
                    .and_then(|c| c.get(y.wrapping_sub(1)))
                    .unwrap_or(&0) // above-and-left of top left corner (would be double-counted)
                - partial_sum
                    .get(x.wrapping_sub(1))
                    .and_then(|c| c.get(y + size - 1))
                    .unwrap_or(&0); // just left of bottom left corner
            ((x + 1, y + 1), power)
        })
        .max_by_key(|&(_, power)| power)
        .unwrap()
}

fn find_largest_variable(partial_sum: &[Vec<isize>]) -> (usize, usize, usize) {
    use rayon::prelude::*;

    (1usize..301)
        .into_par_iter()
        .map(|size| {
            let ((x, y), power) = find_largest(partial_sum, size);
            ((x, y, size), power)
        })
        .max_by_key(|&(_, power)| power)
        .unwrap()
        .0
}

// Precomputes the sum of all fuel cells in the square bounded by (1, 1) and
// (x, y), inclusive.
//
// 300 * 300 * sizeof(isize) is too big for the default stack size, so return a
// Vec instead.
fn generate_partial_sums(serial: usize) -> Vec<Vec<isize>> {
    let mut result = vec![vec![0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            result[x][y] = result
                .get(x.wrapping_sub(1))
                .and_then(|c| c.get(y))
                .unwrap_or(&0)
                + result
                    .get(x)
                    .and_then(|c| c.get(y.wrapping_sub(1)))
                    .unwrap_or(&0)
                - result
                    .get(x.wrapping_sub(1))
                    .and_then(|c| c.get(y.wrapping_sub(1)))
                    .unwrap_or(&0)
                + power_level(serial, x + 1, y + 1); // because the formula given is 1-based
        }
    }

    result
}

fn main() {
    let input = std::env::args().nth(1).unwrap();
    let serial = input.parse::<usize>().unwrap();
    let partial_sums = Box::new(generate_partial_sums(serial));
    let (largest, _power) = find_largest(&partial_sums, 3);

    println!("The largest total power is at {},{}", largest.0, largest.1);

    let (x, y, size) = find_largest_variable(&partial_sums);
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

    let serial_18 = Box::new(generate_partial_sums(18));
    let serial_42 = Box::new(generate_partial_sums(42));

    assert_eq!(find_largest(&serial_18, 3).0, (33, 45));
    assert_eq!(find_largest(&serial_42, 3).0, (21, 61));

    assert_eq!(find_largest_variable(&serial_18), (90, 269, 16));
    assert_eq!(find_largest_variable(&serial_42), (232, 251, 12));
}
