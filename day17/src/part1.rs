use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    pos: (i32, i32),
    dir: Direction,
    remaining_dir: u8,
}

pub fn task(input: &str) -> Option<String> {
    let width = input.lines().next()?.len();
    let height = input.lines().count();
    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect();

    let goal = (width as i32 - 1, height as i32 - 1);

    let mut costs: HashMap<Node, u32> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(u32, Node)>> = BinaryHeap::new();

    let start_node = Node {
        pos: (0, 0),
        remaining_dir: 3,
        dir: Direction::Right,
    };

    costs.insert(start_node, 0);
    queue.push(Reverse((0, start_node)));

    while let Some(Reverse((cost, node))) = queue.pop() {
        if node.pos == goal {
            return Some(cost.to_string());
        }

        if cost > *costs.get(&node).unwrap_or(&u32::MAX) {
            continue;
        }

        for n_node in get_neighbors(node, (width as i32, height as i32)) {
            let next_pos = n_node.pos;
            let next_cost = cost + map[next_pos.1 as usize][next_pos.0 as usize];

            if next_cost < *costs.get(&n_node).unwrap_or(&u32::MAX) {
                queue.push(Reverse((next_cost, n_node)));
                costs.insert(n_node, next_cost);
            }
        }
    }
    None
}

fn get_neighbors(node: Node, (width, height): (i32, i32)) -> impl Iterator<Item = Node> {
    let (x, y) = node.pos;
    let current_dir = node.dir;
    let c_remaining_dir = node.remaining_dir;
    [
        ((x - 1, y), Direction::Left),
        ((x + 1, y), Direction::Right),
        ((x, y - 1), Direction::Up),
        ((x, y + 1), Direction::Down),
    ]
    .into_iter()
    // Bounds check
    .filter(move |&((x, y), _)| x >= 0 && y >= 0 && x < width && y < height)
    // Do not allow backwards movement
    .filter(move |(_, n_dir)| match n_dir {
        Direction::Left => current_dir != Direction::Right,
        Direction::Right => current_dir != Direction::Left,
        Direction::Up => current_dir != Direction::Down,
        Direction::Down => current_dir != Direction::Up,
    })
    // Handle restrictions on movement - can only travel same direction 3 times in
    // a row
    .filter_map(move |(pos, dir)| {
        let remaining_dir = if current_dir == dir {
            c_remaining_dir - 1
        } else {
            3
        };

        (remaining_dir > 0).then_some(Node {
            pos,
            dir,
            remaining_dir,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "102");
    }
}
