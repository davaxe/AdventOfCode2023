use itertools::Itertools;
use rayon::prelude::*;

pub fn task(input: &str) -> Option<String> {
    Some(
        input
            .lines()
            .filter_map(|row| {
                let (springs, groups) = row.split_once(' ')?;
                let unknown_positions = springs
                    .char_indices()
                    .filter_map(|(i, c)| (c == '?').then_some(i))
                    .collect::<Vec<usize>>();

                let required_groups: Vec<usize> =
                    groups.split(',').filter_map(|s| s.parse().ok()).collect();

                Some(possible_positions(
                    &unknown_positions,
                    &required_groups,
                    springs,
                ))
            })
            .sum::<usize>()
            .to_string(),
    )
}

/// Get number of valid spring positions by filling in the unknown positions and
/// counting those that match the required groups.
fn possible_positions(
    unknown_positions: &[usize],
    required_groups: &[usize],
    springs: &str,
) -> usize {
    (0..=unknown_positions.len())
        .map(|i| {
            unknown_positions
                .iter()
                .copied()
                .combinations(i)
                .par_bridge()
                .filter(|comb| groups_from_placement(comb, springs) == required_groups)
                .count()
        })
        .sum()
}

/// Return the groups of springs when the unknown positions are filled with the
/// given placement.
fn groups_from_placement(placement: &[usize], springs: &str) -> Vec<usize> {
    let mut springs_string = springs.to_string();

    springs_string = springs_string.replace('?', ".");
    placement
        .iter()
        .for_each(|&i| springs_string.replace_range(i..i + 1, "#"));

    springs_string
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "21");
    }
}
