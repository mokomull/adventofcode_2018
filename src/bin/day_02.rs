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
