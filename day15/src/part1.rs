pub fn task(input: &str) -> Option<String> {
    Some(
        input
            .split(',')
            .map(|s| {
                s.bytes()
                    .fold(0u32, |acc, b| ((acc + (b as u32)) * 17u32) % 256u32)
            })
            .sum::<u32>()
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
        assert_eq!(result.unwrap(), "1320");
    }
}
