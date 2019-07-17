fn for_all_metadata<F, V>(tree: &[usize], f: &mut F) -> (usize, V)
where
    F: FnMut(&[V], &[usize]) -> V,
{
    let children = tree[0];
    let metadata = tree[1];
    /* initially, skip the header */
    let mut index = 2;

    let mut children_values: Vec<V> = vec![];

    for _ in 0..children {
        let (count, value) = for_all_metadata(&tree[index..], f);
        index += count;
        children_values.push(value);
    }

    let value = f(&children_values, &tree[index..index + metadata]);

    index += metadata;

    (index, value)
}

fn sum_all_metadata(tree: &[usize]) -> usize {
    let mut sum = 0;
    for_all_metadata(tree, &mut |_values, i: &[usize]| {
        sum += i.iter().sum::<usize>();
    });
    sum
}

fn calculate_value(tree: &[usize]) -> usize {
    let (_index, value) = for_all_metadata(tree, &mut |children_values, this_metadata| {
        if children_values.is_empty() {
            this_metadata.iter().sum()
        } else {
            this_metadata
                .iter()
                .map(|index| children_values.get(index - 1).unwrap_or(&0))
                .sum()
        }
    });
    value
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

    println!("Sum is {}", sum_all_metadata(&input));
    println!("Value of the root is {}", calculate_value(&input));
}

#[test]
fn examples() {
    let input = parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(sum_all_metadata(&input), 138);
    assert_eq!(calculate_value(&input), 66);
}
