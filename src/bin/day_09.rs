#[macro_use]
extern crate nom;

fn highest_score(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    // the first element in marbles is always the "current" marble.
    let mut marbles = std::collections::VecDeque::new();
    marbles.push_back(0);
    let mut player = 0;

    for i in 1..=last_marble {
        if i % 23 == 0 {
            let tail: Vec<usize> = (0..6).filter_map(|_| marbles.pop_back()).collect();
            scores[player] += marbles.pop_back().unwrap();
            scores[player] += i;
            for x in tail {
                marbles.push_front(x);
            }
        } else {
            let head: Vec<usize> = (0..2).filter_map(|_| marbles.pop_front()).collect();
            marbles.push_front(i);
            for x in head {
                marbles.push_back(x);
            }
        }
        player = (player + 1) % players;
    }

    *scores.iter().max().unwrap()
}

fn main() {
    use nom::digit;
    use std::io::BufRead;

    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let input = lock.lines().next().unwrap().unwrap();
    let (players, last_marble) = do_parse!(
        nom::types::CompleteStr(&input),
        players: digit
            >> tag!(&" players; last marble is worth ")
            >> last_marble: digit
            >> tag!(&" points")
            >> ((
                players.parse::<usize>().unwrap(),
                last_marble.parse::<usize>().unwrap()
            ))
    )
    .unwrap()
    .1;

    println!(
        "The highest player scored {}",
        highest_score(players, last_marble)
    );

    println!(
        "The highest player with 100x marbles would score {}",
        highest_score(players, last_marble * 100)
    );
}

#[test]
fn examples() {
    assert_eq!(highest_score(9, 25), 32);
    assert_eq!(highest_score(10, 1618), 8317);
    assert_eq!(highest_score(13, 7999), 146373);
    assert_eq!(highest_score(17, 1104), 2764);
    assert_eq!(highest_score(21, 6111), 54718);
    assert_eq!(highest_score(30, 5807), 37305);
}
