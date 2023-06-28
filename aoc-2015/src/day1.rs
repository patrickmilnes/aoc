fn part_1(input: &str) -> i32 {
    let mut floor = 0;

    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("not parenthsise"),
        };
    }

    floor
}

fn part_2(input: &str) -> i32 {
    let mut floor = 0;
    let mut index = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("not parenthsise"),
        }

        if floor < 0 {
            break;
        }

        index += 1;
    }

    index + 1
}

#[cfg(test)]
mod day1_tests {
    use crate::util::get_input;

    use super::*;

    #[test]
    fn part_1_test() {
        let input = get_input("./inputs/data1.txt");
        let r = part_1(&input);
        assert_eq!(r, 232);
    }

    #[test]
    fn part_2_test() {
        let input = get_input("./inputs/data1.txt");
        let r = part_2(&input);
        assert_eq!(r, 1783);
    }
}
