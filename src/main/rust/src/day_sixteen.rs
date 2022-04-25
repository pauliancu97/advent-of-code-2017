use lazy_static::lazy_static;
use regex::Regex;
use std::str;
use crate::utils::read_lines;

#[derive(Clone, Copy)]
enum DanceMove {
    Spin { length: usize },
    Exchange { first: usize, second: usize },
    Partner { first: char, second: char }
}

impl DanceMove {
    fn from_string(string: &str) -> Option<Self> {
        lazy_static! {
            static ref SPIN_RE: Regex = Regex::new(r"s(\d+)").unwrap();
            static ref EXCHANGE_RE: Regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
            static ref PARTNER_RE: Regex = Regex::new(r"p([a-p])/([a-p])").unwrap();
        }
        if let Some(captures) = SPIN_RE.captures(string) {
            let length = captures.get(1)
                .and_then(|re_match| {
                    re_match.as_str().parse::<usize>().ok()
                })
                .unwrap_or(0);
            return Some(DanceMove::Spin { length });
        }
        if let Some(captures) = EXCHANGE_RE.captures(string) {
            let first = captures.get(1)
                .and_then(|re_match| {
                    re_match.as_str().parse::<usize>().ok()
                })
                .unwrap_or(0);
            let second = captures.get(2)
                .and_then(|re_match| {
                    re_match.as_str().parse::<usize>().ok()
                })
                .unwrap_or(0);
            return Some(DanceMove::Exchange { first, second } );
        }
        if let Some(captures) = PARTNER_RE.captures(string) {
            let first = captures.get(1)
                .and_then(|re_match| re_match.as_str().chars().next())
                .unwrap_or(' ');
            let second = captures.get(2)
                .and_then(|re_match| re_match.as_str().chars().next())
                .unwrap_or(' ');
            return Some(DanceMove::Partner { first, second })
        }
        None
    }
}

fn get_after_spin(string: &str, length: usize) -> String {
    let start = string.len() - length;
    let end = string.len();
    let end_part = &string[start..end];
    let start_part = &string[0..start];
    String::from(end_part) + start_part
}

fn get_after_exchange(string: &str, first: usize, second: usize) -> String {
    let mut bytes: Vec<u8> = string.as_bytes().iter().map(|x| *x).collect();
    let temp = bytes[first];
    bytes[first] = bytes[second];
    bytes[second] = temp;
    let result = str::from_utf8(&bytes).unwrap_or("");
    String::from(result)
}

fn get_after_partner(string: &str, first: char, second: char) -> String {
    let mut bytes: Vec<u8> = string.as_bytes().iter().map(|x| *x).collect();
    let first_code = first as u8;
    let second_code = second as u8;
    let first_index = bytes.iter().position(|&byte| byte == first_code).unwrap_or(0);
    let second_index = bytes.iter().position(|&byte| byte == second_code).unwrap_or(0);
    let temp = bytes[first_index];
    bytes[first_index] = bytes[second_index];
    bytes[second_index] = temp;
    let result = str::from_utf8(&bytes).unwrap_or("");
    String::from(result)
}

fn get_after_dance_move(string: &str, dance_move: &DanceMove) -> String {
    match dance_move {
        &DanceMove::Spin { length } => get_after_spin(string, length),
        &DanceMove::Exchange { first, second } => get_after_exchange(string, first, second),
        &DanceMove::Partner { first, second } => get_after_partner(string, first, second)
    }
}

fn get_after_dance_moves(string: &str, dance_moves: &[DanceMove]) -> String {
    dance_moves.iter()
        .fold(String::from(string), |acc, dance_move| get_after_dance_move(&acc, dance_move))
}

fn get_dance_moves(string: &str) -> Vec<DanceMove> {
    string.split(",")
        .filter_map(|move_str| DanceMove::from_string(move_str))
        .collect()
}

fn read_dance_moves(path: &str) -> Vec<DanceMove> {
    read_lines(path)
        .map(|strings| {
            let string = strings[0].clone();
            get_dance_moves(&string)
        })
        .unwrap_or(vec![])
}

pub fn test() {
    let dance_moves_str = "s1,x3/4,pe/b";
    let string = "abcde";
    println!("{}", get_after_dance_moves(string, &get_dance_moves(dance_moves_str)));

}

pub fn solve_part_one() {
    let dance_moves = read_dance_moves("day_sixteen.txt");
    let string = "abcdefghijklmnop";
    println!("{}", get_after_dance_moves(string, &dance_moves));
}