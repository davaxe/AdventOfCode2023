use core::panic;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Empty,
    MirrorRight,
    MirrorLeft,
    HorizontalSplitter,
    VerticalSplitter,
}

pub fn task(input: &str) -> Option<String> {
    let contraption: Vec<Box<[Tile]>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Tile::Empty,
                    '/' => Tile::MirrorRight,
                    '\\' => Tile::MirrorLeft,
                    '-' => Tile::HorizontalSplitter,
                    '|' => Tile::VerticalSplitter,
                    _ => panic!("Unknown tile: {}", c),
                })
                .collect()
        })
        .collect();

    // Set containing beams that have already happened to prevent infinite loops
    let mut already_happened: HashSet<(Direction, Tile, (usize, usize))> = HashSet::new();

    // Set containing all visited positions
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // Vector containing active beams
    let mut beams = vec![(0usize, 0usize, Direction::Right)];

    // Get width and height of the contraption
    let width = contraption[0].len() - 1;
    let height = contraption.len() - 1;

    let mut first = true;

    while !beams.is_empty() {
        let mut new_beams = Vec::new();
        let mut remove = Vec::new();

        for (idx, (x, y, dir)) in beams.iter_mut().enumerate() {
            // Get next position of the beam
            let (nx, ny) = match dir {
                Direction::Right if *x == 0 && *y == 0 && first => {
                    first = false;
                    (*x, *y)
                }
                Direction::Up if *y > 0 => (*x, *y - 1),
                Direction::Down if *y < height => (*x, *y + 1),
                Direction::Left if *x > 0 => (*x - 1, *y),
                Direction::Right if *x < width => (*x + 1, *y),
                _ => {
                    remove.push(idx);
                    continue;
                }
            };

            visited.insert((nx, ny));

            let prev_tile = contraption[*x][*y];

            // Handle beam depending on the next tile
            match contraption[ny][nx] {
                Tile::Empty => {
                    // Move the beam
                    *x = nx;
                    *y = ny;
                }
                Tile::MirrorRight => {
                    // Move the beam
                    *x = nx;
                    *y = ny;

                    // Change direction
                    *dir = match dir {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                }
                Tile::MirrorLeft => {
                    // Move the beam
                    *x = nx;
                    *y = ny;

                    // Change direction
                    *dir = match dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                }
                Tile::HorizontalSplitter => {
                    if already_happened.contains(&(*dir, prev_tile, (*x, *y))) {
                        remove.push(idx);
                        continue;
                    }
                    already_happened.insert((*dir, prev_tile, (*x, *y)));

                    if *dir == Direction::Right || *dir == Direction::Left {
                        *x = nx;
                        *y = ny;
                        continue;
                    }

                    // Create new beam
                    new_beams.push((
                        nx,
                        ny,
                        match dir {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            _ => panic!("Invalid direction for horizontal splitter"),
                        },
                    ));

                    // Move the beam
                    *x = nx;
                    *y = ny;

                    // Change direction
                    *dir = match dir {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        _ => panic!("Invalid direction for horizontal splitter"),
                    };
                }
                Tile::VerticalSplitter => {
                    if already_happened.contains(&(*dir, prev_tile, (*x, *y))) {
                        remove.push(idx);
                        continue;
                    }
                    already_happened.insert((*dir, prev_tile, (*x, *y)));

                    if *dir == Direction::Up || *dir == Direction::Down {
                        *x = nx;
                        *y = ny;
                        continue;
                    }

                    // Create new beam
                    new_beams.push((
                        nx,
                        ny,
                        match dir {
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                            _ => panic!("Invalid direction for vertical splitter"),
                        },
                    ));

                    // Move the beam
                    *x = nx;
                    *y = ny;

                    // Change direction
                    *dir = match dir {
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                        _ => panic!("Invalid direction for vertical splitter"),
                    };
                }
            }
        }

        // Remove beams that are out of bounds
        let len = beams.len();
        for (i, idx) in remove.iter().enumerate() {
            beams.swap(*idx, len - 1 - i);
        }
        beams.truncate(len - remove.len());

        // Add new beams
        beams.append(&mut new_beams);
    }

    Some(visited.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "46");
    }
}
