use lazy_static::lazy_static;
use regex::Regex;
use crate::utils::read_lines;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
    available: u32,
    use_per: u32
}

impl Node {
    fn from_string(string: &str) -> Option<Node> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
        }
        let captures = REGEX.captures(string)?;
        let x = captures.get(1)?.as_str().parse::<u32>().ok()?;
        let y = captures.get(2)?.as_str().parse::<u32>().ok()?;
        let size = captures.get(3)?.as_str().parse::<u32>().ok()?;
        let used = captures.get(4)?.as_str().parse::<u32>().ok()?;
        let available = captures.get(5)?.as_str().parse::<u32>().ok()?;
        let use_per = captures.get(6)?.as_str().parse::<u32>().ok()?;
        Some(Node { x, y, size, used, available, use_per })
    }

    fn is_empty(&self) -> bool {
        self.used == 0
    }

    fn can_fit(&self, other: &Self) -> bool {
        self.available >= other.used
    }
}

fn get_num_viable_node_pairs(nodes: &[Node]) -> u64 {
    let mut result: u64 = 0;
    for first_node in nodes {
        for second_node in nodes {
            if first_node != second_node && !first_node.is_empty() && second_node.can_fit(first_node) {
                result += 1;
            }
        }
    }
    result
}

fn read_nodes(path: &str) -> Vec<Node> {
    read_lines(path)
        .map_or(
            vec![], 
            |strings| {
                strings.iter()
                    .filter_map(|string| Node::from_string(string))
                    .collect()
            }
        )
}

pub fn solve_part_one() {
    let nodes = read_nodes("day_twentytwo_2016.txt");
    println!("{}", get_num_viable_node_pairs(&nodes));
}