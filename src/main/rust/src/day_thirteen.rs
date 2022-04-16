use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use crate::utils::read_lines;

#[derive(Clone)]
enum Direction {
    Up, 
    Down
}

#[derive(Clone)]
struct ScannerState {
    position: u64,
    depth: u64,
    direction: Direction,
    layer: u64
}

struct Scanner {
    layer: u64,
    depth: u64
}

impl Scanner {
    fn from_string(string: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+): (\d+)").unwrap();
        }
        let captures = RE.captures(string)?;
        let layer = captures.get(1)
            .and_then(|re_match| re_match.as_str().parse::<u64>().ok())?;
        let depth = captures.get(2)
            .and_then(|re_match| re_match.as_str().parse::<u64>().ok())?;
        Some(Scanner{ layer, depth })
    }
}

impl ScannerState {

    fn from_string(string: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+): (\d+)").unwrap();
        }
        let captures = RE.captures(string)?;
        let layer = captures.get(1)
            .and_then(|re_match| re_match.as_str().parse::<u64>().ok())?;
        let depth = captures.get(2)
            .and_then(|re_match| re_match.as_str().parse::<u64>().ok())?;
        Some(
            ScannerState {
                position: 0,
                depth,
                layer,
                direction: Direction::Down
            }
        )
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Down => {
                if self.position == self.depth - 1 {
                    self.position -= 1;
                    self.direction = Direction::Up;
                } else {
                    self.position += 1;
                }
            },
            Direction::Up => {
                if self.position == 0 {
                    self.position += 1;
                    self.direction = Direction::Down;
                } else {
                    self.position -= 1;
                }
            }
        }
    }

    fn is_at_top(&self) -> bool {
        self.position == 0
    }

    fn get_severity(&self) -> u64 {
        self.depth * self.layer
    }
} 

fn get_severity_level(scanners: &mut HashMap<u64, ScannerState>) -> u64 {
    let mut severity_level: u64 = 0;
    let num_layers: u64 = scanners.keys().max().map(|u_ref| *u_ref).unwrap_or(0);
    for packet_layer in 0..=num_layers {
        if let Some(scanner) = scanners.get(&packet_layer) {
            if scanner.is_at_top() {
                severity_level += scanner.get_severity();
            }
        }
        for scanner in scanners.values_mut() {
            scanner.update();
        }
    }
    severity_level
}

fn read_scanners_state(path: &str) -> Vec<ScannerState> {
    read_lines(path)
        .map_or(
            vec![],
            |strings| {
                strings.into_iter()
                    .filter_map(|string| ScannerState::from_string(&string))
                    .collect()
            }
        )
}

fn read_scanners(path: &str) -> Vec<Scanner> {
    read_lines(path)
        .map_or(
            vec![],
            |strings| {
                strings.into_iter()
                    .filter_map(|string| Scanner::from_string(&string))
                    .collect()
            }
        )
}

fn to_scanners_map(vector: Vec<ScannerState>) -> HashMap<u64, ScannerState> {
    let mut map: HashMap<u64, ScannerState> = HashMap::new();
    for scanner in vector {
        map.insert(scanner.layer, scanner);
    }
    map
}

fn update_scanners(scanners: &mut HashMap<u64, ScannerState>) {
    for scanner in scanners.values_mut() {
        scanner.update();
    }
}

fn is_probe_caught(scanners: &HashMap<u64, ScannerState>) -> bool {
    let num_level = scanners
        .keys()
        .map(|level_ref| *level_ref)
        .max()
        .unwrap_or(0);
    let mut current_scanners = scanners.clone();
    for probe_level in 0..=num_level {
        if let Some(scanner) = current_scanners.get(&probe_level) {
            if scanner.is_at_top() {
                return true;
            }
        }
        update_scanners(&mut current_scanners);
    }
    false
}

fn get_delay(scanners: &[Scanner]) -> u64 {
    let mut delay: u64 = 0;
    loop {
        let mut is_caught = false;
        for scanner in scanners {
            let current_time = delay + scanner.layer;
            let period = 2 * (scanner.depth - 1);
            if current_time % period == 0 {
                is_caught = true;
            }
        }
        if is_caught {
            delay += 1;
        } else {
            break;
        }
    }
    delay
}



pub fn solve_part_one() {
    let scanners_vec = read_scanners_state("day_thirteen.txt");
    let mut scanners_map = to_scanners_map(scanners_vec);
    println!("{}", get_severity_level(&mut scanners_map));
}

pub fn solve_part_two() {
    let scanners = read_scanners("day_thirteen.txt");
    println!("{}", get_delay(&scanners));
}
