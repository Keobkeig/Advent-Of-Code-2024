advent_of_code::solution!(7);

pub fn parse_input(input: &str) -> Vec<Vec<isize>> {
    let mut matrix: Vec<Vec<isize>> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let mut row: Vec<isize> = Vec::new();
        row.push(parts[0].parse::<isize>().unwrap());

        if parts.len() == 2 {
            let rhs: Vec<isize> = parts[1]
                .split_whitespace()
                .filter_map(|x| x.parse::<isize>().ok())
                .collect();
            row.extend(rhs);
        }
        matrix.push(row);
    }
    matrix
}

pub fn part_one(input: &str) -> Option<isize> {
    let matrix = parse_input(input);

    Some(
        matrix
            .iter()
            .filter_map(|row| match evaluate(0, row[0], &row[1..]) {
                true => Some(row[0]),
                false => None,
            })
            .sum::<isize>(),
    )
}

fn evaluate(start: isize, target: isize, numbers: &[isize]) -> bool {
    if numbers.is_empty() {
        return start == target;
    }
    let (first, rest) = numbers.split_first().unwrap();
    evaluate(start * first, target, rest) || evaluate(start + first, target, rest)
}

fn evaluate_concat(start: isize, target: isize, numbers: &[isize]) -> bool {
    if numbers.is_empty() {
        return start == target;
    }
    if start > target {
        return false;
    }
    let (first, rest) = numbers.split_first().unwrap();
    evaluate_concat(start * first, target, rest)
        || evaluate_concat(start + first, target, rest)
        || evaluate_concat(concat(start, *first), target, rest)
}

fn concat(n1: isize, n2: isize) -> isize {
    let mut offset = 1;
    while offset <= n2 {
        offset *= 10;
    }
    n1 * offset + n2
}

pub fn part_two(input: &str) -> Option<isize> {
    let matrix = parse_input(input);
    Some(
        matrix
            .iter()
            .filter_map(|row| match evaluate_concat(0, row[0], &row[1..]) {
                true => Some(row[0]),
                false => None,
            })
            .sum::<isize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "190: 10 19\n3267: 81 40 27\n292: 11 6 16 20";
        let result = part_one(input);
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
