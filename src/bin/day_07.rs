#[macro_use]
extern crate nom;

use std::collections::{BTreeSet, HashSet};

fn parse_input(input: &[u8]) -> (u8, u8) {
    do_parse!(
        nom::types::CompleteByteSlice(input),
        tag!(&b"Step "[..])
            >> from: take!(1)
            >> tag!(&b" must be finished before step "[..])
            >> to: take!(1)
            >> tag!(&b" can begin."[..])
            >> (from[0], to[0])
    )
    .unwrap()
    .1
}

fn all_nodes(edges: &[(u8, u8)]) -> BTreeSet<u8> {
    edges
        .iter()
        .flat_map(|&(from, to)| vec![from, to])
        .collect()
}

fn topological_sort(edges: &[(u8, u8)]) -> Vec<u8> {
    let mut to_visit = all_nodes(edges);
    let mut completed: HashSet<u8> = HashSet::new();
    let mut order: Vec<u8> = Vec::new();

    while !to_visit.is_empty() {
        let this = to_visit
            .iter()
            .filter(|&&v| {
                edges
                    .iter()
                    .all(|&(from, to)| to != v || completed.contains(&from))
            })
            .next()
            .unwrap()
            .clone();
        completed.insert(this);
        order.push(this);
        to_visit.remove(&this);
    }

    order
}

fn count_ticks(edges: &[(u8, u8)], surcharge: usize) -> usize {
    let mut tick = 0;
    let mut deadlines: [(u8, usize); 2] = [(0, 0), (0, 0)];
    let mut to_visit = all_nodes(&edges);
    let mut completed: HashSet<u8> = HashSet::new();

    while !to_visit.is_empty() {
        let available = deadlines
            .iter_mut()
            .filter(move |&&mut (_node, deadline)| deadline <= tick);

        for i in available {
            completed.insert(i.0);
            // must handle to_visit being empty since we may run this loop multiple times, unlike in topological_sort().
            let maybe_this = to_visit
                .iter()
                .filter(|&&v| {
                    edges
                        .iter()
                        .all(|&(from, to)| to != v || completed.contains(&from))
                })
                .next()
                .cloned();
            if let Some(this) = maybe_this {
                let new_deadline = tick + surcharge + (this - b'A') as usize + 1;
                to_visit.remove(&this);
                *i = (this, new_deadline);
            }
        }

        tick += 1;
    }

    // tick is the tick where started the last work item, so we need to compute when it will finish
    deadlines
        .iter()
        .max_by_key(|&(_node, deadline)| deadline)
        .unwrap()
        .1
}

fn main() {
    use std::io::BufRead;
    let stdin = std::io::stdin();
    let lock = stdin.lock();

    let input: Vec<_> = lock
        .lines()
        .map(|l| parse_input(l.unwrap().as_bytes()))
        .collect();
    println!(
        "Order: {}",
        std::str::from_utf8(&topological_sort(&input)).unwrap()
    );
}

#[test]
fn examples() {
    let input = [
        b"Step C must be finished before step A can begin.",
        b"Step C must be finished before step F can begin.",
        b"Step A must be finished before step B can begin.",
        b"Step A must be finished before step D can begin.",
        b"Step B must be finished before step E can begin.",
        b"Step D must be finished before step E can begin.",
        b"Step F must be finished before step E can begin.",
    ];
    let output: Vec<_> = input.iter().map(|&x| parse_input(x)).collect();
    assert_eq!(output[0], (b'C', b'A'));
    assert_eq!(output[1], (b'C', b'F'));

    assert_eq!(topological_sort(&output), b"CABDFE");

    assert_eq!(count_ticks(&output, 0), 15);
}
