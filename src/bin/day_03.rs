#[macro_use]
extern crate nom;

#[derive(Debug, PartialEq)]
struct Claim {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

fn parse_claim(input: &str) -> Option<Claim> {
    use nom::digit;
    let parsed = ws!(
        nom::types::CompleteStr(input),
        do_parse!(
            tag!("#")
                >> id: digit
                >> tag!("@")
                >> left: digit
                >> tag!(",")
                >> top: digit
                >> tag!(":")
                >> width: digit
                >> tag!("x")
                >> height: dbg_dmp!(digit)
                >> (Claim {
                    id: id.parse::<usize>().expect("parse digits"),
                    left: left.parse::<usize>().expect("parse digits"),
                    top: top.parse::<usize>().expect("parse digits"),
                    width: width.parse::<usize>().expect("parse digits"),
                    height: height.parse::<usize>().expect("parse digits")
                })
        )
    );
    println!("{:?}", parsed);
    parsed.ok().map(|(_rest, result)| result)
}

#[test]
fn test_parse() {
    assert_eq!(
        parse_claim("#1 @ 1,3: 4x4 "),
        Some(Claim {
            id: 1,
            left: 1,
            top: 3,
            width: 4,
            height: 4
        })
    );
    assert_eq!(
        parse_claim("#2 @ 3,1: 4x4"),
        Some(Claim {
            id: 2,
            left: 3,
            top: 1,
            width: 4,
            height: 4
        })
    );
    assert_eq!(
        parse_claim("#3 @ 5,5: 2x2"),
        Some(Claim {
            id: 3,
            left: 5,
            top: 5,
            width: 2,
            height: 2
        })
    );
}
