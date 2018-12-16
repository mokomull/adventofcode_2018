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
}
