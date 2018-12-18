#[macro_use]
extern crate nom;

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

fn topological_sort(edges: &[(u8, u8)]) -> Vec<u8> {
    use std::collections::{BTreeSet, HashSet};
    let mut to_visit: BTreeSet<u8> = edges
        .iter()
        .flat_map(|&(from, to)| vec![from, to])
        .collect();
    let mut completed: std::collections::HashSet<u8> = HashSet::new();
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
}
