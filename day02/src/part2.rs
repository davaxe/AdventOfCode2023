use crate::parser;

pub fn task(input: &str) -> Option<String> {
    let (_, games) = parser::games(input).ok()?;

    let a = games
        .iter()
        .map(|g| {
            let (r, g, b) = g
                .sets
                .iter()
                .map(|s| s.max_count())
                .fold((0, 0, 0), |(r, g, b), (r1, g1, b1)| {
                    (u32::max(r, r1), u32::max(g, g1), u32::max(b, b1))
                });
            r * g * b
        })
        .sum::<u32>();

    Some(a.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "2286");
    }
}
