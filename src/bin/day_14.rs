struct Recipes {
    scoreboard: Vec<u8>,
    elf_a: usize,
    elf_b: usize,
    to_emit: usize,
}

impl Recipes {
    fn new() -> Self {
        Recipes {
            scoreboard: vec![3, 7],
            elf_a: 0,
            elf_b: 1,
            to_emit: 0,
        }
    }
}

impl Iterator for Recipes {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.scoreboard.len() > self.to_emit {
            self.to_emit += 1;
            return Some(self.scoreboard[self.to_emit - 1]);
        }

        let digits = self.scoreboard[self.elf_a] + self.scoreboard[self.elf_b];

        if digits / 10 > 0 {
            self.scoreboard.push(digits / 10);
        }
        self.scoreboard.push(digits % 10);

        self.elf_a =
            (self.elf_a + self.scoreboard[self.elf_a] as usize + 1) % self.scoreboard.len();
        self.elf_b =
            (self.elf_b + self.scoreboard[self.elf_b] as usize + 1) % self.scoreboard.len();

        return self.next();
    }
}

fn trailing_ten(after: usize) -> [u8; 10] {
    let recipes = Recipes::new();
    let trailing: Vec<u8> = recipes.skip(after).take(10).collect();

    let mut retval = [0; 10];
    retval.copy_from_slice(&trailing);
    retval
}

fn main() {
    let output = trailing_ten(503761);
    print!("The next ten are ");
    for i in &output {
        print!("{}", i);
    }
    println!("");
}

#[test]
fn examples() {
    assert_eq!(trailing_ten(9), [5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
    assert_eq!(trailing_ten(5), [0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
    assert_eq!(trailing_ten(18), [9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
    assert_eq!(trailing_ten(2018), [5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
}
