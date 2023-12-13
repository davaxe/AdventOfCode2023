use crate::parser::{self, Reflection};

pub fn task(input: &str) -> Option<String> {
    let (_, patterns) = parser::patterns(input).ok()?;

    Some(
        patterns
            .iter()
            .map(
                |pattern| match pattern.find_reflection().expect("No reflection found") {
                    Reflection::Horizontal(_, u) => u * 100,
                    Reflection::Vertical(_, u) => u,
                },
            )
            .sum::<usize>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "405");
    }
}
