use std::io::BufRead;

#[derive(Debug, PartialEq)]
struct Letters {
    twice: Option<u8>,
    thrice: Option<u8>,
}

fn count(input: &[u8]) -> Letters {
    let mut counts = [0; 256];

    for i in input {
        counts[*i as usize] += 1;
    }

    Letters {
        twice: counts.iter().position(|&i| i == 2).map(|i| i as u8),
        thrice: counts.iter().position(|&i| i == 3).map(|i| i as u8),
    }
}

fn checksum<I: AsRef<[u8]>, T: Iterator<Item = I>>(input: T) -> usize {
    let mut twice = 0;
    let mut thrice = 0;

    for i in input {
        let c = count(i.as_ref());
        if c.twice.is_some() {
            twice += 1;
        }
        if c.thrice.is_some() {
            thrice += 1;
        }
    }

    return twice * thrice;
}

fn find_differ_by_one<S: AsRef<[u8]>, T: AsRef<[S]>>(input: &T) -> Option<Vec<u8>> {
    for i in input.as_ref() {
        for j in input.as_ref() {
            /* Since i and j have the type &S, they aren't usable as slices until we force them to
               be &[u8]. */
            let i = i.as_ref();
            let j = j.as_ref();

            if j.len() != i.len() {
                continue;
            }

            let common: Vec<_> = i
                .iter()
                .zip(j)
                .filter_map(|(x, y)| if x == y { Some(*x) } else { None })
                .collect();
            if common.len() == i.len() - 1 {
                return Some(common);
            }
        }
    }

    None
}

fn main() {
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    println!(
        "Checksum: {}",
        checksum(lock.lines().filter_map(|i| i.ok()))
    );
}

#[test]
fn examples() {
    assert_eq!(
        count(b"abcdef"),
        Letters {
            twice: None,
            thrice: None
        }
    );
    assert_eq!(
        count(b"bababc"),
        Letters {
            twice: Some(b'a'),
            thrice: Some(b'b')
        }
    );
    assert_eq!(
        count(b"abbcde"),
        Letters {
            twice: Some(b'b'),
            thrice: None
        }
    );
    assert_eq!(
        count(b"abcccd"),
        Letters {
            twice: None,
            thrice: Some(b'c')
        }
    );
    assert_eq!(
        count(b"aabcdd"),
        Letters {
            twice: Some(b'a'),
            thrice: None
        }
    );
    assert_eq!(
        count(b"abcdee"),
        Letters {
            twice: Some(b'e'),
            thrice: None
        }
    );
    assert_eq!(
        count(b"ababab"),
        Letters {
            twice: None,
            thrice: Some(b'a')
        }
    );

    assert_eq!(
        checksum(
            vec![b"abcdef", b"bababc", b"abbcde", b"abcccd", b"aabcdd", b"abcdee", b"ababab"]
                .iter()
        ),
        12
    );
}

#[test]
fn examples_2() {
    assert_eq!(
        find_differ_by_one(&vec![
            b"abcde", b"fghij", b"klmno", b"pqrst", b"fguij", b"axcye", b"wvxyz"
        ]),
        Some(b"fgij".to_vec())
    );
}
