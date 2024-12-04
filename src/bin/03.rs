advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    // let mut res = 0;

    // let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // for cap in re.captures_iter(input) {
    //     let a: u32 = cap[1].parse().unwrap();
    //     let b: u32 = cap[2].parse().unwrap();
    //     res += a * b;
    // }

    // Some(res)
    let mul = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    Some(
        mul.captures_iter(input)
            .map(|cap| cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled: bool = true;
    let re = regex::Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don\'t\(\)").unwrap();
    Some(
        re.captures_iter(input)
            .filter_map(|cap| match &cap[0] {
                "do()" => {
                    enabled = true;
                    None
                }
                "don't()" => {
                    enabled = false;
                    None
                }
                _ => {
                    if enabled {
                        Some(cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap())
                    } else {
                        None
                    }
                }
            })
            .sum(),
    )
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
