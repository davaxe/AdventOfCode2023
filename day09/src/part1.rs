pub fn task(input: &str) -> Option<String> {
    let values: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split(' ')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect() // Might not need to collect here
        })
        .collect();

    Some(
        values
            .iter()
            .map(|row| next_prediction(row))
            .sum::<i32>()
            .to_string(),
    )
}

fn next_prediction(row: &[i32]) -> i32 {
    let mut differences = vec![];
    let mut current = row.to_vec();
    let last = current
        .last()
        .copied()
        .expect("Expected at least one value");

    // Keep looping until all values are 0
    loop {
        if current.iter().all(|&num| num == 0) {
            break;
        }

        current = current.windows(2).map(|pair| pair[1] - pair[0]).collect();
        // Save last value in differences, which is used for prediction
        differences.push(
            current
                .last()
                .copied()
                .expect("Expected at least one value"),
        );
    }
    last + differences.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "114");
    }
}
