#[macro_use]
extern crate nom;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        call!(nom::line_ending) >>
        call!(nom::line_ending) >>
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
            *state.get((i as usize).wrapping_sub(2)).unwrap_or(&Empty),
            *state.get((i as usize).wrapping_sub(1)).unwrap_or(&Empty),
            *state.get((i) as usize).unwrap_or(&Empty),
            *state.get((i + 1) as usize).unwrap_or(&Empty),
            *state.get((i + 2) as usize).unwrap_or(&Empty),
        ];
        for pattern in next_generation {
            if window == *pattern {
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
    use std::collections::HashMap;
    let mut state = initial_state.clone();
    let mut shift = 0;
    let mut seen_states: HashMap<Vec<Pot>, (usize, isize)> = HashMap::new();

    for i in 0..generations {
        // shrink the state to ..#[.....]#.. -- we need the two Empty cells on
        // either end, because we may create a plant "before" the row of pots.
        while state.front() == Some(&Empty) {
            state.pop_front();
            shift -= 1;
        }
        state.push_front(Empty);
        state.push_front(Empty);
        shift += 2;

        while state.back() == Some(&Empty) {
            state.pop_back();
        }
        state.push_back(Empty);
        state.push_back(Empty);

        if let Some((old_i, old_shift)) =
            seen_states.insert(state.iter().cloned().collect(), (i, shift))
        {
            let cycle_length = i - old_i;
            if (generations - i) % cycle_length == 0 {
                shift += (shift - old_shift) * ((generations - i) / cycle_length) as isize;
                break;
            }
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

fn main() {
    use std::io::Read;
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();

    let mut input = Vec::new();
    lock.read_to_end(&mut input).expect("read from stdin");

    let (_rest, (parsed_initial, parsed_rules)) =
        input_file(nom::types::CompleteByteSlice(&input)).unwrap();
    let state = parsed_initial.iter().cloned().collect();
    println!(
        "Sum of indices is {}",
        sum_indices_after(&state, &parsed_rules, 20)
    );

    println!(
        "50 billionth generation: {}",
        sum_indices_after(&state, &parsed_rules, 50_000_000_000)
    );
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
..... => .
####. => #",
    ))
    .unwrap()
    .1;
    let parsed_state = parsed_initial.iter().cloned().collect();
    assert_eq!(sum_indices_after(&parsed_state, &parsed_rules, 20), 325)
}
