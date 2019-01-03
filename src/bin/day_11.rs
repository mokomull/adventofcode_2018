fn power_level(serial: usize, x: usize, y: usize) -> isize {
    let rack_id = x + 10;
    let starting_power_level = rack_id * y;
    let power_level = starting_power_level + serial;
    let power_level = power_level * rack_id;
    let digit = (power_level / 100) % 10;

    digit as isize - 5
}

#[test]
fn example() {
    assert_eq!(power_level(8, 3, 5), 4);
    assert_eq!(power_level(57, 122, 79), -5);
    assert_eq!(power_level(39, 217, 196), 0);
    assert_eq!(power_level(71, 101, 153), 4);
}
