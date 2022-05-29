use crate::utils::remove;
use std::collections::HashSet;
use queues::*;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Element {
    Generator(u64),
    Microchip(u64)
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct State {
    elevator: usize,
    floors: [Vec<Element>; 4]
}

impl State {

    fn new() -> Self {
        Self {
            elevator: 0,
            floors: [vec![], vec![], vec![], vec![]]
        }
    }

    fn place(&mut self, floor: usize, elements: &[Element]) {
        for &element in elements {
            self.floors[floor].push(element);
        }
        self.floors[floor].sort();
    }

    fn get_updated(&self, updated_elevator: usize, first: Option<Element>, second: Option<Element>) -> Self {
        let mut result = self.clone();
        if let Some(first_element) = first {
            remove(&mut result.floors[result.elevator], first_element);
        }
        if let Some(second_element) = second {
            remove(&mut result.floors[result.elevator], second_element);
        }
        result.floors[result.elevator].sort();
        result.elevator = updated_elevator;
        if let Some(first_element) = first {
            result.floors[result.elevator].push(first_element);
        }
        if let Some(second_element) = second {
            result.floors[result.elevator].push(second_element);
        }
        result.floors[result.elevator].sort();
        result
    }

    fn is_microchip_safe(microchip_id: u64, elements: &[Element]) -> bool {
        let has_generators = elements.iter()
            .filter(|&element| if let Element::Generator(_) = element { true } else { false })
            .count() > 0;
        if !has_generators {
            true
        } else {
            elements.iter()
                .find(|&&element| {
                    if let Element::Generator(generator_id) = element {
                        generator_id == microchip_id
                    } else {
                        false
                    }
                })
                .is_some()
        }
    }

    fn is_floor_safe(elements: &[Element]) -> bool {
        let microchips_ids = elements.iter()
            .filter_map(|&element| {
                if let Element::Microchip(id) = element {
                    Some(id)
                } else {
                    None
                }
            });
        for microchip_id in microchips_ids {
            if !Self::is_microchip_safe(microchip_id, elements) {
                return false;
            }
        }
        true
    }

    fn is_move_safe(&self, next_elevator: usize, first: Option<Element>, second: Option<Element>) -> bool {
        let mut current_floor = self.floors[self.elevator].clone();
        let mut next_floor = self.floors[next_elevator].clone();
        if let Some(first_element) = first { remove(&mut current_floor, first_element) }
        if let Some(second_element) = second { remove(&mut current_floor, second_element) }
        if let Some(first_element) = first { next_floor.push(first_element) }
        if let Some(second_element) = second { next_floor.push(second_element) }
        Self::is_floor_safe(&current_floor) && Self::is_floor_safe(&next_floor)
    }

    fn is_goal(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }
}

fn get_new_states(state: &State, states: &HashSet<State>) -> Vec<State> {
    let mut result: Vec<State> = Vec::new();
    for &elevator_offset in &[-1i64, 1i64] {
        let signed_updated_elevator = (state.elevator as i64) + elevator_offset;
        if signed_updated_elevator >= 0 && signed_updated_elevator < 4 {
            let updated_elevator = signed_updated_elevator as usize;
            for &element in &state.floors[state.elevator] {
                if state.is_move_safe(updated_elevator, Some(element), None) {
                    let updated_state = state.get_updated(updated_elevator, Some(element), None);
                    if !states.contains(&updated_state) {
                        result.push(updated_state);
                    }
                }
            }
            for (first_index, &first_element) in state.floors[state.elevator].iter().enumerate() {
                for (second_index, &second_element) in state.floors[state.elevator].iter().enumerate() {
                    if first_index != second_index {
                        if state.is_move_safe(updated_elevator, Some(first_element), Some(second_element)) {
                            let updated_state = state.get_updated(updated_elevator, Some(first_element), Some(second_element));
                            if !states.contains(&updated_state) {
                                result.push(updated_state);
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

fn get_min_steps(state: &State) -> u64 {
    let mut states: HashSet<State> = HashSet::new();
    states.insert(state.clone());
    let mut queue: Queue<(State, u64)> = Queue::new();
    queue.add((state.clone(), 0));
    while let Some((current_state, steps)) = queue.remove().ok() {
        if current_state.is_goal() {
            return steps;
        }
        let new_states = get_new_states(&current_state, &states);
        for new_state in new_states {
            queue.add((new_state.clone(), steps + 1));
            states.insert(new_state.clone());
        }
    }
    0
}

pub fn test() {
    let mut state = State::new();
    state.place(0, &[Element::Microchip(0), Element::Microchip(1)]);
    state.place(1, &[Element::Generator(0)]);
    state.place(2, &[Element::Generator(1)]);
    println!("{}", get_min_steps(&state));
}