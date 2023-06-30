fn part_1(input: &str) -> i32 {
    let dimention_list: Vec<&str> = input.lines().collect();

    let mut total_feet = 0;

    for dimention in dimention_list {
        let mut split = dimention.trim().split('x');
        // L x W x H
        let l: i32 = split.next().unwrap().parse().unwrap();
        let w: i32 = split.next().unwrap().parse().unwrap();
        let h: i32 = split.next().unwrap().parse().unwrap();

        let init_area_1 = l * w;
        let init_area_2 = w * h;
        let init_area_3 = h * l;

        let areas = vec![init_area_1, init_area_2, init_area_2];
        let min = areas.iter().min().unwrap();

        let area_1 = 2 * init_area_1;
        let area_2 = 2 * init_area_2;
        let area_3 = 2 * init_area_3;

        let total = area_1 + area_2 + area_3 + min;

        total_feet += total;
    }

    total_feet
}

fn part_2(input: &str) -> i32 {
    let mut input_split: Vec<&str> = input.lines().collect();

    for dimention in input_split {
        let mut split = dimention.trim().split('x');
        // L x W x H
        let l: i32 = split.next().unwrap().parse().unwrap();
        let w: i32 = split.next().unwrap().parse().unwrap();
        let h: i32 = split.next().unwrap().parse().unwrap();
    }

    33
}

#[cfg(test)]
mod day2_tests {
    use crate::util::get_input;

    use super::*;

    #[test]
    fn part_1_test() {
        let input = get_input("./inputs/data2.txt");
        let r = part_1(&input);
        assert_eq!(r, 1602437);
    }

    #[test]
    fn part_2_test() {
        let input = get_input("./inputs/data2.txt");
        let r = part_2(&input);
        assert_eq!(r, 33);
    }
}
