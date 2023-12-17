use std::fs;

// In a shared module, e.g., utils.rs
pub fn read_input(day_number: u32) -> Vec<String> {
    let lines = fs::read_to_string(format!("./inputs/day{}.txt", day_number))
        .unwrap()
        .lines()
        .map(|s| s.to_string())
        .collect();
    lines
}