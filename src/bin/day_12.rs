#[macro_use]
extern crate nom;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pot {
    Plant,
    Empty,
}

use self::Pot::*;

type Pattern = [Pot; 5];

named!(pot(nom::types::CompleteByteSlice) -> Pot,
    alt!(
        do_parse!(tag!(&b"#"[..]) >> (Plant)) |
        do_parse!(tag!(&b"."[..]) >> (Empty))
    )
);

named!(rule(nom::types::CompleteByteSlice) -> Option<Pattern>,
    do_parse!(
        p0: pot >>
        p1: pot >>
        p2: pot >>
        p3: pot >>
        p4: pot >>
        tag!(&b" => "[..]) >>
        result: pot >>
        (if result == Plant {
            Some([p0, p1, p2, p3, p4])
        } else {
            None
        })
    )
);

named!(input_file(nom::types::CompleteByteSlice) -> (Vec<Pot>, Vec<Pattern>),
    do_parse!(
        tag!("initial state: ") >>
        initial_state: many1!(pot) >>
        tag!("\n\n") >>
        rules: ws!(many1!(rule)) >>
        (
            initial_state,
            rules.iter().filter_map(|&x| x).collect()
        )
    )
);

type State = std::collections::VecDeque<Pot>;

fn advance(state: &State, next_generation: &[Pattern]) -> State {
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

fn sum_indices_after(
    initial_state: &State,
    next_generation: &[Pattern],
    generations: usize,
) -> isize {
    let mut state = initial_state.clone();
    let mut shift = 0;

    for _ in 0..generations {
        while state[0] == Plant || state[1] == Plant {
            state.push_front(Empty);
            shift += 1;
        }
        while state[state.len() - 1] == Plant || state[state.len() - 2] == Plant {
            state.push_back(Empty);
        }
        state = advance(&state, next_generation);
    }

    let mut result = 0;

    for (i, &pot) in state.iter().enumerate() {
        if pot == Plant {
            result += i as isize - shift;
        }
    }

    result
}

#[test]
fn examples() {
    let initial_state: State = [
        Plant, Empty, Empty, Plant, Empty, Plant, Empty, Empty, Plant, Plant, Empty, Empty, Empty,
        Empty, Empty, Empty, Plant, Plant, Plant, Empty, Empty, Empty, Plant, Plant, Plant,
    ]
    .iter()
    .cloned()
    .collect();

    let mut extended = initial_state.clone();

    for _ in 0..3 {
        extended.push_front(Empty);
    }
    for _ in 0..11 {
        extended.push_back(Empty);
    }

    assert!(extended.iter().eq([
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

    let state = advance(&extended, &plants_to_keep);

    assert_eq!(
        state.iter().cloned().collect::<Vec<Pot>>(),
        &[
            Empty, Empty, Empty, Plant, Empty, Empty, Empty, Plant, Empty, Empty, Empty, Empty,
            Plant, Empty, Empty, Empty, Empty, Empty, Plant, Empty, Empty, Plant, Empty, Empty,
            Plant, Empty, Empty, Plant, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
            Empty, Empty, Empty,
        ][..]
    );

    assert_eq!(sum_indices_after(&initial_state, &plants_to_keep, 20), 325);

    let (parsed_initial, parsed_rules) = input_file(nom::types::CompleteByteSlice(
        b"initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #",
    ))
    .unwrap()
    .1;
    let parsed_state = parsed_initial.iter().cloned().collect();
    assert_eq!(sum_indices_after(&parsed_state, &parsed_rules, 20), 325)
}
