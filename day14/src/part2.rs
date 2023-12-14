use crate::parser::{self, Direction};

pub fn task(input: &str) -> Option<String> {
    let (_, mut board) = parser::board(input).ok()?;

    let cycle = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ];

    // Achieve stable state, 250 cycles
    for _ in 0..250 {
        for direction in &cycle {
            board.move_round_rocks(*direction);
        }
    }

    // Find cycle with len count
    let current_load = board.total_load();
    let mut count = 0;
    loop {
        for direction in &cycle {
            board.move_round_rocks(*direction);
        }

        let new_load = board.total_load();
        count += 1;
        if current_load == new_load {
            break;
        }
    }

    // Find remaining cycles after `(1_000_000_000 - 250) / count` iterations
    let remaining = (1_000_000_000 - 250) % count;

    // Manually iterate remaining cycles
    for _ in 0..remaining {
        for direction in &cycle {
            board.move_round_rocks(*direction);
        }
    }

    Some(board.total_load().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "64");
    }
}
