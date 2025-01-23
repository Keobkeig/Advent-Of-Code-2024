use core::str;
advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskType {
    Free,
    File 
}

struct Disk {
    type_: DiskType,
    data: Vec<usize>,
    free: usize,
}

impl Disk {
    fn new(id: usize, size: usize) -> Self {
        if id % 2 == 0 {
            Disk { type_: DiskType::File, data: vec![id/2; size], free: 0 }
        } else {
            Disk { type_: DiskType::Free, data: Vec::new(), free: size }
        }
    }

    fn fill(&mut self, data: &[usize]) {
        self.data.extend(data);
        self.free -= data.len();
        if self.free == 0 {
            self.type_ = DiskType::File;
        }
    }

    fn clear(&mut self) {
        self.free = self.data.len();
        self.data.clear();
        self.type_ = DiskType::Free;
    }
}


fn parse_input(input: &str) -> Vec<Option<usize>> {
    let disk: Vec<Option<usize>> = input.chars()
        .filter(|&c| c.is_digit(10))
        .filter_map(|c| c.to_digit(10))
        .map(|c| c as usize)
        .enumerate()
        .flat_map( |(idx, size)| {
            if idx % 2 == 0 {
                vec![Some(idx/2); size]
            } else {
                vec![None; size]
            }
        })
        .collect();
    disk
}

fn check_sum_1(disk: &Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, &x)| 
            x.map(|x| x * i))
        .sum()
}

fn check_sum_2(disk: &Vec<Disk>) -> usize {
    disk.iter()
        .flat_map(|d| {
            d.data.iter()
                .cloned()
                .chain(std::iter::repeat(0).take(d.free))
        })
        .enumerate()
        .map(|(i, x)| x * i)
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = parse_input(input);
    let mut left = 0;
    let mut right = disk.len() - 1;

    while left < right {
        if disk[left].is_some() {
            left += 1;
        } 
        if disk[right].is_none() {
            right -= 1;
        }
        if disk[left].is_none() && disk[right].is_some() {
            disk.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
   
    Some(check_sum_1(&disk))
}

pub fn part_two(_input: &str) -> Option<usize> {
    let mut disk: Vec<Disk> = _input
        .chars()
        .filter(|&c| c.is_digit(10)) // No whitespace.
        .filter_map(|c| c.to_digit(10)) // Convert to digit.
        .map(|c| c as usize) // We'll use this as the size of the vec.
        .enumerate()
        .map(|(i, size)| Disk::new(i, size))
        .collect();

    let mut right = disk.len() - 1;

    while right > 0 {
        if disk[right].type_ == DiskType::Free {
            right -= 1;
            continue;
        }
        
        let mut left = 0;
        while left < right {
            if disk[left].type_ == DiskType::File ||
            (disk[right].type_ == DiskType::Free && disk[left].free < disk[right].data.len()) {
                left += 1;
                continue;
            }

            let r = disk[right].data.clone();
            disk[left].fill(&r);
            disk[right].clear();
            break;
        }
        right -= 1;
    }

    Some(check_sum_2(&disk))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(60));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
