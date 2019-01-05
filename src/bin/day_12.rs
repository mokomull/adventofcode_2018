#[derive(Clone, Copy, Debug, PartialEq)]
enum Pot {
    Plant,
    Empty,
}

use self::Pot::*;

type Pattern = [Pot; 5];
type State = std::collections::VecDeque<Pot>;

fn advance(state: State, next_generation: &[Pattern]) -> State {
    let mut result = State::new();

    'cells: for i in 0..state.len() {
        let window = [
            *state.get((i - 2) as usize).unwrap_or(&Empty),
            *state.get((i - 1) as usize).unwrap_or(&Empty),
            *state.get((i) as usize).unwrap_or(&Empty),
            *state.get((i + 1) as usize).unwrap_or(&Empty),
            *state.get((i + 2) as usize).unwrap_or(&Empty),
        ];
        for pattern in next_generation {
            if window == *pattern {
                println!("matched rule {:?} at index {}", *pattern, i);
                result.push_back(Plant);
                continue 'cells;
            }
        }
        result.push_back(Empty);
    }

    result
}

#[test]
fn examples() {
    let mut initial_state: State = [
        Plant, Empty, Empty, Plant, Empty, Plant, Empty, Empty, Plant, Plant, Empty, Empty, Empty,
        Empty, Empty, Empty, Plant, Plant, Plant, Empty, Empty, Empty, Plant, Plant, Plant,
    ]
    .iter()
    .cloned()
    .collect();

    for _ in 0..3 {
        initial_state.push_front(Empty);
    }
    for _ in 0..11 {
        initial_state.push_back(Empty);
    }

    assert!(initial_state.iter().eq([
        Empty, Empty, Empty, Plant, Empty, Empty, Plant, Empty, Plant, Empty, Empty, Plant, Plant,
        Empty, Empty, Empty, Empty, Empty, Empty, Plant, Plant, Plant, Empty, Empty, Empty, Plant,
        Plant, Plant, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
    ]
    .iter()));

    let plants_to_keep = [
        [Empty, Empty, Empty, Plant, Plant],
        [Empty, Empty, Plant, Empty, Empty],
        [Empty, Plant, Empty, Empty, Empty],
        [Empty, Plant, Empty, Plant, Empty],
        [Empty, Plant, Empty, Plant, Plant],
        [Empty, Plant, Plant, Empty, Empty],
        [Empty, Plant, Plant, Plant, Plant],
        [Plant, Empty, Plant, Empty, Plant],
        [Plant, Empty, Plant, Plant, Plant],
        [Plant, Plant, Empty, Plant, Empty],
        [Plant, Plant, Empty, Plant, Plant],
        [Plant, Plant, Plant, Empty, Empty],
        [Plant, Plant, Plant, Empty, Plant],
        [Plant, Plant, Plant, Plant, Empty],
    ];

    let state = advance(initial_state, &plants_to_keep);

    assert_eq!(
        state.iter().cloned().collect::<Vec<Pot>>(),
        &[
            Empty, Empty, Empty, Plant, Empty, Empty, Empty, Plant, Empty, Empty, Empty, Empty,
            Plant, Empty, Empty, Empty, Empty, Empty, Plant, Empty, Empty, Plant, Empty, Empty,
            Plant, Empty, Empty, Plant, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty,
        ][..]
    );
}
