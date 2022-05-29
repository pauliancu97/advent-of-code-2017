use std::fs;

pub fn read_lines(path: &str) -> Option<Vec<String>> {
    let string = fs::read_to_string(path).ok()?;
    let lines: Vec<String> = string
        .split('\n')
        .map(|string| string.to_string())
        .collect();
    Some(lines.clone())
}

pub fn remove<T:Eq + PartialEq>(vec: &mut Vec<T>, element: T) {
    if let Some(index) = vec.iter().position(|val| *val == element) {
        vec.remove(index);
    }
}

pub fn get_num_set_bits(n: u64) -> u8 {
    let mut result: u8 = 0;
    for shift in 0..64 {
        let mask = 1u64 << shift;
        if n & mask != 0 {
            result += 1;
        }
    }
    result
}