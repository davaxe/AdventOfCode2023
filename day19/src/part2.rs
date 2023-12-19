use std::{collections::HashMap, ops::Range};

use crate::parser::{rules, ComparisonType, PartCategory, Rule};

pub fn task(input: &str) -> Option<String> {
    let (_, rules) = rules(input).ok()?;

    Some(valid_combinations("in", [1..4000, 1..4000, 1..4000, 1..4000], &rules).to_string())
}

/// Recursively compute the number of valid combinations for a map of rules
/// and ranges of values for each part category.
fn valid_combinations(
    current_rule: &str,
    range: [Range<u32>; 4],
    rules: &HashMap<&str, Rule>,
) -> u64 {
    match current_rule {
        "A" => {
            return range.into_iter().map(|r| (r.count() + 1) as u64).product();
        }
        "R" => return 0,
        _ => {}
    }

    let rule = rules.get(current_rule).expect("Expected rule to exist");
    let mut current_range = range;
    let mut combinations = 0;

    for comp in &rule.comparisons {
        let next = comp.result;
        // Get relevant range, if it unwraps we are guaranteed to have NoComparison as kind
        let range_index = part_category_to_range_index(
            comp.part_category.as_ref().unwrap_or(&PartCategory::Cool),
        );

        match comp.kind {
            ComparisonType::GreaterThan => {
                let mut range = current_range.clone();
                if range[range_index].end > comp.value {
                    // Compute new ranges, `range` contains range that are valid for
                    // the next rule `current_range` contains ranges that remain for
                    // next comparisons
                    let lower = range[range_index].start.min(comp.value);
                    let upper = range[range_index].start.max(comp.value);
                    range[range_index].start = upper + 1;
                    current_range[range_index].start = lower;
                    current_range[range_index].end = upper;
                    combinations += valid_combinations(next, range, rules);
                }
            }
            ComparisonType::LessThan => {
                let mut range = current_range.clone();

                if range[range_index].start < comp.value {
                    // Compute new ranges, `range` contains range that are valid for
                    // the next rule `current_range` contains ranges that remain for
                    // next comparisons
                    let lower = range[range_index].end.min(comp.value);
                    let upper = range[range_index].end.max(comp.value);
                    range[range_index].end = lower - 1;
                    current_range[range_index].start = lower;
                    current_range[range_index].end = upper;
                    combinations += valid_combinations(next, range, rules);
                }
            }
            ComparisonType::NoComparison => {
                combinations += valid_combinations(next, current_range.clone(), rules);
            }
        }
    }

    combinations
}

fn part_category_to_range_index(part_category: &PartCategory) -> usize {
    match part_category {
        PartCategory::Cool => 0,
        PartCategory::Musical => 1,
        PartCategory::Aerodynamic => 2,
        PartCategory::Shiny => 3,
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
        assert_eq!(result.unwrap(), "167409079868000");
    }
}
