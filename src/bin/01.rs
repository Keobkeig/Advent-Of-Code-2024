use std::{collections::HashMap, io::BufRead};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2) = parse_input(input);
    list1.sort();
    list2.sort();

    let mut diff_sum = 0;

    for pair in list1.iter().zip(list2.iter()) {
        diff_sum += pair.0.abs_diff(*pair.1);
    }

    return Some(diff_sum);
}

pub fn part_two(input: &str) -> Option<u32> {
    let (list1, list2) = parse_input(input);
    let mut sim_sum = 0;

    let mut hm: HashMap<i32, usize> = HashMap::new();

    for &num2 in list2.iter() {
        *hm.entry(num2).or_insert(0) += 1;
    }

    for num1 in list1.iter() {
        if let Some(&count) = hm.get(num1) {
            sim_sum += *num1 as u32 * count as u32;
        }
    }

    return Some(sim_sum);
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let reader = std::io::BufReader::new(input.as_bytes());

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read lines");
        let parts: Vec<&str> = line.split_whitespace().collect();

        list1.push(
            parts[0]
                .parse()
                .expect(&("Failed to parse num1 on line".to_owned() + &line)),
        );
        list2.push(
            parts[1]
                .parse()
                .expect(&("Failed to parse num2 on line".to_owned() + &line)),
        );
    }

    return (list1, list2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
