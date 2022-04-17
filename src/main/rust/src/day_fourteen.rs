use crate::matrix::Matrix;
use queues::*;
use std::collections::HashSet;

#[derive(Debug)]
struct HashState {
    buffer_size: usize,
    position: usize,
    skip_size: usize
}

impl HashState {
    fn new(buffer_size: usize) -> Self {
        HashState {
            buffer_size,
            position: 0,
            skip_size: 0
        }
    }

    fn update(&mut self, length: usize) {
        self.position = (self.position + (length % self.buffer_size) + (self.skip_size % self.buffer_size)) % self.buffer_size;
        self.skip_size += 1;
    }
}

static SUFIX: [usize; 5] = [17, 31, 73, 47, 23];
static DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn reverse_buffer(buffer: &mut [u8], start: usize, length: usize) {
    let buffer_size = buffer.len();
    let end_index = (start + (length - 1) % buffer_size) % buffer_size;
    for offset in 0..(length / 2) {
        let first_index = (start + offset % buffer_size) % buffer_size;
        let second_index =  if (end_index as i64) - (offset as i64) >= 0 {
            end_index - offset
        } else {
            buffer_size + end_index - offset
        };
        let temp = buffer[first_index];
        buffer[first_index] = buffer[second_index];
        buffer[second_index] = temp;
    }
}

fn reverse_buffer_for_length(buffer: &mut [u8], length: usize, hash_state: &mut HashState) {
    reverse_buffer(buffer, hash_state.position, length);
    hash_state.update(length);
}

fn reverse_buffer_for_lengths(buffer: &mut [u8], lengths: &[usize], hash_state: &mut HashState) {
    for &length in lengths {
        reverse_buffer_for_length(buffer, length, hash_state);
    }
}

fn get_sparse_hash(buffer: &[u8], lengths: &[usize]) -> Vec<u8> {
    let mut current_buffer: Vec<u8> = Vec::new();
    for &byte in buffer {
        current_buffer.push(byte);
    }
    let mut hash_state = HashState::new(buffer.len());
    for _ in 0..64 {
        reverse_buffer_for_lengths(&mut current_buffer, lengths, &mut hash_state);
    }
    current_buffer
}

fn get_dense_hash(buffer: &[u8]) -> Vec<u8> {
    buffer.chunks(16)
        .map(|chunk| chunk.iter().map(|&x| x).reduce(|acc, x| acc ^ x).unwrap_or(0))
        .collect()
}

fn get_hash(input: &[usize]) -> Vec<u8> {
    let buffer: Vec<u8> = (0..=255).collect();
    let mut lengths: Vec<usize> = input.iter().map(|x| *x).collect();
    lengths.extend_from_slice(&SUFIX);
    let sparse_hash = get_sparse_hash(&buffer, &lengths);
    get_dense_hash(&sparse_hash)
}

fn get_bits(n: u8) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    for shift in (0..8).rev() {
        let shifted = n >> shift;
        result.push((shifted & 0x1) != 0);
    }
    result
}

fn get_num_set_bits(n: u8) -> u64 {
    let mut result: u64 = 0;
    for shift in (0..8).rev() {
        let shifted = n >> shift;
        if (shifted & 0x1) != 0 {
            result += 1;
        }
    }
    result
}

fn get_buffer_num_set_bits(buffer: &[u8]) -> u64 {
    buffer.iter()
        .map(|&x| get_num_set_bits(x))
        .sum()
}

fn get_buffer_bits(buffer: &[u8]) -> Vec<bool> {
    buffer.iter()
        .map(|&x| get_bits(x))
        .flatten()
        .collect()
}


fn get_row_used_cells(key: &str, row: u64) -> u64 {
    let mut hash_input_str = String::from(key);
    hash_input_str.push('-');
    hash_input_str.push_str(&row.to_string());
    let hash_input: Vec<usize> = hash_input_str.as_bytes().iter().map(|&x| x as usize).collect();
    let hash = get_hash(&hash_input);
    get_buffer_num_set_bits(&hash)
}

fn get_row(key: &str, row: u64) -> Vec<bool> {
    let mut hash_input_str = String::from(key);
    hash_input_str.push('-');
    hash_input_str.push_str(&row.to_string());
    let hash_input: Vec<usize> = hash_input_str.as_bytes().iter().map(|&x| x as usize).collect();
    let hash = get_hash(&hash_input);
    get_buffer_bits(&hash)
}

fn get_disk(key: &str) -> Matrix<bool> {
    let mut matrix: Matrix<bool> = Matrix::new(128, 128, false);
    for row in 0..128 {
        let row_data = get_row(key, row);
        for (col, &bit) in row_data.iter().enumerate() {
            matrix.set(row as usize, col, bit);
        }
    }
    matrix
}

fn get_connected_component(matrix: &Matrix<bool>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut queue: Queue<(usize, usize)> = Queue::new();
    queue.add(start);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start);
    while let Ok((row, col)) = queue.remove() {
        for &(offset_row, offset_col) in &DIRECTIONS {
            let offseted_row = (row as isize) + offset_row;
            let offseted_col = (col as isize) + offset_col;
            if matrix.has_coordinate(offseted_row, offseted_col) {
                let updated_row = offseted_row as usize;
                let updated_col = offseted_col as usize;
                if matrix.get(updated_row, updated_col) && !visited.contains(&(updated_row, updated_col)) {
                    visited.insert((updated_row, updated_col));
                    queue.add((updated_row, updated_col));
                }
            }
        }
    }
    visited
}

fn get_num_connected_components(matrix: &Matrix<bool>) -> u64 {
    let mut num_connected_components: u64 = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..matrix.rows {
        for col in 0..matrix.cols {
            if matrix.get(row, col) && !visited.contains(&(row, col)) {
                num_connected_components += 1;
                let coordinates = get_connected_component(&matrix, (row, col));
                for &coordinate in &coordinates {
                    visited.insert(coordinate);
                }
            }
        }
    }
    num_connected_components
}

fn get_used_cells(key: &str) -> u64 {
    (0..128).map(|row| get_row_used_cells(key, row)).sum()
}

pub fn solve_part_one() {
    println!("{}", get_used_cells("ugkiagan"))
}

pub fn solve_part_two() {
    let disk = get_disk("ugkiagan");
    println!("{}", get_num_connected_components(&disk));
}