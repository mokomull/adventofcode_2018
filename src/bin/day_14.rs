fn trailing_ten(after: usize) -> [u8; 10] {
    let mut recipes = vec![3u8, 7];
    let mut elf_a = 0;
    let mut elf_b = 1;

    while recipes.len() < after + 10 {
        let digits = recipes[elf_a] + recipes[elf_b];

        if digits / 10 > 0 {
            recipes.push(digits / 10);
        }
        recipes.push(digits % 10);

        elf_a = (elf_a + recipes[elf_a] as usize + 1) % recipes.len();
        elf_b = (elf_b + recipes[elf_b] as usize + 1) % recipes.len();
    }

    let mut retval = [0; 10];
    retval.copy_from_slice(&recipes[after..after + 10]);
    retval
}

#[test]
fn examples() {
    assert_eq!(trailing_ten(9), [5, 1, 5, 8, 9, 1, 6, 7, 7, 9]);
    assert_eq!(trailing_ten(5), [0, 1, 2, 4, 5, 1, 5, 8, 9, 1]);
    assert_eq!(trailing_ten(18), [9, 2, 5, 1, 0, 7, 1, 0, 8, 5]);
    assert_eq!(trailing_ten(2018), [5, 9, 4, 1, 4, 2, 9, 8, 8, 2]);
}
