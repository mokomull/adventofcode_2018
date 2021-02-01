use nom::{alt, tag, IResult};

#[derive(Debug, Clone, Eq, PartialEq)]
enum Acre {
    Open,
    Trees,
    Lumberyard,
}

fn acre(input: &[u8]) -> IResult<&[u8], Acre> {
    alt!(
        input,
        tag!(b".") => { |_| Acre::Open } |
        tag!(b"|") => { |_| Acre::Trees } |
        tag!(b"#") => { |_| Acre::Lumberyard }
    )
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_acre() {
        assert_eq!(acre(b"."), Ok((&b""[..], Acre::Open)));
        assert!(acre(b"l").is_err());
    }
}
