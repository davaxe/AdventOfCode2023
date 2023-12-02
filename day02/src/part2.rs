#[allow(dead_code)]

pub fn task(_input: &str) -> Option<String> {
    todo!("Implement the task here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part2-example.txt");
        assert!(task(input).is_some());
        assert_eq!(task(input).unwrap(), "");
    }
}
