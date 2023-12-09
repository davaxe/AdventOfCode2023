use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrDigit<'a> {
    pub digit: &'a str,
}

pub fn task(input: &str) -> Option<String> {
    let line_len = input.lines().next()?.len() + 2;
    let mut index = 0;
    let mut result = 0;
    while let Some(star_index) = find_star(&input[index..]) {
        if let Some((left, right)) = gear(input, star_index + index, line_len) {
            let a = left.digit.parse::<u32>().ok()?;
            let b = right.digit.parse::<u32>().ok()?;
            result += a * b;
        }
        index += star_index + 1;
    }
    Some(result.to_string())
}

fn find_star(input: &str) -> Option<usize> {
    input
        .char_indices()
        .find(|(_, c)| *c == '*')
        .map(|(i, _)| i)
}

fn gear(input: &str, star_index: usize, line_len: usize) -> Option<(StrDigit, StrDigit)> {
    // Indices of the surrounding characters
    let index = star_index as i32;
    let line_len = line_len as i32;
    let str_digits = [
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
    // Convert indices to digits, if possible
    .filter_map(|i| str_digit(input, i))
    // Only want unique digits
    .collect::<HashSet<StrDigit>>()
    .into_iter()
    .collect::<Vec<StrDigit>>();

    // Exactly two digits should be found
    if str_digits.len() != 2 {
        return None;
    }

    Some((str_digits[0], str_digits[1]))
}

/// Given a string and an index of a digit, return the whole digit
fn str_digit(input: &str, index: usize) -> Option<StrDigit> {
    let _ = &input[index..index + 1].parse::<u32>().ok()?;

    // How many characters to the right of the digit are digits?
    let right = input[index..]
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit());

    // How many characters to the left of the digit are digits?
    let left = input[..index]
        .chars()
        .rev()
        .enumerate()
        .find(|(_, c)| !c.is_ascii_digit());

    // Return the whole digit
    match (left, right) {
        (Some((l_offset, _)), Some((r_offset, _))) => Some(StrDigit {
            digit: &input[index - l_offset..index + r_offset],
        }),
        (Some((l_offset, _)), None) => Some(StrDigit {
            digit: &input[index - l_offset..],
        }),
        (None, Some((r_offset, _))) => Some(StrDigit {
            digit: &input[..index + r_offset],
        }),
        (None, None) => Some(StrDigit { digit: input }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "467835");
    }

    #[test]
    fn digit_test() {
        {
            let input = "abc1353abc";
            assert_eq!(str_digit(input, 3).unwrap().digit, "1353");
        }
        {
            let input = "467...";
            println!("Input: {}", input);
            assert_eq!(str_digit(input, 2).unwrap().digit, "467");
        }
    }
}
