use std::io::Read;

use opcodes::Reg;

#[derive(Debug, Eq, PartialEq)]
struct Opcode(opcodes::Opcode, usize, usize, usize);

mod parser {
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, line_ending, space1};
    use nom::multi::separated_list1;
    use nom::sequence::tuple;
    use nom::IResult;
    use nom::Parser;

    use crate::Opcode;

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

    fn line(input: &str) -> IResult<&str, Opcode> {
        tuple((opcode, space1, integer, space1, integer, space1, integer))
            .map(|(op, _, a, _, b, _, c)| Opcode(op, a, b, c))
            .parse(input)
    }

    pub(crate) fn top(input: &str) -> IResult<&str, (usize, Vec<Opcode>)> {
        tuple((
            tag("#ip "),
            integer,
            line_ending,
            separated_list1(line_ending, line),
        ))
        .map(|(_, binding, _, opcodes)| (binding, opcodes))
        .parse(input)
    }

    #[cfg(test)]
    mod test {
        #[test]
        fn top() {
            use crate::Opcode;
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
                (
                    0,
                    vec![
                        Opcode(Seti, 5, 0, 1),
                        Opcode(Seti, 6, 0, 2),
                        Opcode(Addi, 0, 1, 0),
                        Opcode(Addr, 1, 2, 3),
                        Opcode(Setr, 1, 0, 0),
                        Opcode(Seti, 8, 0, 4),
                        Opcode(Seti, 9, 0, 5),
                    ]
                )
            );
        }
    }
}

fn eval(binding: usize, opcodes: Vec<Opcode>) -> [Reg; 6] {
    let mut regs = [0; 6];

    while (0..opcodes.len()).contains(&(regs[binding] as usize)) {
        let ip = regs[binding] as usize;
        let Opcode(opcode, a, b, c) = opcodes[ip];

        regs = opcodes::eval_one(opcode, regs, [0, a, b, c]);

        regs[binding] += 1;
    }

    // the last instruction executed would have had an out-of-bounds instruction pointer, but that
    // value would have not been written into the registers (since that happens "during" opcode
    // execution).  Undo the last += 1, then.
    regs[binding] -= 1;

    regs
}

#[cfg(test)]
mod test {
    use super::*;
    use opcodes::Opcode::*;

    #[test]
    fn eval() {
        assert_eq!(
            super::eval(
                0,
                vec![
                    Opcode(Seti, 5, 0, 1),
                    Opcode(Seti, 6, 0, 2),
                    Opcode(Addi, 0, 1, 0),
                    Opcode(Addr, 1, 2, 3),
                    Opcode(Setr, 1, 0, 0),
                    Opcode(Seti, 8, 0, 4),
                    Opcode(Seti, 9, 0, 5),
                ]
            ),
            [6, 5, 6, 0, 0, 9]
        );
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut input)
        .expect("could not read stdin");

    let (binding, opcodes) = parser::top(&input).expect("parsing failed").1;
    let regs = eval(binding, opcodes);
    let part1 = regs[0];
    dbg!(part1);
}
