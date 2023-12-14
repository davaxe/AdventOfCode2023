use crate::parser::{self, Direction};

pub fn task(input: &str) -> Option<String> {
    let (_, mut board) = parser::board(input).ok()?;
    board.move_round_rocks(Direction::North);
    Some(board.total_load().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "136");
    }
}
