use lazy_static::lazy_static;
use regex::Regex;
use crate::utils::read_lines;

#[derive(Clone, Copy)]
struct Disk {
    position: u64,
    num_positions: u64
}

impl Disk {

    fn from_string(string: &str) -> Option<Self> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+)\.$").unwrap();
        }
        let captures = REGEX.captures(string)?;
        let num_positions = captures.get(1)?.as_str().parse::<u64>().ok()?;
        let position = captures.get(2)?.as_str().parse::<u64>().ok()?;
        Some(Disk::new(num_positions, position))
    }

    fn new(num_positions: u64, position: u64) -> Self {
        Disk { position, num_positions }
    }

    fn is_open_at_time(&self, time: u64) -> bool {
        let remaining_time = time % self.num_positions;
        let final_position = (self.position + remaining_time) % self.num_positions;
        final_position == 0
    }
}

fn is_passing_at_time(disks: &[Disk], time: u64) -> bool {
    for (index, disk) in disks.iter().enumerate() {
        if !disk.is_open_at_time(time + (index as u64) + 1) {
            return false;
        }
    }
    true
}

fn get_fall_time(disks: &[Disk]) -> u64 {
    let mut fall_time: u64 = 0;
    while !is_passing_at_time(disks, fall_time) {
        fall_time += 1;
    }
    fall_time
}

fn read_disks(path: &str) -> Vec<Disk> {
    read_lines(path)
        .map_or(
            vec![], 
            |strings| {
                strings.iter()
                    .filter_map(|string| Disk::from_string(string))
                    .collect()
            }
        )
}

pub fn solve_part_one() {
    let disks = read_disks("day_fifteen_2016.txt");
    let answer = get_fall_time(&disks);
    println!("{}", answer);
}