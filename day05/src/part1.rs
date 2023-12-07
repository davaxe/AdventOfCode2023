use crate::parser::{self, Seed};

pub fn task(input: &str) -> Option<String> {
    let (_, (seeds, maps)) = parser::almanac(input).ok()?;
    Some(
        seeds
            .iter()
            .map(|Seed(s)| {
                let mut value = *s;
                for map in &maps {
                    value = map.convert(value);
                }
                value
            })
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
        let input = include_str!("../part1-example.txt");
        assert!(task(input).is_some());
        assert_eq!(task(input).unwrap(), "35");
    }
}
