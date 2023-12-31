pub fn task(input: &str) -> Option<String> {
    let res: u32 = input
        .lines()
        .filter_map(|line| {
            let left = line.find(char::is_numeric).unwrap();
            let right = line.rfind(char::is_numeric).unwrap();
            format!("{}{}", &line[left..left + 1], &line[right..right + 1])
                .parse::<u32>()
                .ok()
        })
        .sum();
    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "142");
    }
}
