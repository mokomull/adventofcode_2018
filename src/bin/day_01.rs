use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let data: Vec<_> = lock
        .lines()
        .map(|line| {
            line.expect("file should be readable")
                .parse::<i64>()
                .expect("file should contain integers")
        })
        .collect();

    let frequency: i64 = data.iter().sum();

    println!("Total frequency: {}", frequency);

    let seen_twice = data.iter().cycle().try_fold(
        (std::collections::HashSet::new(), 0),
        |(mut seen, frequency), i| {
            let frequency = frequency + i;
            if !seen.insert(frequency) {
                Err(frequency)
            } else {
                Ok((seen, frequency))
            }
        },
    );

    println!("Seen twice: {}", seen_twice.unwrap_err());
}
