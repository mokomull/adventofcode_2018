fn highest_score(players: usize, last_marble: usize) -> usize {
    let mut scores = vec![0; players];
    let mut marbles = vec![0];
    let mut current = 0;
    let mut player = 0;

    for i in 1..=last_marble {
        if i % 23 == 0 {
            current = (current + marbles.len() - 7) % marbles.len();
            scores[player] += i;
            scores[player] += marbles.remove(current);
            current %= marbles.len();
        } else {
            current = (current + 2) % marbles.len();
            marbles.insert(current, i);
        }
        player = (player + 1) % players;
    }

    *scores.iter().max().unwrap()
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
