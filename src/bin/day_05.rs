fn remove_one(input: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let result = input.iter().fold(Vec::new(), |mut s: Vec<u8>, i| {
        match s.last() {
            Some(&x)
                if (x.is_ascii_lowercase()
                    && i.is_ascii_uppercase()
                    && x == i.to_ascii_lowercase())
                    || (x.is_ascii_uppercase()
                        && i.is_ascii_lowercase()
                        && x == i.to_ascii_uppercase()) =>
            {
                s.pop();
            }
            _ => s.push(*i),
        }
        s
    });

    if result.len() == input.len() {
        Err(result)
    } else {
        Ok(result)
    }
}

fn remove_all(input: &str) -> String {
    let mut result = Ok(input.bytes().collect::<Vec<_>>());

    while let Ok(x) = result {
        result = remove_one(&x);
    }

    String::from_utf8(result.unwrap_err()).unwrap()
}

fn main() {
    let stdin = std::io::stdin();
    let mut lock = stdin.lock();

    let mut input = Vec::new();
    std::io::Read::read_to_end(&mut lock, &mut input).expect("read from stdin failed");

    let result = remove_all(std::str::from_utf8(&input).unwrap().trim_end());
    println!("{} units left", result.len());
}

#[test]
fn example() {
    assert_eq!(remove_all("dabAcCaCBAcCcaDA"), "dabCBAcaDA");
}
