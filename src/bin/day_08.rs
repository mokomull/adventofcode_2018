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

fn parse(input: &[u8]) -> Vec<usize> {
    input
        .split(|&i| i == b' ')
        .map(|i| std::str::from_utf8(i).unwrap().parse::<usize>().unwrap())
        .collect()
}

#[test]
fn examples() {
    let input = parse(b"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(sum_all_metadata(&input), 138);
}
