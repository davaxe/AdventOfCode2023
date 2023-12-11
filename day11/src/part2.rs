use itertools::Itertools;

pub fn task(input: &str, increase: u64) -> Option<String> {
    let r = input.lines().count() as i32;
    let c = input.lines().next().unwrap().chars().count() as i32;

    let mut galaxies = Vec::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.char_indices().for_each(|(x, c)| match c {
            '#' => {
                galaxies.push((x as i32, y as i32));
            }
            '.' => {}
            _ => panic!("Unknown character: {}", c),
        })
    });

    let empty_rows: Vec<i32> = (0..r)
        .filter(|y| !galaxies.iter().any(|(_, gy)| *gy == *y))
        .collect();

    let empty_columns: Vec<i32> = (0..c)
        .filter(|x| !galaxies.iter().any(|(gx, _)| *gx == *x))
        .collect();

    Some(
        galaxies
            .iter()
            .combinations(2)
            // .par_bridge()
            .map(|g_vec| (g_vec[0], g_vec[1]))
            .map(|(&(fx, fy), &(tx, ty))| {
                // L-distance is optimal for this problem
                let d = (fx - tx).abs() + (fy - ty).abs();
                let row_range = fy.min(ty)..fy.max(ty);
                let col_range = fx.min(tx)..fx.max(tx);
                let empty_rows_between = empty_rows
                    .iter()
                    .filter(|&ey| row_range.contains(ey))
                    .count() as u64;

                let empty_cols_between = empty_columns
                    .iter()
                    .filter(|&ex| col_range.contains(ex))
                    .count() as u64;

                d as u64 + empty_rows_between * (increase - 1) + empty_cols_between * (increase - 1)
            })
            .sum::<u64>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let input = include_str!("../example.txt");
        let result = task(input, 10);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "1030");
    }

    #[test]
    fn test_task2() {
        let input = include_str!("../example.txt");
        let result = task(input, 100);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "8410");
    }
}
