use std::ops::Range;

use rayon::prelude::*;

use crate::parser::{self, Seed};

pub fn task(input: &str) -> Option<String> {
    let (_, (seeds, maps)) = parser::almanac(input).ok()?;
    let seeds_ranges: Vec<Range<u64>> = seeds
        .chunks(2)
        .map(|chunk| {
            let Seed(start) = chunk[0];
            let Seed(range) = chunk[1];
            start..start + range
        })
        .collect();

    Some(
        seeds_ranges
            .into_par_iter()
            .flat_map(|range| range)
            .map(|seed| maps.iter().fold(seed, |acc, map| map.convert(acc)))
            .min()
            .unwrap()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "46");
    }
}
