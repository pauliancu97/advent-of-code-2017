use std::collections::HashMap;
use md5;

fn get_triplet(bytes: &[u8]) -> Option<u8> {
    for index in 0..(bytes.len() - 2) {
        if bytes[index] == bytes[index + 1] && bytes[index + 1] == bytes[index + 2] {
            return Some(bytes[index]);
        }
    }
    None
}

fn has_quintuplet(bytes: &[u8], byte: u8) -> bool {
    for index in 0..(bytes.len() - 4) {
        let mut is_quintuplet = true;
        for offset in 0..5 {
            if bytes[index + offset] != byte {
                is_quintuplet = false;
                break;
            }
        }
        if is_quintuplet {
            return true;
        }
    }
    false
}

struct KeyFinder {
    possible_keys: HashMap<(u64, u8), u64>,
    num_keys: u64,
    last_key_index: Option<u64>,
    goal_num_keys: u64
}

impl KeyFinder {
    fn new(goal_num_keys: u64) -> Self {
        Self {
            possible_keys: HashMap::new(),
            num_keys: 0,
            last_key_index: None,
            goal_num_keys
        }
    }

    fn update(&mut self, index: u64, bytes: &[u8]) {
        let expired_keys: Vec<_> = self.possible_keys.iter()
            .filter(|&(_, &lifetime)| lifetime == 0)
            .map(|(&key, _)| key)
            .collect();
        for key in expired_keys {
            self.possible_keys.remove(&key);
        }
        for value in self.possible_keys.values_mut() {
            *value -= 1;
        }
        let mut sorted_keys: Vec<_> = self.possible_keys.keys()
            .map(|&key| key)
            .collect();
        sorted_keys.sort();
        for &(key_index, key_byte) in &sorted_keys {
            if has_quintuplet(bytes, key_byte) {
                if self.num_keys != self.goal_num_keys {
                    self.num_keys += 1;
                    self.last_key_index = Some(key_index);
                }
            }
        }
        if let Some(new_key_byte) = get_triplet(bytes) {
            self.possible_keys.insert((index, new_key_byte), 1000);
        }
    }

    fn is_goal(&self) -> bool {
        self.num_keys == self.goal_num_keys
    }

    fn get_key_index(&self) -> u64 {
        self.last_key_index.unwrap()
    }
}

fn get_hash_bytes(string: &str, index: u64) -> Vec<u8> {
    let owned_string = string.to_string();
    let index_string = index.to_string();
    let hash_string = owned_string + &index_string;
    let digest = md5::compute(hash_string.as_bytes());
    let digest_string = format!("{:x}", digest);
    digest_string.as_bytes().iter().map(|&byte| byte).collect()
}

fn get_stretched_hash_bytes(string: &str, index: u64, output: &mut [u8]) {
    let owned_string = string.to_string();
    let index_string = index.to_string();
    let hash_string = owned_string + &index_string;
    let mut hash_bytes: [u8; 32] = [0; 32];
    &hash_bytes[0..hash_string.len()].copy_from_slice(hash_string.as_bytes());
    let mut length: usize = hash_string.as_bytes().len();
    for _ in 0..2017 {
        let digest = md5::compute(&hash_bytes[0..length]);
        let digest_string = format!("{:x}", digest);
        let digest_bytes = digest_string.as_bytes();
        &hash_bytes.copy_from_slice(digest_bytes);
        length = 32;
    }
    output.copy_from_slice(&hash_bytes);
}

fn get_index_key(string: &str, goal: u64) -> u64 {
    let mut key_finder: KeyFinder = KeyFinder::new(goal);
    let mut index: u64 = 0;
    while !key_finder.is_goal() {
        let bytes = get_hash_bytes(string, index);
        key_finder.update(index, &bytes);
        index += 1;
    }
    key_finder.get_key_index()
}

fn get_stretched_index_key(string: &str, goal: u64) -> u64 {
    let mut key_finder: KeyFinder = KeyFinder::new(goal);
    let mut index: u64 = 0;
    while !key_finder.is_goal() {
        let mut bytes: [u8; 32] = [0; 32];
        get_stretched_hash_bytes(string, index, &mut bytes);
        key_finder.update(index, &bytes);
        index += 1;
    }
    key_finder.get_key_index()
}

pub fn solve_part_one() {
    println!("{}", get_index_key("cuanljph", 64));
}

pub fn solve_part_two() {
    println!("{}", get_stretched_index_key("cuanljph", 64));
}