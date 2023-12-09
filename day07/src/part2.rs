use std::collections::HashMap;

static CARDS: &str = "J23456789TQKA";

pub fn task(input: &str) -> Option<String> {
    let mut hands = input
        .lines()
        .map(|line| {
            let mut parts = line.split(' ');
            let hand = parts.next().expect("Expected two parts");
            let bet: u32 = parts
                .next()
                .expect("Expected two parts")
                .parse()
                .expect("Expected a number");
            Hand::new(hand, bet)
        })
        .collect::<Vec<Hand>>();

    // Sort based on the rules of the game (implicitly implemented in the Ord trait)
    hands.sort();

    Some(
        hands
            .into_iter()
            .enumerate()
            .map(|(i, hand)| ((i + 1) as u32) * hand.bet)
            .sum::<u32>()
            .to_string(),
    )
}

/// Types of hands in the game, ordered from worst to best hand
/// (HighCard is the worst hand)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    None,
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

/// Struct representing a hand in the game, with the hand type, the hand itself
/// and the bet.
#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    pub hand_type: HandType,
    pub hand: &'a str,
    pub bet: u32,
}

impl<'a> Hand<'a> {
    pub fn new(hand: &'a str, bet: u32) -> Hand<'a> {
        Hand {
            hand_type: get_hand_type(hand),
            hand,
            bet,
        }
    }
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Map the card values to numbers
        let card_values: HashMap<char, u32> = CARDS
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as u32))
            .collect();

        let ordering = self.hand_type.cmp(&other.hand_type);

        // If the hand types are different, return the ordering of the hand types
        if ordering != std::cmp::Ordering::Equal {
            return ordering;
        }

        // Compare each card in the hand, and return the ordering (based on value of the card)
        // of the first card that is different
        for (a, b) in self.hand.chars().zip(other.hand.chars()) {
            if a != b {
                return card_values[&a].cmp(&card_values[&b]);
            }
        }
        std::cmp::Ordering::Equal
    }
}

/// Will return the best possible hand type for the given hand
fn get_hand_type(hand: &str) -> HandType {
    let mut card_count = HashMap::new();
    let mut count = [0; 5];

    // Keep track of the number of jokers
    let mut jokers = 0;
    for card in hand.chars() {
        if card == 'J' {
            jokers += 1;
            continue;
        }
        *card_count.entry(card).or_insert(0) += 1;
    }

    // Find card that appears the most
    let (max_key, _) = card_count
        .iter()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap_or((&'J', &0)); // If there are no max, it means that all cards are jokers

    // Greedily use jokers to further increase the max card
    *card_count.entry(*max_key).or_insert(0) += jokers;

    // Covert to count of counts, value of index i is the number of cards that
    // appear i+1 times.
    // Ex: [1, 2, 0, 0, 0] means there is one card that appears once, two cards
    // that appear twice (two pairs).
    for (_, card_count) in card_count.iter() {
        count[*card_count as usize - 1] += 1;
    }

    // All possible hand types, ordered from best to worst
    match count {
        [_, _, _, _, 1] => HandType::FiveOfAKind,
        [_, _, _, 1, _] => HandType::FourOfAKind,
        [_, 1, 1, _, _] => HandType::FullHouse,
        [_, _, 1, _, _] => HandType::ThreeOfAKind,
        [_, 2, _, _, _] => HandType::TwoPairs,
        [_, 1, _, _, _] => HandType::OnePair,
        [5, _, _, _, _] => HandType::HighCard,
        _ => HandType::None,
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
        assert_eq!(result.unwrap(), "5905");
    }
}
