use std::{ops::Range, vec};

use crate::parser::{self, ConversionMap, Seed};

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

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part2-example.txt");
        assert!(task(input).is_some());
        assert_eq!(task(input).unwrap(), "46");
    }
}
