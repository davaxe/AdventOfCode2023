use crate::parser;

pub fn task(_input: &str) -> Option<String> {
    let (_, cards) = parser::cards(_input).ok()?;
    Some(
        cards
            .iter()
            .map(|c| c.winning_score())
            .sum::<u32>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part1-example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "13");
    }
}
