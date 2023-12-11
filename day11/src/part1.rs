use itertools::Itertools;

pub fn task(input: &str) -> Option<String> {
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
            .map(|g_vec| (g_vec[0], g_vec[1]))
            .map(|(&(fx, fy), &(tx, ty))| {
                // L-distance is optimal for this problem
                let d = (fx - tx).abs() + (fy - ty).abs();
                let row_range = fy.min(ty)..fy.max(ty);
                let col_range = fx.min(tx)..fx.max(tx);
                let empty_rows_between = empty_rows
                    .iter()
                    .filter(|&ey| row_range.contains(ey))
                    .count() as i32;

                let empty_cols_between = empty_columns
                    .iter()
                    .filter(|&ex| col_range.contains(ex))
                    .count() as i32;

                d + empty_rows_between + empty_cols_between
            })
            .sum::<i32>()
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
        assert_eq!(result.unwrap(), "374");
    }
}
