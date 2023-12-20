use std::collections::HashMap;

use itertools::Itertools;

pub fn task(input: &str) -> Option<String> {
    // Hashmap mapping box number to a vector of tuples containing the label and
    // focal length
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();

    input.split(',').for_each(|s| {
        let (box_nr, idx) = s
            .bytes()
            .enumerate()
            .take_while(|(_, c)| *c != b'=' && *c != b'-')
            .fold((0u32, 0), |(acc, _), (i, b)| {
                ((((acc + (b as u32)) * 17u32) % 256u32), i)
            });

        let label = &s[..idx + 1];
        let operator = s.as_bytes()[idx + 1];

        match operator {
            b'=' => {
                let focal_length = s[idx + 2..]
                    .chars()
                    .take_while(char::is_ascii_digit)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();

                // Modify if exists, otherwise insert new
                boxes
                    .entry(box_nr)
                    .and_modify(|v| {
                        if let Some((idx, _)) = v.iter().find_position(|(l, _)| *l == label) {
                            v[idx] = (label, focal_length);
                        } else {
                            v.push((label, focal_length));
                        }
                    })
                    .or_insert(vec![(label, focal_length)]);
            }
            b'-' => {
                boxes.entry(box_nr).and_modify(|v| {
                    if let Some((idx, _)) = v.iter().find_position(|(l, _)| *l == label) {
                        v.remove(idx);
                    }
                });
            }
            _ => panic!("Invalid operator"),
        }
    });

    // Calculate the total focusing power
    Some(
        boxes
            .iter()
            .map(|(box_nr, seq)| {
                seq.iter()
                    .enumerate()
                    .map(|(i, (_, focal_length))| (1 + box_nr) * (i as u32 + 1) * focal_length)
                    .sum::<u32>()
            })
            .sum::<u32>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "145");
    }
}
