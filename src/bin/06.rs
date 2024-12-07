advent_of_code::solution!(6);

#[derive(Default, Clone)]
enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&mut self) {
        *self = match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        };
    }
    fn clone(&self) -> Self {
        match self {
            Dir::Up => Dir::Up,
            Dir::Down => Dir::Down,
            Dir::Left => Dir::Left,
            Dir::Right => Dir::Right,
        }
    }
}

#[derive(Default, Clone)]
struct Position {
    y: i32,
    x: i32,
    dir: Dir,
}

impl Position {
    fn new(y: i32, x: i32) -> Self {
        Self { y, x, dir: Dir::Up }
    }

    fn neww(y: i32, x: i32, dir: Dir) -> Self {
        Self { y, x, dir }
    }
    fn turn_right(&mut self) {
        self.dir.turn_right();
    }
    fn move_forward(&mut self) {
        match self.dir {
            Dir::Up => self.y -= 1,
            Dir::Down => self.y += 1,
            Dir::Left => self.x -= 1,
            Dir::Right => self.x += 1,
        }
    }
    fn clone(&self) -> Self {
        Self {
            y: self.y,
            x: self.x,
            dir: self.dir.clone(),
        }
    }
    fn step(&mut self, reverse: bool) {
        match self.dir {
            Dir::Up => {
                if !reverse {
                    self.y -= 1
                } else {
                    self.y += 1
                }
            }
            Dir::Right => {
                if !reverse {
                    self.x += 1
                } else {
                    self.x -= 1
                }
            }
            Dir::Down => {
                if !reverse {
                    self.y += 1
                } else {
                    self.y -= 1
                }
            }
            Dir::Left => {
                if !reverse {
                    self.x -= 1
                } else {
                    self.x += 1
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .position(|&c| c == '^' || c == 'v' || c == '<' || c == '>')
                .map(|x| Position::new(y as i32, x as i32))
        })
        .expect("No guard found");

    let mut visited = std::collections::HashSet::new();
    visited.insert((pos.x, pos.y));

    loop {
        let (n_y, n_x) = match pos.dir {
            Dir::Up => (pos.y - 1, pos.x),
            Dir::Down => (pos.y + 1, pos.x),
            Dir::Left => (pos.y, pos.x - 1),
            Dir::Right => (pos.y, pos.x + 1),
        };

        //bound check
        if n_y < 0 || n_x < 0 || n_y >= grid.len() as i32 || n_x >= grid[0].len() as i32 {
            break;
        }

        //turn if wall
        if grid[n_y as usize][n_x as usize] == '#' {
            pos.turn_right();
            continue;
        } else {
            pos.move_forward();
            visited.insert((pos.x, pos.y));
        }
    }
    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter().enumerate().find_map(|(x, &c)| match c {
                '^' => Some(Position::neww(y as i32, x as i32, Dir::Up)),
                'v' => Some(Position::neww(y as i32, x as i32, Dir::Down)),
                '<' => Some(Position::neww(y as i32, x as i32, Dir::Left)),
                '>' => Some(Position::neww(y as i32, x as i32, Dir::Right)),
                _ => None,
            })
        })
        .expect("No guard found");

    // Mark reachable tiles
    let mut marked_grid = grid.clone();
    mark_reachable(&mut marked_grid, pos.clone());

    let mut possible_positions = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if marked_grid[y][x] == 'X' {
                grid[y][x] = '#';
                if walk(&grid, pos.clone()) {
                    possible_positions += 1;
                }
                grid[y][x] = '.';
            }
        }
    }

    Some(possible_positions)
}

fn mark_reachable(grid: &mut Vec<Vec<char>>, mut pos: Position) {
    while pos.y >= 0 && pos.y < grid.len() as i32 && pos.x >= 0 && pos.x < grid[0].len() as i32 {
        if grid[pos.y as usize][pos.x as usize] == '#' {
            pos.step(true);
            pos.turn_right();
        } else if grid[pos.y as usize][pos.x as usize] != 'X' {
            grid[pos.y as usize][pos.x as usize] = 'X';
            pos.step(false);
        } else {
            break;
        }
    }
}

fn walk(grid: &Vec<Vec<char>>, mut pos: Position) -> bool {
    let mut count = 0;
    let mut visited = std::collections::HashSet::new();

    while pos.y >= 0 && pos.y < grid.len() as i32 && pos.x >= 0 && pos.x < grid[0].len() as i32 {
        if grid[pos.y as usize][pos.x as usize] == '#' {
            pos.step(true);
            pos.turn_right();
        }

        if visited.contains(&(pos.y, pos.x)) {
            return true;
        }

        visited.insert((pos.y, pos.x));
        count += 1;

        if count > visited.len() * 2 {
            return false;
        }

        pos.step(false);
    }

    false
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
