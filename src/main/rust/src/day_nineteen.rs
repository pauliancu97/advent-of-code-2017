use crate::matrix::Matrix;
use crate::utils::read_lines;

static NORTH: (isize, isize) = (-1, 0);
static EAST: (isize, isize) = (0, 1);
static SOUTH: (isize, isize) = (1, 0);
static WEST: (isize, isize) = (0, -1);
static DIRECTIONS: [(isize, isize); 4] = [NORTH, EAST, SOUTH, WEST];

fn get_oposite(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        (-1, 0) => SOUTH,
        (0, 1) => WEST,
        (1, 0) => NORTH,
        (0, -1) => EAST,
        _ => NORTH
    }
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Path,
    PathWithId(char)
}

impl Tile {
    fn is_path(&self) -> bool {
        match &self {
            Tile::Path => true,
            Tile::PathWithId(_) => true,
            Tile::Empty => false
        }
    }
}

fn get_tile_map(strings: &[String]) -> Matrix<Tile> {
    let rows = strings.len();
    let cols = strings.first().map(|string| string.len()).unwrap_or(0);
    let mut tile_map: Matrix<Tile> = Matrix::new(rows, cols, Tile::Empty);
    for (row, string) in strings.iter().enumerate() {
        for (col, char) in string.chars().enumerate() {
            let tile = match char {
                ' ' => Tile::Empty,
                '|' | '-' | '+' => Tile::Path,
                'A'..='Z' => Tile::PathWithId(char),
                _ => Tile::Empty
            };
            tile_map.set(row, col, tile);
        }
    }
    tile_map
}

struct Packet {
    row: usize,
    col: usize,
    direction: (isize, isize)
}

impl Packet {
    fn get_initial_packet(tile_map: &Matrix<Tile>) -> Self {
        for row in 0..tile_map.rows {
            for col in 0..tile_map.cols {
                if tile_map.get(row, col).is_path() {
                    let (row_offset, col_offset) = EAST;
                    let row_offseted = row as isize + row_offset;
                    let col_offseted = col as isize + col_offset;
                    if tile_map.has_coordinate(row_offseted, col_offseted) && tile_map.get(row_offseted as usize, col_offseted as usize).is_path() {
                        return Packet { row, col, direction: EAST };
                    }
                    let (row_offset, col_offset) = SOUTH;
                    let row_offseted = row as isize + row_offset;
                    let col_offseted = col as isize + col_offset;
                    if tile_map.has_coordinate(row_offseted, col_offseted) && tile_map.get(row_offseted as usize, col_offseted as usize).is_path() {
                        return Packet { row, col, direction: SOUTH };
                    }
                }
            }
        }
        panic!("");
    }
    
    fn update(&mut self, tile_map: &Matrix<Tile>) -> bool {
        let mut updated = false;
        let (row_offset, col_offset) = self.direction;
        let row_offseted = self.row as isize + row_offset;
        let col_offseted = self.col as isize + col_offset;
        if tile_map.has_coordinate(row_offseted, col_offseted) && tile_map.get(row_offseted as usize, col_offseted as usize).is_path() {
            updated = true;
            self.row = row_offseted as usize;
            self.col = col_offseted as usize;
        } else {
            for &direction in &DIRECTIONS {
                if direction != self.direction && direction != get_oposite(self.direction) {
                    let (row_offset, col_offset) = direction;
                    let row_offseted = self.row as isize + row_offset;
                    let col_offseted = self.col as isize + col_offset;
                    if tile_map.has_coordinate(row_offseted, col_offseted) && tile_map.get(row_offseted as usize, col_offseted as usize).is_path() {
                        updated = true;
                        self.row = row_offseted as usize;
                        self.col = col_offseted as usize;
                        self.direction = direction;
                        break;
                    }
                }
            }
        }
        updated
    }

    fn get_coordinates(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

fn get_packet_trail(tile_map: &Matrix<Tile>) -> Vec<(usize, usize)> {
    let mut coordinates: Vec<(usize, usize)> = Vec::new();
    let mut packet = Packet::get_initial_packet(tile_map);
    coordinates.push(packet.get_coordinates());
    while packet.update(tile_map) {
        coordinates.push(packet.get_coordinates());
    }
    coordinates
}

fn get_packet_ids_trail(tile_map: &Matrix<Tile>) -> String {
    let mut result = String::from("");
    let coordinates = get_packet_trail(tile_map);
    for (row, col) in coordinates {
        if let Tile::PathWithId(chr) = tile_map.get(row, col) {
            result.push(chr);
        }
    }
    result
}

pub fn solve_part_one() {
    let strings = read_lines("day_nineteen.txt").unwrap_or(vec![]);
    let tile_map = get_tile_map(&strings);
    println!("{}", get_packet_ids_trail(&tile_map));
}