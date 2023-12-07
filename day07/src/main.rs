// Change day01 to the correct day
use day07::{part1, part2};
fn main() {
    // Output results for both parts
    println!(
        "Part 1:\n  {}",
        part1::task(include_str!("../part1-input.txt")).unwrap_or("No solution found".to_string())
    );
    println!(
        "Part 2:\n  {}",
        part2::task(include_str!("../part2-input.txt")).unwrap_or("No solution found".to_string())
    );
}
