advent_of_code::solution!(8);

use std::{collections::{HashMap, HashSet}, i32};

use itertools::Itertools;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(col: i32, row: i32) -> Pos {
        Pos { x: col, y: row }
    }

    fn check_bounds(&self, grid: &Vec<Vec<char>>) -> bool {
        self.y >= 0 && self.y < grid.len() as i32 && self.x >= 0 && self.x < grid[0].len() as i32
    }

    fn add(self, rhs: Pos) -> Pos {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
    fn sub(self, rhs: Pos) -> Pos {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();

    grid.iter().enumerate().for_each(|(row, cells)| {
        cells.iter().enumerate().for_each(|(col, cell)| {
            if *cell == '.' {
                return;
            }
            antennas.entry(*cell).or_default().push(Pos::new(col as i32, row as i32));
        });
    });

    let mut antinodes: HashSet<Pos> = HashSet::new();

    antennas.values().for_each(|positions| {
        for pair in positions.iter().combinations(2) {
            let node1 = pair[0];
            let node2 = pair[1];

            let diff = node2.sub(*node1);

            let anti1 = node2.add(diff);
            let anti2 = node1.sub(diff);

            if anti1.check_bounds(&grid) {
                antinodes.insert(anti1);
            }

            if anti2.check_bounds(&grid) {
                antinodes.insert(anti2);
            }
        }
    });

    Some(antinodes.len() as u32)

}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut antennas: HashMap<char, Vec<Pos>> = HashMap::new();

    grid.iter().enumerate().for_each(|(row, cells)| {
        cells.iter().enumerate().for_each(|(col, cell)| {
            if *cell == '.' {
                return;
            }
            antennas.entry(*cell).or_default().push(Pos::new(col as i32, row as i32));
        });
    });

    let mut antinodes: HashSet<Pos> = HashSet::new();

    antennas.values().for_each(|positions| {
        positions.iter().combinations(2).for_each(|pair| {
            let node1 = pair[0];
            let node2 = pair[1];

            let diff = node2.sub(*node1);

            let mut anti1 = *node2;
            let mut anti2 = *node1;

            while anti1.check_bounds(&grid) {
                antinodes.insert(anti1);
                anti1 = anti1.add(diff);
            }

            while anti2.check_bounds(&grid) {
                antinodes.insert(anti2);
                anti2 = anti2.sub(diff);
            }
        });
        
    });

    Some(antinodes.len() as u32)
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
        assert_eq!(result, Some(11387));
    }
}
