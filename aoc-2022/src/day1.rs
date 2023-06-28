pub fn part_a(input: &str) -> i32 {
    println!("{:?}", input);
    let elf_total_c_counts: Vec<i32> = Vec::new();

    0
}

pub fn part_b(input: &str) -> i32 {
    println!("{:?}", input);
    1
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_a() {
        assert_eq!(super::part_a(include_str!("../data1.txt")), 0);
    }

    #[test]
    fn test_part_b() {
        let lines = include_str!("../data1.txt");
        println!("{}", lines);
        assert_eq!(super::part_b(lines), 1);
    }
}
