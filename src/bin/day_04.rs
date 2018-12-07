#[macro_use]
extern crate nom;

#[derive(Debug, PartialEq)]
enum Action {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}
use Action::*;

#[derive(Debug, PartialEq)]
struct Record {
    date: (usize, usize, usize),
    hour: usize,
    minute: usize,
    action: Action,
}

fn parse_record(input: &str) -> Record {
    use nom::digit;

    ws!(
        nom::types::CompleteStr(input),
        do_parse!(
            tag!("[")
                >> year: digit
                >> tag!("-")
                >> month: digit
                >> tag!("-")
                >> day: digit
                >> hour: digit
                >> tag!(":")
                >> minute: digit
                >> tag!("]")
                >> action:
                    alt!(
                        do_parse!(
                            tag!("Guard #")
                                >> number: digit
                                >> tag!("begins shift")
                                >> (BeginShift(number.parse::<usize>().unwrap()))
                        ) | do_parse!(tag!("falls asleep") >> (FallAsleep))
                            | do_parse!(tag!("wakes up") >> (WakeUp))
                    )
                >> (Record {
                    date: (
                        year.parse::<usize>().unwrap(),
                        month.parse::<usize>().unwrap(),
                        day.parse::<usize>().unwrap()
                    ),
                    hour: hour.parse::<usize>().unwrap(),
                    minute: minute.parse::<usize>().unwrap(),
                    action: action,
                })
        )
    ).expect("well-formed input")
    .1
}

#[test]
fn test_parse() {
    assert_eq!(
        parse_record("[1518-11-01 00:00] Guard #10 begins shift"),
        Record {
            date: (1518, 11, 1),
            hour: 0,
            minute: 0,
            action: BeginShift(10),
        }
    );
    assert_eq!(
        parse_record("[1518-11-01 00:05] falls asleep"),
        Record {
            date: (1518, 11, 1),
            hour: 0,
            minute: 5,
            action: FallAsleep,
        }
    );
    assert_eq!(
        parse_record("[1518-11-01 00:25] wakes up"),
        Record {
            date: (1518, 11, 1),
            hour: 0,
            minute: 25,
            action: WakeUp,
        }
    );
}
