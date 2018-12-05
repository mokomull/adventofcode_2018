use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let mut frequency = 0;

    for line in lock.lines() {
        let i = line
            .expect("file should be readable")
            .parse::<i64>()
            .expect("file should contain integers");
        frequency += i;
    }

    println!("Total frequency: {}", frequency);
}
