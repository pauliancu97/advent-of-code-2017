use std::fs;

pub fn read_lines(path: &str) -> Option<Vec<String>> {
    let string = fs::read_to_string(path).ok()?;
    let lines: Vec<String> = string
        .split('\n')
        .map(|string| string.to_string())
        .collect();
    Some(lines.clone())
}