use crate::parser;

pub fn task(input: &str) -> Option<String> {
    let (input, times) = parser::times(input).ok()?;
    let (_, distances) = parser::distances(input).ok()?;

    let time = times
        .iter()
        .fold(String::new(), |acc, n| acc + &n.to_string())
        .parse::<u64>()
        .expect("Could not parse time");

    let distance = distances
        .iter()
        .fold(String::new(), |acc, n| acc + &n.to_string())
        .parse::<u64>()
        .expect("Could not parse distance");

    // Find min and max time that result equality to d_max. All values
    // whole values between min and max are valid.(second order polynomial)
    let d_max = distance as f32;
    let t = time as f32;
    // Offset a bit to avoid edge cases, i.e. equality. If no offset is used, the
    // there might occur extra valid values.
    let t_min = 0.5f32 * (t - (t * t - 4f32 * d_max).sqrt()) + 0.01;
    let t_max = 0.5f32 * (t + (t * t - 4f32 * d_max).sqrt()) - 0.01;
    Some((t_max.floor() as u32 - t_min.ceil() as u32 + 1).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "71503");
    }
}
