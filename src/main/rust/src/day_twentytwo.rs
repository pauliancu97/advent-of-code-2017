use std::collections::HashMap;
use std::ops::{Index, IndexMut, Add};
use crate::utils::read_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged
}

impl NodeState {
    fn get_next(&self) -> Self {
        match self {
            &NodeState::Clean => NodeState::Weakened,
            &NodeState::Weakened => NodeState::Infected,
            &NodeState::Infected => NodeState::Flagged,
            &NodeState::Flagged => NodeState::Clean
        }
    }
}

struct MapNodeState {
    data: HashMap<(i64, i64), NodeState>
}

impl Index<(i64, i64)> for MapNodeState {
    type Output = NodeState;

    fn index(&self, coordinates: (i64, i64)) -> &Self::Output {
        self.data.get(&coordinates).unwrap_or(&NodeState::Clean)
    }
}

impl IndexMut<(i64, i64)> for MapNodeState {
    fn index_mut(&mut self, coordinates: (i64, i64)) -> &mut Self::Output {
        self.data.entry(coordinates).or_insert(NodeState::Clean)
    }
}

impl MapNodeState {

    fn read_map(path: &str) -> Self {
        let strings = read_lines(path).unwrap_or(vec![]);
        Self::from_strings(&strings)
    }

    fn from_strings(strings: &[String]) -> Self {
        let height = strings.len() as i64;
        let width = strings[0].len() as i64;
        let mut map = MapNodeState { data: HashMap::new() };
        for y_offset in 0..height {
            for x_offset in 0..width {
                let y = - height / 2 + y_offset;
                let x = - width / 2 + x_offset;
                let cell_char = strings[y_offset as usize].as_bytes()[x_offset as usize];
                let is_infected = cell_char == b'#';
                map[(x, y)] = if is_infected { NodeState::Infected } else { NodeState::Clean }; 
            }
        }
        map
    }

}

struct Map {
    data: HashMap<(i64, i64), bool>
}

impl Index<(i64, i64)> for Map {
    type Output = bool;

    fn index(&self, coordinates: (i64, i64)) -> &Self::Output {
        self.data.get(&coordinates).unwrap_or(&false)
    }
}

impl IndexMut<(i64, i64)> for Map {
    fn index_mut(&mut self, coordinates: (i64, i64)) -> &mut Self::Output {
        self.data.entry(coordinates).or_insert(false)
    }
}

impl Map {

    fn read_map(path: &str) -> Self {
        let strings = read_lines(path).unwrap_or(vec![]);
        Self::from_strings(&strings)
    }

    fn from_strings(strings: &[String]) -> Self {
        let height = strings.len() as i64;
        let width = strings[0].len() as i64;
        let mut map = Map { data: HashMap::new() };
        for y_offset in 0..height {
            for x_offset in 0..width {
                let y = - height / 2 + y_offset;
                let x = - width / 2 + x_offset;
                let cell_char = strings[y_offset as usize].as_bytes()[x_offset as usize];
                let is_infected = cell_char == b'#';
                map[(x, y)] = is_infected;
            }
        }
        map
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    x: i64,
    y: i64
}

impl Coordinates {
    fn new() -> Self {
        Coordinates { x: 0, y: 0 }
    }
}

impl Add<Coordinates> for Coordinates {
    type Output = Self;
    fn add(self, rhs: Coordinates) -> Self {
        Coordinates { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl From<(i64, i64)> for Coordinates {
    fn from(tuple: (i64, i64)) -> Self {
        let (x, y) = tuple;
        Coordinates { x, y }
    }
}

impl From<Coordinates> for (i64, i64) {
    fn from(coordinates: Coordinates) -> Self {
        (coordinates.x, coordinates.y)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Right,
    Left
}

impl Direction {
    fn get_turn_left(&self) -> Self {
        match self {
            &Direction::Up => Direction::Left,
            &Direction::Down => Direction::Right,
            &Direction::Right => Direction::Up,
            &Direction::Left => Direction::Down
        }
    }

    fn get_turn_right(&self) -> Self {
        match self {
            &Direction::Up => Direction::Right,
            &Direction::Down => Direction::Left,
            &Direction::Right => Direction::Down,
            &Direction::Left => Direction::Up
        }
    }

    fn get_reversed(&self) -> Self {
        match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Right => Direction::Left,
            &Direction::Left => Direction::Right
        }
    }

    fn get_coordinates(&self) -> Coordinates {
        match self {
            &Direction::Up => Coordinates { x: 0, y: -1 },
            &Direction::Down => Coordinates { x: 0, y: 1 },
            &Direction::Right => Coordinates { x: 1, y: 0 },
            &Direction::Left => Coordinates { x: -1, y: 0 }
        }
    }
}

struct VirusCarrier {
    coordinates: Coordinates,
    direction: Direction
}

impl VirusCarrier {

    fn new() -> Self {
        VirusCarrier { coordinates: Coordinates::new(), direction: Direction::Up }
    }

    fn update(&mut self, map: &mut Map) -> bool {
        let mut has_infected = false;
        if map[self.coordinates.into()] {
            self.direction = self.direction.get_turn_right();
            map[self.coordinates.into()] = false;
            has_infected = false;
        } else {
            self.direction = self.direction.get_turn_left();
            map[self.coordinates.into()] = true;
            has_infected = true;
        }
        self.coordinates = self.coordinates + self.direction.get_coordinates();
        has_infected
    }

    fn update_node_states(&mut self, map: &mut MapNodeState) -> bool {
        self.direction = match map[self.coordinates.into()] {
            NodeState::Clean => self.direction.get_turn_left(),
            NodeState::Weakened => self.direction,
            NodeState::Infected => self.direction.get_turn_right(),
            NodeState::Flagged => self.direction.get_reversed()
        };
        let next_node_state = map[self.coordinates.into()].get_next();
        let has_infected = next_node_state == NodeState::Infected;
        map[self.coordinates.into()] = next_node_state;
        self.coordinates = self.coordinates + self.direction.get_coordinates();
        has_infected
    }
}

pub fn solve_part_one(num_iter: u64) {
    let mut map = Map::read_map("day_twentytwo.txt");
    let mut virus_carrier = VirusCarrier::new();
    let mut num_infections: u64 = 0;
    for _ in 0..num_iter {
        let has_infected = virus_carrier.update(&mut map);
        if has_infected {
            num_infections += 1;
        }
    }
    println!("{}", num_infections);
}

pub fn solve_part_two(num_iter: u64) {
    let mut map = MapNodeState::read_map("day_twentytwo.txt");
    let mut virus_carrier = VirusCarrier::new();
    let mut num_infections: u64 = 0;
    for _ in 0..num_iter {
        let has_infected = virus_carrier.update_node_states(&mut map);
        if has_infected {
            num_infections += 1;
        }
    }
    println!("{}", num_infections);
}