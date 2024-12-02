advent_of_code::solution!(2);

fn inc_or_dec(n1: i32, n2: i32) -> bool {
    n1 < n2
}

fn valid_bounds(diff: u32) -> bool {
    (1..=3).contains(&diff)
}

fn is_safe(row: &[i32]) -> bool {
    if row.len() < 2 {
        return false;
    }
    let first_inc_dec = inc_or_dec(row[0], row[1]);
    row.windows(2).all(|pair| {
        let (prev, curr) = (pair[0], pair[1]);
        let diff = prev.abs_diff(curr);
        inc_or_dec(prev, curr) == first_inc_dec && valid_bounds(diff)
    })
}

fn is_partially_safe(row: &[i32]) -> bool {
    if is_safe(row) {
        return true;
    }
    //enumerate to get index, discard value, functional-style for-loops
    row.iter().enumerate().any(|(i, _)| {
        let mut safe_copy = row.to_vec();
        safe_copy.remove(i);
        is_safe(&safe_copy)
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    Some(reports.iter().filter(|row| is_safe(row)).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse_input(input);
    Some(reports.iter().filter(|row| is_partially_safe(row)).count() as u32)
}

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    // let reader = std::io::BufReader::new(input.as_bytes());
    // let mut res: Vec<Vec<i32>> = Vec::new();

    // for line in reader.lines() {
    //     let line = line.expect("Failed to read lines");
    //     let row: Vec<i32> = line
    //         .split_whitespace()
    //         .map(|x| x.parse::<i32>().expect("Failed to parse number"))
    //         .collect();

    //     res.push(row);
    // }

    // return res;
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().expect("Failed to parse number"))
                .collect()
        })
        .collect()
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
