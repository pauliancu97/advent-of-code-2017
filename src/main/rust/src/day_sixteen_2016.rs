use std::str;

fn get_dragon_curve_one_step(data: &[u8]) -> Vec<u8> {
    let mut updated_data = vec![0u8; 2 * data.len() + 1];
    for (index, &bit) in data.iter().enumerate() {
        updated_data[index] = bit;
    }
    for index in 0..data.len() {
        let updated_data_index = data.len() + index + 1;
        let original_data_index = data.len() - index - 1;
        let bit = 1 - data[original_data_index];
        updated_data[updated_data_index] = bit
    }
    updated_data
}

fn get_dragon_curve(data: &[u8], size: usize) ->  Vec<u8> {
    let mut current_data = data.to_vec();
    while current_data.len() < size {
        current_data = get_dragon_curve_one_step(&current_data);
    }
    (&current_data.as_slice()[0..size]).to_vec()
}

fn get_checksum_step(data: &[u8]) -> Vec<u8> {
    data.chunks(2)
        .map(|chunk| {
            if chunk[0] == chunk[1] {
                1u8
            } else {
                0u8
            }
        })
        .collect()
}

fn get_checksum(data: &[u8]) -> Vec<u8> {
    let mut current_data = data.to_vec();
    while current_data.len() % 2 == 0 {
        current_data = get_checksum_step(&current_data);
    }
    current_data
}

fn get_final_data(data: &[u8], size: usize) -> Vec<u8> {
    let dragon_curve = get_dragon_curve(data, size);
    get_checksum(&dragon_curve)
}

fn get_data_from_string(string: &str) -> Vec<u8> {
    string.as_bytes().iter()
        .map(|&byte| if byte == b'1' { 1u8 } else { 0u8 })
        .collect()
}

fn get_string_from_data(data: &[u8]) -> String {
    let bytes: Vec<_> = data.iter()
        .map(|&byte| if byte == 1 { b'1' } else { b'0' })
        .collect();
    let string = str::from_utf8(&bytes).unwrap();
    String::from(string)
}

pub fn solve_part_one(data: &str, size: usize) -> String {
    let final_data = get_final_data(&get_data_from_string(data), size);
    get_string_from_data(&final_data)
}
