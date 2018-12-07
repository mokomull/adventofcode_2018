use std::collections::HashMap;

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

fn asleep_minutes<'a, T: Iterator<Item = &'a Record>>(input: T) -> HashMap<usize, [usize; 60]> {
    let mut guards = HashMap::new();

    let mut cur_guard = None;
    let mut cur_asleep = None;

    for i in input {
        match i.action {
            BeginShift(new_guard) => {
                cur_guard = Some(new_guard);
                cur_asleep = None;
            }
            FallAsleep => {
                assert!(cur_asleep.is_none());
                cur_asleep = Some(i.minute);
            }
            WakeUp => {
                if !guards.contains_key(&cur_guard.unwrap()) {
                    guards.insert(cur_guard.unwrap(), [0usize; 60]);
                }

                let data = guards.get_mut(&cur_guard.unwrap()).unwrap();

                for m in cur_asleep.unwrap()..i.minute {
                    data[m] += 1
                }

                cur_asleep = None;
            }
        }
    }

    guards
}

fn most_asleep_guard<'a, T: Iterator<Item = &'a Record>>(input: T) -> usize {
    let guards = asleep_minutes(input);

    let (most_asleep_guard, minutes) = guards
        .iter()
        .max_by_key(|(&guard, &minutes)| minutes.iter().sum::<usize>())
        .unwrap();
    let most_asleep_minute = minutes
        .iter()
        .enumerate()
        .max_by_key(|&(minute, count)| count)
        .unwrap()
        .0;

    most_asleep_guard * most_asleep_minute
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

#[test]
fn example() {
    let input = vec![
        parse_record("[1518-11-01 00:00] Guard #10 begins shift"),
        parse_record("[1518-11-01 00:05] falls asleep"),
        parse_record("[1518-11-01 00:25] wakes up"),
        parse_record("[1518-11-01 00:30] falls asleep"),
        parse_record("[1518-11-01 00:55] wakes up"),
        parse_record("[1518-11-01 23:58] Guard #99 begins shift"),
        parse_record("[1518-11-02 00:40] falls asleep"),
        parse_record("[1518-11-02 00:50] wakes up"),
        parse_record("[1518-11-03 00:05] Guard #10 begins shift"),
        parse_record("[1518-11-03 00:24] falls asleep"),
        parse_record("[1518-11-03 00:29] wakes up"),
        parse_record("[1518-11-04 00:02] Guard #99 begins shift"),
        parse_record("[1518-11-04 00:36] falls asleep"),
        parse_record("[1518-11-04 00:46] wakes up"),
        parse_record("[1518-11-05 00:03] Guard #99 begins shift"),
        parse_record("[1518-11-05 00:45] falls asleep"),
        parse_record("[1518-11-05 00:55] wakes up"),
    ];

    assert_eq!(most_asleep_guard(input.iter()), 240);
}
