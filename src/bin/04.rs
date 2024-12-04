use std::marker::PhantomData;

advent_of_code::solution!(4);

trait CoordsType {
    fn find_xmas(vec: &Vec<Vec<char>>, x: i32, y: i32, dir: (i32, i32)) -> bool;
}

struct X;
struct A;

impl CoordsType for X {
    fn find_xmas(vec: &Vec<Vec<char>>, x: i32, y: i32, dir: (i32, i32)) -> bool {
        let mas_to_match = "MAS";

        for i in 0..mas_to_match.len() {
            let new_x = x + dir.0 * (i as i32 + 1);
            let new_y = y + dir.1 * (i as i32 + 1);

            // Bounds check
            if new_y < 0
                || new_x < 0
                || new_y as usize >= vec.len()
                || new_x as usize >= vec[new_y as usize].len()
            {
                return false;
            }

            let expected_char = mas_to_match.chars().nth(i).unwrap();
            if vec[new_y as usize][new_x as usize] != expected_char {
                return false;
            }
        }

        true
    }
}

impl CoordsType for A {
    fn find_xmas(vec: &Vec<Vec<char>>, x: i32, y: i32, _dir: (i32, i32)) -> bool {
        let diagonals = [
            [(-1, -1), (1, 1)], // Upper left to lower right
            [(1, -1), (-1, 1)], // Upper right to lower left
        ];

        let mut valid_diagonals = 0;

        for diagonal in diagonals {
            let mut string_found = String::with_capacity(2);

            for (dx, dy) in diagonal {
                let check_x = x + dx;
                let check_y = y + dy;

                // Bounds check
                if check_y < 0
                    || check_y as usize >= vec.len()
                    || check_x < 0
                    || check_x as usize >= vec[check_y as usize].len()
                {
                    return false;
                }

                let char_ = vec[check_y as usize][check_x as usize];
                match char_ {
                    'S' | 'M' => string_found.push(char_),
                    _ => return false,
                }
            }

            if string_found.as_str() == "MS" || string_found.as_str() == "SM" {
                valid_diagonals += 1;
            }
        }

        valid_diagonals == 2
    }
}

#[derive(Debug)]
struct Coords<T: CoordsType> {
    x: i32,
    y: i32,
    markers: PhantomData<T>,
}

impl<T: CoordsType> Coords<T> {
    fn new(x: i32, y: i32) -> Self {
        Coords {
            x,
            y,
            markers: PhantomData,
        }
    }

    fn find_xmas(&self, vec: &Vec<Vec<char>>, dir: (i32, i32)) -> bool {
        T::find_xmas(vec, self.x, self.y, dir)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let dirs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut input_vec: Vec<Vec<char>> = vec![];
    let mut x_coords: Vec<Coords<X>> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();

        for (x, &ch) in row.iter().enumerate() {
            if ch == 'X' {
                x_coords.push(Coords::new(x as i32, y as i32));
            }
        }

        input_vec.push(row);
    }

    Some(
        x_coords
            .iter()
            .flat_map(|coords| {
                dirs.iter()
                    .filter(|&&dir| coords.find_xmas(&input_vec, dir))
            })
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_vec: Vec<Vec<char>> = vec![];
    let mut a_coords: Vec<Coords<A>> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        let row: Vec<char> = line.chars().collect();

        for (x, &ch) in row.iter().enumerate() {
            if ch == 'A' {
                a_coords.push(Coords::new(x as i32, y as i32));
            }
        }

        input_vec.push(row);
    }

    Some(
        a_coords
            .iter()
            .filter(|coords| coords.find_xmas(&input_vec, (0, 0)))
            .count() as u32,
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
