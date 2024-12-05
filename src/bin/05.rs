use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (hm, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|update| is_update_in_order(update, &hm))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (hm, updates) = parse_input(input);

    Some(
        updates
            .iter()
            .filter(|update| !is_update_in_order(update, &hm))
            .map(|update| fix_update(update, &hm)[update.len() / 2])
            .sum(),
    )
}

//O(n^2) solution
// fn fix_update(update: &[u32], dependencies: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
//     let mut update = update.to_vec();
//     for i in 0..update.len() {
//         let current = update[i];
//         if let Some(required_after) = dependencies.get(&current) {
//             for &after_page in required_after {
//                 if let Some(after_idx) = update.iter().position(|&x| x == after_page) {
//                     if after_idx < i {
//                         //keep swap the two pages until they are in order
//                         update.swap(i, after_idx);
//                         if !is_update_in_order(&update, &dependencies) {
//                             return fix_update(&update, &dependencies);
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     update
// }

//O(n) solution
fn fix_update(update: &[u32], hm: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    let update = update.to_vec();

    // Calculate the correct position for each page based on rule references
    let positions: Vec<usize> = update
        .iter()
        .map(|&seq| {
            update
                .iter()
                .filter(|&&i| i != seq)
                .filter(|&&ii| hm.get(&seq).map_or(false, |deps| deps.contains(&ii)))
                .count()
        })
        .collect();

    // Reorder the update based on calculated positions
    let mut reordered = vec![0; update.len()];
    for (idx, &pos) in positions.iter().enumerate() {
        reordered[pos] = update[idx];
    }

    reordered
}

fn is_update_in_order(update: &[u32], dependencies: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 0..update.len() {
        let current = update[i];
        if let Some(required_after) = dependencies.get(&current) {
            //check every page that must come after
            for &after_page in required_after {
                // if page not in/after the update, return false
                if let Some(after_idx) = update.iter().position(|&x| x == after_page) {
                    if after_idx < i {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut hm: HashMap<u32, HashSet<u32>> = std::collections::HashMap::new();

    let mut line_num = 0;
    for line in input.lines() {
        line_num += 1;
        if line.is_empty() {
            break;
        }

        let parts: Vec<u32> = line.split('|').map(|x| x.parse().unwrap()).collect();

        hm.entry(parts[0])
            .or_insert_with(HashSet::new)
            .insert(parts[1]);
    }

    let updates: Vec<Vec<u32>> = input
        .lines()
        .skip(line_num)
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (hm, updates)
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
