use std::collections::{HashSet, VecDeque};

use crate::parser::{self};

pub fn task(input: &str) -> Option<String> {
    let (_, instructions) = parser::instructions(input).ok()?;

    let mut x = 0;
    let mut y = 0;

    let mut right = HashSet::new();
    let mut left = HashSet::new();

    let positions: HashSet<(i32, i32)> = instructions
        .iter()
        .flat_map(|instruction| {
            let dist = instruction.distance as i32;
            let x_l = x;
            let y_l = y;
            match instruction.direction {
                parser::Direction::Up => {
                    y -= dist;
                    (0..dist)
                        .map(|i| {
                            let (nx, ny) = (x_l, y_l - i);
                            right.insert((nx + 1, ny));
                            left.insert((nx - 1, ny));
                            (nx, ny)
                        })
                        .collect::<Vec<_>>()
                }
                parser::Direction::Down => {
                    y += dist;
                    (0..dist)
                        .map(|i| {
                            let (nx, ny) = (x_l, y_l + i);
                            right.insert((nx - 1, ny));
                            left.insert((nx + 1, ny));
                            (nx, ny)
                        })
                        .collect()
                }
                parser::Direction::Left => {
                    x -= dist;
                    (0..dist)
                        .map(|i| {
                            let (nx, ny) = (x_l - i, y_l);
                            right.insert((nx, ny - 1));
                            left.insert((nx, ny + 1));
                            (nx, ny)
                        })
                        .collect()
                }
                parser::Direction::Right => {
                    x += dist;
                    (0..dist)
                        .map(|i| {
                            let (nx, ny) = (x_l + i, y_l);
                            right.insert((nx, ny + 1));
                            left.insert((nx, ny - 1));
                            (nx, ny)
                        })
                        .collect()
                }
            }
        })
        .collect();

    let right_c = right
        .iter()
        .filter(|(x, y)| positions.contains(&(*x, *y)))
        .count();

    let left_c = left
        .iter()
        .filter(|(x, y)| positions.contains(&(*x, *y)))
        .count();

    // Find the starting position, based on which side has more positions
    let start_pos = if right_c > left_c {
        *right.iter().next().unwrap()
    } else {
        *left.iter().next().unwrap()
    };

    // Set of all in
    let mut inner = HashSet::new();
    let mut queue = VecDeque::from(vec![start_pos]);

    // Perform BFS to find all inner positions
    while let Some((x, y)) = queue.pop_front() {
        if inner.contains(&(x, y)) {
            continue;
        }
        inner.insert((x, y));

        for (nx, ny) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if !positions.contains(&(*nx, *ny)) {
                queue.push_back((*nx, *ny));
            }
        }
    }

    Some((inner.len() + positions.len()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "62");
    }
}
