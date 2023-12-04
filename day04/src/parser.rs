use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
pub struct Card<'a> {
    winning_numbers: HashSet<&'a str>,
    numbers: HashSet<&'a str>,
}

impl Card<'_> {
    pub fn winning_score(&self) -> u32 {
        let overlapping = self.winning_numbers.intersection(&self.numbers).count() as u32;
        if overlapping == 0 {
            0
        } else {
            u32::pow(2, overlapping - 1)
        }
    }

    pub fn matches(&self) -> u32 {
        self.winning_numbers.intersection(&self.numbers).count() as u32
    }
}

fn numbers(input: &str) -> IResult<&str, HashSet<&str>> {
    let (input, numbers) = separated_list1(space1, digit1)(input)?;
    Ok((input, numbers.into_iter().collect::<HashSet<_>>()))
}

fn card(input: &str) -> IResult<&str, Card> {
    let (input, (_, _, _card_nr, ..)) =
        tuple((tag("Card"), space1, digit1, tag(":"), space1))(input)?;
    let (input, (winning, numbers)) =
        separated_pair(numbers, delimited(space1, tag("|"), space1), numbers)(input)?;
    Ok((
        input,
        Card {
            winning_numbers: winning,
            numbers,
        },
    ))
}

pub fn cards(input: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(input)
}
