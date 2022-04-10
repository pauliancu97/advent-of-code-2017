use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;
use lazy_static::lazy_static;
use crate::utils::read_lines;
use queues::*;

struct Node {
    id: u64,
    neighbours_ids: Vec<u64>
}

impl Node {
    pub fn from_string(string: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) <-> ((?:\d+(?:, )?)+)").unwrap();
        }
        let captures = RE.captures(string)?;
        let node_id = captures.get(1)
            .map(|re_match| re_match.as_str())
            .and_then(|capture| capture.parse::<u64>().ok())?;
        let neighbours_ids: Vec<u64> = captures.get(2)
            .map(|re_match| re_match.as_str())
            .map(|capture| {
                capture
                    .split(", ")
                    .filter_map(|id_str| id_str.parse::<u64>().ok())
                    .collect()
            })?;
        Some(Node{id: node_id, neighbours_ids})
    }
}

fn read_nodes(path: &str) -> Vec<Node> {
    read_lines(path)
        .map_or(
            vec![],
            |lines| {
                lines
                    .into_iter()
                    .filter_map(|line| Node::from_string(&line))
                    .collect()
            }
        )
}

fn to_node_map(nodes: Vec<Node>) -> HashMap<u64, Node> {
    let mut map: HashMap<u64, Node> = HashMap::new();
    for node in nodes {
        map.insert(node.id, node);
    }
    map
}

fn get_connected_component(nodes_map: &HashMap<u64, Node>, root: u64) -> HashSet<u64> {
    let mut visited: HashSet<u64> = HashSet::new();
    let mut queue: Queue<u64> = queue![root];
    visited.insert(root);
    while let Some(node) = queue.remove().ok() {
        for neighbour in &nodes_map[&node].neighbours_ids {
            if !visited.contains(neighbour) {
                visited.insert(*neighbour);
                queue.add(*neighbour);
            }
        }
    }
    visited
}

fn get_num_connected_components(nodes_map: &HashMap<u64, Node>) -> u64 {
    let mut num_connected_components: u64 = 0;
    let num_nodes = nodes_map.len();
    let mut visited: HashSet<u64> = HashSet::new();
    while visited.len() != num_nodes {
        let root: u64 = nodes_map
            .keys()
            .find(|node_id| !visited.contains(*node_id))
            .map(|node_id| *node_id)
            .unwrap_or(0);
        let current_visited = get_connected_component(nodes_map, root);
        for node in current_visited {
            visited.insert(node);
        }
        num_connected_components += 1;
    }
    num_connected_components
}

pub fn solve_part_one() {
    let nodes = read_nodes("day_twelve.txt");
    let nodes_map = to_node_map(nodes);
    let connected_component_size = get_connected_component(&nodes_map, 0).len();
    println!("{:?}", connected_component_size);
}

pub fn solve_part_two() {
    let nodes = read_nodes("day_twelve.txt");
    let nodes_map = to_node_map(nodes);
    let num_connected_components = get_num_connected_components(&nodes_map);
    println!("{:?}", num_connected_components);
}
