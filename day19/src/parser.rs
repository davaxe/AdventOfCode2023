use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{self, line_ending};
use nom::multi::{many0, separated_list1};
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

#[derive(Debug, PartialEq, Eq)]
pub enum PartCategory {
    Cool,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ComparisonType {
    GreaterThan,
    LessThan,
    NoComparison,
}

#[derive(Debug)]
pub struct Comparison<'a> {
    /// The category of part to check.
    pub part_category: Option<PartCategory>,
    /// The comparison to perform.
    pub kind: ComparisonType,
    /// The value to compare against.
    pub value: u32,
    /// The result of the rule, if it is valid.
    pub result: &'a str,
}

impl<'a> Comparison<'a> {
    fn evaluate(&self, part: &Part) -> Option<&'a str> {
        // Get correct value from part.
        let part_value = match self.part_category {
            Some(_) if self.kind == ComparisonType::NoComparison => return Some(self.result),
            Some(PartCategory::Cool) => part.x,
            Some(PartCategory::Musical) => part.m,
            Some(PartCategory::Aerodynamic) => part.a,
            Some(PartCategory::Shiny) => part.s,
            _ => return None,
        };

        match self.kind {
            ComparisonType::GreaterThan if part_value > self.value => Some(self.result),
            ComparisonType::LessThan if part_value < self.value => Some(self.result),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Rule<'a> {
    pub comparisons: Vec<Comparison<'a>>,
}

impl<'a> Rule<'a> {
    pub fn evaluate(&self, part: &Part) -> &'a str {
        // Evaluate each comparison, and return first valid result.
        for comparison in &self.comparisons {
            if let Some(result) = comparison.evaluate(part) {
                return result;
            }
        }

        // If no comparisons are valid, return the final result.
        self.comparisons.last().unwrap().result
    }
}

fn part_category(input: &str) -> IResult<&str, PartCategory> {
    alt((
        complete::char('x'),
        complete::char('m'),
        complete::char('a'),
        complete::char('s'),
    ))(input)
    .map(|(input, c)| {
        let part_category = match c {
            'x' => PartCategory::Cool,
            'm' => PartCategory::Musical,
            'a' => PartCategory::Aerodynamic,
            's' => PartCategory::Shiny,
            _ => unreachable!("Invalid part category: {}", c),
        };
        (input, part_category)
    })
}

fn comparison_type(input: &str) -> IResult<&str, ComparisonType> {
    alt((complete::char('>'), complete::char('<')))(input).map(|(input, c)| {
        let comparison = match c {
            '>' => ComparisonType::GreaterThan,
            '<' => ComparisonType::LessThan,
            _ => unreachable!("Invalid comparison type: {}", c),
        };
        (input, comparison)
    })
}

fn comparison_result(input: &str) -> IResult<&str, &str> {
    preceded(tag(":"), take_till(|c| c == ','))(input)
}

fn comparison(input: &str) -> IResult<&str, Comparison> {
    tuple((
        part_category,
        comparison_type,
        complete::u32,
        comparison_result,
    ))(input)
    .map(|(input, (part_category, comparison, value, result))| {
        let comparison = Comparison {
            part_category: Some(part_category),
            kind: comparison,
            value,
            result,
        };
        (input, comparison)
    })
}

fn rule(input: &str) -> IResult<&str, (&str, Rule)> {
    let (input, name) = take_till(|c| c == '{')(input)?;
    let (input, mut comparisons) =
        preceded(tag("{"), separated_list1(tag(","), comparison))(input)?;
    let (input, final_result) = delimited(tag(","), take_till(|c| c == '}'), tag("}"))(input)?;

    comparisons.push(Comparison {
        part_category: None,
        kind: ComparisonType::NoComparison,
        value: 0,
        result: final_result,
    });

    Ok((input, (name, Rule { comparisons })))
}

pub fn rules(input: &str) -> IResult<&str, HashMap<&str, Rule>> {
    separated_list1(tag("\r\n"), rule)(input).map(|(input, rules)| {
        let rules = rules.into_iter().collect();
        (input, rules)
    })
}

fn part_value(input: &str) -> IResult<&str, u32> {
    preceded(tuple((part_category, tag("="))), complete::u32)(input)
}

fn part(input: &str) -> IResult<&str, Part> {
    // Note part values must be in the order x, m, a, s.
    delimited(
        tag("{"),
        tuple((
            part_value,
            preceded(tag(","), part_value),
            preceded(tag(","), part_value),
            preceded(tag(","), part_value),
        )),
        tag("}"),
    )(input)
    .map(|(input, (x, m, a, s))| (input, Part { x, m, a, s }))
}

pub fn parts(input: &str) -> IResult<&str, Vec<Part>> {
    preceded(many0(line_ending), separated_list1(line_ending, part))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "ex{x>10:one,m<20:two,a>30:R,A}";
        let (input, _) = rule(input).unwrap();
        assert!(input.is_empty());
    }
}
