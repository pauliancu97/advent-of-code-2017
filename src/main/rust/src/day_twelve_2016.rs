use crate::utils::get_num_set_bits;
use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use queues::*;

struct Map {
    data: RefCell<HashMap<(u64, u64), bool>>,
    number: u64
}

impl Map {

    fn new(number: u64) -> Self {
        Map { data: RefCell::new(HashMap::new()), number }
    }

    fn is_cell_wall(row: u64, col: u64, number: u64) -> bool {
        let temp: u64 = col * col + 3 * col + 2 * col * row + row + row * row + number;
        get_num_set_bits(temp) % 2 == 1
    }

    fn is_wall(&self, row: u64, col: u64) -> bool {
        let opt = self.data.borrow().get(&(row, col)).map(|&value| value);
        if let Some(value) = opt {
            value
        } else {
            let is_wall = Self::is_cell_wall(row, col, self.number);
            self.data.borrow_mut().insert((row, col), is_wall);
            is_wall
        } 
    }
}

fn get_min_steps(start: (u64, u64), destination: (u64, u64), map: &Map) -> u64 {
    let mut queue: Queue<((u64, u64), u64)> = Queue::new();
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let directions: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    queue.add((start, 0));
    visited.insert(start);
    while let Some(((row, col), steps)) = queue.remove().ok() {
        for &(row_offset, col_offset) in &directions {
            let signed_offseted_row = row as i64 + row_offset;
            let signed_offseted_col = col as i64 + col_offset;
            if signed_offseted_row >= 0 && signed_offseted_col >= 0 {
                let offseted_row = signed_offseted_row as u64;
                let offseted_col = signed_offseted_col as u64;
                if !visited.contains(&(offseted_row, offseted_col)) && !map.is_wall(offseted_row, offseted_col) {
                    if (offseted_row, offseted_col) == destination {
                        return steps + 1;
                    } else {
                        visited.insert((offseted_row, offseted_col));
                        queue.add(((offseted_row, offseted_col), steps + 1));
                    }
                }
            }
        }
    }
    0
}

fn get_num_destinations(start: (u64, u64), distance: u64, map: &Map) -> u64 {
    let mut queue: Queue<((u64, u64), u64)> = Queue::new();
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let directions: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    queue.add((start, 0));
    visited.insert(start);
    while let Some(((row, col), steps)) = queue.remove().ok() {
        if steps < distance {
            for &(row_offset, col_offset) in &directions {
                let signed_offseted_row = row as i64 + row_offset;
                let signed_offseted_col = col as i64 + col_offset;
                if signed_offseted_row >= 0 && signed_offseted_col >= 0 {
                    let offseted_row = signed_offseted_row as u64;
                    let offseted_col = signed_offseted_col as u64;
                    if !visited.contains(&(offseted_row, offseted_col)) && !map.is_wall(offseted_row, offseted_col) {
                        visited.insert((offseted_row, offseted_col));
                        queue.add(((offseted_row, offseted_col), steps + 1));
                    }
                }
            }
        }
    }
    visited.len() as u64
}

pub fn solve_part_one() {
    let map = Map::new(1350);
    println!("{}", get_min_steps((1, 1), (39, 31), &map));
}

pub fn solve_part_two() {
    let map = Map::new(1350);
    println!("{}", get_num_destinations((1, 1), 50, &map));
}

