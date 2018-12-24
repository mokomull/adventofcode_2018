fn for_all_metadata<F>(tree: &[usize], f: &mut F) -> usize
where
    F: FnMut(usize) -> (),
{
    let children = tree[0];
    let metadata = tree[1];
    /* initially, skip the header */
    let mut index = 2;

    for _ in 0..children {
        index += for_all_metadata(&tree[index..], f);
    }

    for _ in 0..metadata {
        f(tree[index]);
        index += 1;
    }

    index
}

fn sum_all_metadata(tree: &[usize]) -> usize {
    let mut sum = 0;
    for_all_metadata(tree, &mut |i: usize| {
        sum += i;
    });
    sum
}

fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn main() {
    use std::io::Read;
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();

    let mut v = vec![];
    lock.read_to_end(&mut v).unwrap();
    let raw_input = String::from_utf8(v).unwrap();
    let input = parse(raw_input.trim());

    print!("Sum is {}", sum_all_metadata(&input));
}

#[test]
fn examples() {
    let input = parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(sum_all_metadata(&input), 138);
}
