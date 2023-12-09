pub fn task(input: &str) -> Option<String> {
    // All lines are the same length
    let line_len = input.lines().next()?.len();
    let input = input.replace("\r\n", "");

    let mut index = 0;
    let mut result = 0;
    while let Some(c) = input.chars().nth(index) {
        if c.is_ascii_digit() {
            let len = digit_len(&input[index..]);
            (index..index + len)
                .any(|i| valid_digit(&input, i, line_len))
                .then(|| {
                    result += input[index..index + len].parse::<u32>().unwrap();
                });
            index += len;
        } else {
            index += 1;
        }
    }

    Some(result.to_string())
}

fn digit_len(input: &str) -> usize {
    input
        .chars()
        .enumerate()
        .find(|(_, c)| !c.is_ascii_digit())
        .expect("Expected correct input")
        .0
}

fn valid_digit(input: &str, index: usize, line_len: usize) -> bool {
    // Indices of the surrounding characters
    let index = index as i32;
    let line_len = line_len as i32;
    [
        index - line_len - 1,
        index - line_len,
        index - line_len + 1,
        index + line_len - 1,
        index + line_len,
        index + line_len + 1,
        index - 1,
        index + 1,
    ]
    .into_iter()
    // Remove negative indices
    .filter_map(|i| usize::try_from(i).ok())
    // Convert to char
    .filter_map(|i| input.get(i..i + 1).map(|s| s.chars().next().unwrap()))
    .any(|c| !c.is_ascii_digit() && c != '.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "4361");
    }
}
