#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Ip(usize),
    Opcode(opcodes::Opcode, usize, usize, usize),
}

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, line_ending, space1};
    use nom::multi::separated_list1;
    use nom::sequence::tuple;
    use nom::IResult;
    use nom::Parser;

    use crate::Operation;

    fn opcode(input: &str) -> IResult<&str, opcodes::Opcode> {
        alt((
            tag("addr").map(|_| opcodes::Opcode::Addr),
            tag("addi").map(|_| opcodes::Opcode::Addi),
            tag("mulr").map(|_| opcodes::Opcode::Mulr),
            tag("muli").map(|_| opcodes::Opcode::Muli),
            tag("banr").map(|_| opcodes::Opcode::Banr),
            tag("bani").map(|_| opcodes::Opcode::Bani),
            tag("borr").map(|_| opcodes::Opcode::Borr),
            tag("bori").map(|_| opcodes::Opcode::Bori),
            tag("setr").map(|_| opcodes::Opcode::Setr),
            tag("seti").map(|_| opcodes::Opcode::Seti),
            tag("gtir").map(|_| opcodes::Opcode::Gtir),
            tag("gtri").map(|_| opcodes::Opcode::Gtri),
            tag("gtrr").map(|_| opcodes::Opcode::Gtrr),
            tag("eqir").map(|_| opcodes::Opcode::Eqir),
            tag("eqri").map(|_| opcodes::Opcode::Eqri),
            tag("eqrr").map(|_| opcodes::Opcode::Eqrr),
        ))(input)
    }

    fn integer(input: &str) -> IResult<&str, usize> {
        digit1
            .map(|s: &str| s.parse::<usize>().expect("could not parse integer"))
            .parse(input)
    }

    fn line(input: &str) -> IResult<&str, Operation> {
        alt((
            tuple((tag("#ip "), integer)).map(|(_, i)| Operation::Ip(i)),
            tuple((opcode, space1, integer, space1, integer, space1, integer))
                .map(|(op, _, a, _, b, _, c)| Operation::Opcode(op, a, b, c)),
        ))(input)
    }

    fn top(input: &str) -> IResult<&str, Vec<Operation>> {
        separated_list1(line_ending, line)(input)
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn top() {
            use crate::Operation::*;
            use opcodes::Opcode::*;

            let result = super::top(
                "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
",
            )
            .expect("parsing failed")
            .1;

            assert_eq!(
                result,
                vec![
                    Ip(0),
                    Opcode(Seti, 5, 0, 1),
                    Opcode(Seti, 6, 0, 2),
                    Opcode(Addi, 0, 1, 0),
                    Opcode(Addr, 1, 2, 3),
                    Opcode(Setr, 1, 0, 0),
                    Opcode(Seti, 8, 0, 4),
                    Opcode(Seti, 9, 0, 5),
                ]
            );
        }
    }
}

fn main() {
    println!("Hello, world!");
}
