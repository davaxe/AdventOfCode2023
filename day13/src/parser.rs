use itertools::Itertools;
use nom::bytes::complete::take_till;
use nom::character::complete::line_ending;
use nom::multi::separated_list1;
use nom::IResult;

/// A reflection is a horizontal or vertical reflection in a pattern. It is
/// defined by the indices of the two rows or columns that are each other's
/// reflection.
#[derive(Clone, Copy)]
pub enum Reflection {
    Horizontal((), usize),
    Vertical((), usize),
}

/// A pattern is a rectangular grid of characters.
#[derive(Debug)]
pub struct Pattern {
    rows: Vec<String>,
    columns: Vec<String>,
}

impl Pattern {
    /// Create a new pattern from a list of rows.
    pub fn from_rows(rows: Vec<String>) -> Self {
        let width = rows[0].len();

        let mut columns = vec![String::new(); width];
        for row in &rows {
            for (i, c) in row.chars().enumerate() {
                columns[i].push(c);
            }
        }

        Self { rows, columns }
    }

    /// Find a horizontal or vertical reflection in the pattern.
    pub fn find_reflection(&self) -> Option<Reflection> {
        if let Some(i) = Self::find_reflection_in(&self.rows) {
            return Some(Reflection::Horizontal((), i + 1));
        }
        if let Some(i) = Self::find_reflection_in(&self.columns) {
            return Some(Reflection::Vertical((), i + 1));
        }
        None
    }

    /// Find a horizontal or vertical reflection in the pattern, allowing for
    /// max_changes changes in the pattern. Note that this will require at least
    /// one change, i.e. won't find exact reflections.
    pub fn find_almost_reflection(&self, max_changes: usize) -> Option<Reflection> {
        if let Some(i) = Self::find_potential_reflection_in(&self.rows, max_changes) {
            return Some(Reflection::Horizontal((), i + 1));
        }
        if let Some(i) = Self::find_potential_reflection_in(&self.columns, max_changes) {
            return Some(Reflection::Vertical((), i + 1));
        }
        None
    }

    /// Helper function to find a reflection in a list of strings. List will
    /// either be the rows or columns of the pattern, since reflections are
    /// either horizontal or vertical.
    fn find_reflection_in(values: &[String]) -> Option<usize> {
        let mut indices = vec![];

        let mut current_index = 0;
        while let Some(index) = values[current_index..]
            .windows(2)
            .find_position(|win| win[0] == win[1])
            .map(|(i, _)| i)
        {
            indices.push(index + current_index);
            current_index += index + 1;
        }

        for index in indices {
            let start_left = index;
            let start_right = index + 1;

            if Self::is_reflection(start_left, start_right, values) {
                return Some(index);
            }
        }
        None
    }

    /// Same as `find_reflection_in`, but allows for max_changes changes in the
    /// pattern. Note that this will require at least one change, i.e. won't
    /// return true for exact reflections.
    fn find_potential_reflection_in(values: &[String], max_changes: usize) -> Option<usize> {
        let mut indices = vec![];
        let mut current_index = 0;
        while let Some((index, differences)) = values[current_index..]
            .windows(2)
            .map(|win| {
                win[0]
                    .chars()
                    .zip(win[1].chars())
                    .filter(|(a, b)| a != b)
                    .count()
            })
            .find_position(|&diff| diff <= max_changes)
        {
            indices.push((index + current_index, differences));
            current_index += index + 1;
        }

        for (index, differences) in indices {
            let start_left = index;
            let start_right = index + 1;

            if Self::is_almost_reflections(
                start_left,
                start_right,
                values,
                differences,
                max_changes,
            ) {
                return Some(index);
            }
        }
        None
    }

    /// Check whenever the values at the given indices are a reflection of each
    /// other. Helper function for `find_reflection_in`
    fn is_reflection(lower: usize, upper: usize, values: &[String]) -> bool {
        let max_offset = if lower < (values.len() - upper) {
            lower
        } else {
            values.len() - upper - 1
        };

        for offset in 1..=max_offset {
            let top = &values[lower - offset];
            let bottom = &values[upper + offset];
            if top != bottom {
                return false;
            }
        }
        true
    }

    /// Similar to `is_reflection`, but allows for max_changes changes in the
    /// pattern. Note that this will require at least one change, i.e. won't
    /// return true for exact reflections.
    fn is_almost_reflections(
        lower: usize,
        upper: usize,
        values: &[String],
        differences: usize,
        max_changes: usize,
    ) -> bool {
        let mut required_changes = differences;

        let max_offset = if lower < (values.len() - upper) {
            lower
        } else {
            values.len() - upper - 1
        };

        for offset in 1..=max_offset {
            let top = &values[lower - offset];
            let bottom = &values[upper + offset];
            if top != bottom {
                // Additional changes required
                required_changes += top
                    .chars()
                    .zip(bottom.chars())
                    .filter(|(a, b)| a != b)
                    .count();
            }

            if required_changes > max_changes {
                return false;
            }
        }
        required_changes > 0 && required_changes <= max_changes
    }
}

fn pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, rows) = separated_list1(line_ending, take_till(|c| c == '\r'))(input)?;
    Ok((
        input,
        Pattern::from_rows(rows.into_iter().map(|x| x.to_string()).collect()),
    ))
}

pub fn patterns(input: &str) -> IResult<&str, Vec<Pattern>> {
    let a = input
        .split("\r\n\r\n")
        .map(pattern)
        .filter_map(|x| x.ok())
        .map(|x| x.1)
        .collect();
    Ok(("", a))
}
