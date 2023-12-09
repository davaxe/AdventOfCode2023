use crate::parser;

pub fn task(input: &str) -> Option<String> {
    let (_, (nodes, instructions)) = parser::map(input).ok()?;

    // Start at the first node (from instructions)
    let mut current = "AAA";
    for (count, instr) in instructions.0.chars().cycle().enumerate() {
        // End is always `ZZZ`
        if current == "ZZZ" {
            return Some(count.to_string());
        }

        // Next node, based on instruction.
        current = match instr {
            'L' => nodes.get(current)?.left,
            'R' => nodes.get(current)?.right,
            _ => panic!("invalid instruction"),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "2");
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example2.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6");
    }
}
