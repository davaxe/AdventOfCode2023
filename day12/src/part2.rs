use itertools::Itertools;

pub fn task(input: &str) -> Option<String> {
    Some(
        input
            .lines()
            .filter_map(|row| {
                let (springs, groups) = row.split_once(' ')?;

                let springs = std::iter::once(springs).cycle().take(5).join("?");

                let required_groups: Vec<usize> =
                    groups.split(',').filter_map(|s| s.parse().ok()).collect();
                let n = required_groups.len();
                let required_groups = required_groups
                    .into_iter()
                    .cycle()
                    .take(5 * n)
                    .collect_vec();

                Some(possible_positions(&springs, required_groups))
            })
            .sum::<usize>()
            .to_string(),
    )
}

// Function from https://github.com/mfornet/advent-of-code-2023/blob/main/src/bin/12.rs#L6
fn possible_positions(springs: &str, required_groups: Vec<usize>) -> usize {
    let spring = format!(".{}", springs.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();

    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in required_groups {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut chunk = 0;

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= count && spring[i - count] != '#' {
                n_dp[i + 1] += dp[i - count];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "525152");
    }
}
