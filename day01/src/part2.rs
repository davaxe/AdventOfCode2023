use std::collections::HashMap;

pub fn task(input: &str) -> Option<String> {
    // Map of string digits to their numeric counterparts
    let str_digits: HashMap<&str, &str> = vec![
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .into_iter()
    .collect();

    let res = input
        .lines()
        // Filter out lines that don't contain any digits (should not happen in this case)
        .filter_map(|line| {
            // Find first digit appearance from left to right
            let first_str_digit_left = str_digits
                .keys()
                .filter_map(|digit| line.find(digit).map(|pos| (pos, *digit)))
                .min_by_key(|(pos, _)| *pos);

            let first_digit_left = line
                .find(char::is_numeric)
                .map(|pos| (pos, &line[pos..pos + 1]));

            // Decide which digit to use, based on position
            let left_digit = match (first_str_digit_left, first_digit_left) {
                (Some((pos1, digit_str)), Some((pos2, digit))) => {
                    if pos1 < pos2 {
                        str_digits[digit_str]
                    } else {
                        digit
                    }
                }
                (Some((_, digit_str)), None) => str_digits[digit_str],
                (None, Some((_, digit))) => digit,
                _ => {
                    panic!("No digit found")
                }
            };

            // Find first digit appearance from right to left, similar to above
            let first_str_digit_right = str_digits
                .keys()
                .filter_map(|digit| line.rfind(digit).map(|pos| (pos, *digit)))
                .max_by_key(|(pos, _)| *pos);

            let first_digit_right = line
                .rfind(char::is_numeric)
                .map(|pos| (pos, &line[pos..pos + 1]));

            // Decide which digit to use, based on position
            let right_digit = match (first_str_digit_right, first_digit_right) {
                (Some((pos1, digit_str)), Some((pos2, digit))) => {
                    if pos1 > pos2 {
                        str_digits[digit_str]
                    } else {
                        digit
                    }
                }
                (Some((_, digit_str)), None) => str_digits[digit_str],
                (None, Some((_, digit))) => digit,
                _ => {
                    panic!("No digit found")
                }
            };

            // Parse combined digits to u32
            format!("{}{}", left_digit, right_digit).parse::<u32>().ok()
        })
        .sum::<u32>();

    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part2-example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "281");
    }
}
