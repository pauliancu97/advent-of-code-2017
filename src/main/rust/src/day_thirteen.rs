use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use crate::utils::read_lines;

enum Direction {
    Up, 
    Down
}

struct ScannerState {
    position: u64,
    depth: u64,
    direction: Direction,
    layer: u64
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

fn read_scanners(path: &str) -> Vec<ScannerState> {
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

fn to_scanners_map(vector: Vec<ScannerState>) -> HashMap<u64, ScannerState> {
    let mut map: HashMap<u64, ScannerState> = HashMap::new();
    for scanner in vector {
        map.insert(scanner.layer, scanner);
    }
    map
}

pub fn solve_part_one() {
    let scanners_vec = read_scanners("day_thirteen.txt");
    let mut scanners_map = to_scanners_map(scanners_vec);
    println!("{}", get_severity_level(&mut scanners_map));
}
