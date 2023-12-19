use std::collections::HashMap;

use crate::parser::{self, Part, Rule};

pub fn task(input: &str) -> Option<String> {
    let (input, rules) = parser::rules(input).ok()?;
    let (_, parts) = parser::parts(input).ok()?;

    Some(
        parts
            .into_iter()
            .filter(|part| process_part(part, &rules))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum::<u32>()
            .to_string(),
    )
}

fn process_part(part: &Part, rules: &HashMap<&str, Rule>) -> bool {
    let mut current_rule = "in";
    loop {
        current_rule = rules[current_rule].evaluate(part);

        if current_rule == "A" {
            return true;
        } else if current_rule == "R" {
            return false;
        }
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
        assert_eq!(result.unwrap(), "19114");
    }
}
