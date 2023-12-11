use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Connection {
    Up,
    Down,
    Left,
    Right,
}

impl Connection {
    pub fn opposite(&self) -> Self {
        match self {
            Connection::Up => Connection::Down,
            Connection::Down => Connection::Up,
            Connection::Left => Connection::Right,
            Connection::Right => Connection::Left,
        }
    }
}

#[derive(Debug)]
struct Pipe {
    connections: Vec<Connection>,
}

impl Pipe {
    pub fn from_char(c: char) -> Option<Self> {
        let connections = match c {
            '|' => vec![Connection::Up, Connection::Down],
            '-' => vec![Connection::Left, Connection::Right],
            'L' => vec![Connection::Up, Connection::Right],
            'J' => vec![Connection::Up, Connection::Left],
            '7' => vec![Connection::Down, Connection::Left],
            'F' => vec![Connection::Down, Connection::Right],
            'S' => vec![
                Connection::Up,
                Connection::Down,
                Connection::Left,
                Connection::Right,
            ],
            _ => return None,
        };
        Some(Self { connections })
    }
}

pub fn task(input: &str) -> Option<String> {
    let mut map: HashMap<(i32, i32), Pipe> = HashMap::new();
    let mut start = (0, 0);

    // Build map
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            if let Some(pipe) = Pipe::from_char(c) {
                map.insert((x as i32, y as i32), pipe);
            }

            if c == 'S' {
                start = (x as i32, y as i32);
            }
        }
    }

    // Perform Breadth-first search to find the longest path possible
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashMap::new();

    while let Some((pos, steps)) = queue.pop_front() {
        visited.insert(pos, steps);

        for nbr in get_neighbors(pos, &map) {
            if visited.contains_key(&nbr) {
                continue;
            }

            queue.push_back((nbr, steps + 1));
        }
    }

    visited.values().max().map(|v| v.to_string())
}

fn get_neighbors((x, y): (i32, i32), map: &HashMap<(i32, i32), Pipe>) -> Vec<(i32, i32)> {
    // pipe at current position
    let pipe = match map.get(&(x, y)) {
        Some(pipe) => pipe,
        None => return vec![],
    };

    pipe.connections
        .iter()
        // Remove all connections that are not in the map
        .filter(|con| map.contains_key(&offset((x, y), con)))
        // Add all connections that are in the map, along with the required connection
        .map(|con| (offset((x, y), con), con.opposite()))
        .filter(|(nbr_pos, req)| {
            let nbr = map.get(nbr_pos).unwrap();
            nbr.connections.iter().any(|con| con == req)
        })
        .map(|(nbr_pos, _)| nbr_pos)
        .collect()
}

fn offset((x, y): (i32, i32), connection: &Connection) -> (i32, i32) {
    match connection {
        Connection::Up => (x, y - 1),
        Connection::Down => (x, y + 1),
        Connection::Left => (x - 1, y),
        Connection::Right => (x + 1, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "4");
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example2.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "8");
    }
}
