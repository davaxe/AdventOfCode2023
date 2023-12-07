use crate::parser;

pub fn task(_input: &str) -> Option<String> {
    let (_, cards) = parser::cards(_input).ok()?;

    let mut count: Vec<u32> = vec![1; cards.len()];

    cards.iter().enumerate().for_each(|(i, card)| {
        (i + 1..i + 1 + (card.matches() as usize)).for_each(|j| {
            if j < cards.len() {
                count[j] += count[i];
            }
        })
    });

    Some(count.into_iter().sum::<u32>().to_string())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../part2-example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "30");
    }
}
