use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use std::cmp::max;
use crate::utils::read_lines;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Output {
    OutputBin(usize),
    Bot(usize)
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Value {
        value: usize,
        bot: usize
    },
    LowHigh {
        bot: usize,
        low_output: Output,
        high_output: Output
    }
}

impl Instruction {
    fn get_value_instr(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"value (\d+) goes to bot (\d+)").unwrap();
        }
        let captures = RE.captures(string)?;
        let value: usize = captures.get(1)?.as_str().parse().ok()?;
        let bot: usize = captures.get(2)?.as_str().parse().ok()?;
        Some(Instruction::Value { value, bot })
    }

    fn get_low_high_instr(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"bot (\d+) gives low to (output|bot) (\d+) and high to (output|bot) (\d+)").unwrap();
        }
        let captures = RE.captures(string)?;
        let bot: usize = captures.get(1)?.as_str().parse().ok()?;
        let low_output_id: usize = captures.get(3)?.as_str().parse().ok()?;
        let low_output_str_type = captures.get(2)?.as_str();
        let low_output = match low_output_str_type {
            "output" => Some(Output::OutputBin(low_output_id)),
            "bot" => Some(Output::Bot(low_output_id)),
            _ => None
        }?;
        let high_output_id: usize = captures.get(5)?.as_str().parse().ok()?;
        let high_output_str_type = captures.get(4)?.as_str();
        let high_output = match high_output_str_type {
            "output" => Some(Output::OutputBin(high_output_id)),
            "bot" => Some(Output::Bot(high_output_id)),
            _ => None
        }?;
        Some(Instruction::LowHigh { bot, low_output, high_output })
    }

    fn get_instruction(string: &str) -> Option<Instruction> {
        Self::get_value_instr(string).or(Self::get_low_high_instr(string))
    }
}

fn read_instructions(path: &str) -> Vec<Instruction> {
    read_lines(path)
        .map_or(
            vec![],
            |strings| {
                strings.iter()
                    .filter_map(|string| Instruction::get_instruction(string))
                    .collect()
            }
        )
}

struct State {
    bots: HashMap<usize, Vec<usize>>,
    bins: HashMap<usize, Vec<usize>>,
    first_value: usize,
    second_value: usize
}

fn remove<T:Eq + PartialEq>(vec: &mut Vec<T>, element: T) {
    if let Some(index) = vec.iter().position(|val| *val == element) {
        vec.remove(index);
    }
}

impl State {

    fn new(first_value: usize, second_value: usize) -> Self {
        State {
            bots: HashMap::new(),
            bins: HashMap::new(),
            first_value,
            second_value
        }
    }

    fn initialize(&mut self, instructions: &[Instruction]) {
        let mut bots: HashSet<usize> = HashSet::new();
        for instruction in instructions {
            if let &Instruction::Value { bot, value: _ } = instruction {
                bots.insert(bot);
            }
            if let &Instruction::LowHigh { bot, low_output, high_output } = instruction {
                bots.insert(bot);
                if let Output::Bot(bot) = low_output {
                    bots.insert(bot);
                }
                if let Output::Bot(bot) = high_output {
                    bots.insert(bot);
                }
            }
        }
        for bot in bots{
            self.bots.insert(bot, vec![]);
        }
        for instruction in instructions {
            if let &Instruction::Value { bot, value } = instruction {
                if let Some(values) = self.bots.get_mut(&bot) {
                    values.push(value);
                } else {
                    self.bots.insert(bot, vec![value]);
                }
            }
        }
        let mut bins: HashSet<usize> = HashSet::new();
        for instruction in instructions {
            if let &Instruction::LowHigh { bot: _, low_output, high_output } = instruction {
                if let Output::OutputBin(bin) = low_output {
                    bins.insert(bin);
                }
                if let Output::OutputBin(bin) = high_output {
                    bins.insert(bin);
                }
            }
        }
        for bin in bins {
            self.bins.insert(bin, vec![]);
        }
    }

    fn update(&mut self, instruction: &[Instruction]) -> Option<usize> {
        let (bot, low_output, high_output) = self.bots.iter()
            .find_map(|(&bot_id, values)| {
                if values.len() >= 2 {
                    let instr_opt = instruction.iter()
                        .find_map(|instr| {
                            if let &Instruction::LowHigh { bot, low_output, high_output } = instr {
                                if bot == bot_id {
                                    Some((low_output, high_output))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        });
                    if let Some((low_output, high_output)) = instr_opt {
                        Some((bot_id, low_output, high_output))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }).unwrap();
        let max_value = self.bots[&bot].iter().max().map(|&val| val).unwrap_or(0);
        let min_value = self.bots[&bot].iter().min().map(|&val| val).unwrap_or(0);
        if let Some(values) = self.bots.get_mut(&bot) {
            remove(values, max_value);
            remove(values, min_value);
        }
        match low_output {
            Output::Bot(output_bot) => {
                if let Some(values) = self.bots.get_mut(&output_bot) {
                    values.push(min_value);
                }
            }
            Output::OutputBin(output_bin) => {
                if let Some(values) = self.bins.get_mut(&output_bin) {
                    values.push(min_value);
                }
            }
        };
        match high_output {
            Output::Bot(output_bot) => {
                if let Some(values) = self.bots.get_mut(&output_bot) {
                    values.push(max_value);
                }
            }
            Output::OutputBin(output_bin) => {
                if let Some(values) = self.bins.get_mut(&output_bin) {
                    values.push(max_value);
                }
            }
        };
        if max_value == max(self.first_value, self.second_value) && min_value == min(self.first_value, self.second_value) {
            Some(bot)
        } else {
            None
        }
    }
}

pub fn solve_part_one() {
    let instructions = read_instructions("day_ten.txt");
    let mut state = State::new(17, 61);
    state.initialize(&instructions);
    loop {
        if let Some(bot) = state.update(&instructions) {
            println!("{}", bot);
            break;
        }
    }
}