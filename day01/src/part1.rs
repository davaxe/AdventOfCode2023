#[allow(dead_code)]

fn task(_input: &str) -> Option<String> {
    todo!("Implement the task here");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = "";
        assert!(task(input).is_some());
        assert_eq!(task(input).unwrap(), "");
    }
}