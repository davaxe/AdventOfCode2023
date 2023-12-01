#[allow(dead_code)]

fn task(input: &str) -> Option<String> {
    let res: u32 = input
        .lines()
        .filter_map(|line| {
            let left = line.find(char::is_numeric).unwrap();
            let right = line.rfind(char::is_numeric).unwrap();
            format!(
                "{}{}",
                &line[left..left + 1],
                &line[right..right + 1]
            )
            .parse::<u32>()
            .ok()
        })
        .sum();
    Some(res.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";
        assert!(task(input).is_some());
        assert_eq!(task(input).unwrap(), "142");
    }

    #[test]
    fn main() {
        let input = include_str!("../input1.txt");
        let res = task(input);
        print!("Result: {:?}", res);
        assert!(res.is_some());
    }
}
