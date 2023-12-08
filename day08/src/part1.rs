pub fn task(_input: &str) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part1-example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "");
    }
}
