use crate::parser;

pub fn task(input: &str) -> Option<String> {
    let (_, (nodes, instructions)) = parser::map(input).ok()?;

    let starting_nodes = nodes
        .keys()
        .copied()
        .filter(|name| name.ends_with('A'))
        .collect::<Vec<&str>>();

    // All starting node have a specific cycle length, where end of the cycle is
    // always a valid end node.
    let cycle_lengths: Vec<usize> = starting_nodes
        .iter()
        .map(|&start| {
            let mut current = start;
            instructions
                .0
                .chars()
                .cycle()
                .enumerate()
                .find_map(|(count, instr)| {
                    current = match instr {
                        'L' => nodes.get(current)?.left,
                        'R' => nodes.get(current)?.right,
                        _ => panic!("invalid instruction"),
                    };

                    if current.ends_with('Z') {
                        Some(count + 1)
                    } else {
                        None
                    }
                })
                .expect("no cycle found")
        })
        .collect();

    // Need to find the least common multiple of all cycle lengths.
    Some(lcm(&cycle_lengths).to_string())
}

fn lcm(numbers: &[usize]) -> usize {
    if numbers.len() == 1 {
        return numbers[0];
    }
    let a = numbers[0];
    let b = lcm(&numbers[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example3.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "6");
    }
}
