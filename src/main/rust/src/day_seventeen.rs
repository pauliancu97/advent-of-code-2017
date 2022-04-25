use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use crate::utils::read_lines;
use std::sync::mpsc;
use std::thread;

#[derive(Copy, Clone)]
enum Operand {
    Value(i64),
    Register(u8)
}

#[derive(Copy, Clone)]
enum Instruction {
    Sound { operand: Operand },
    Set { dest: u8, value: Operand },
    Add { dest: u8, value: Operand },
    Mul { dest: u8, value: Operand },
    Mod { dest: u8, value: Operand },
    Recover { operand: Operand },
    Jump { test: Operand, offset: Operand }
}

impl Instruction {
    fn get_operand(string: &str) -> Operand {
        if let Some(value) = string.parse::<i64>().ok() {
            Operand::Value(value)
        } else {
            Operand::Register(string.as_bytes()[0])
        }
    }

    fn get_sound_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"snd ((?:[-+]?\d+)|\w)").unwrap();
        }
        let capture = RE.captures(string)?.get(1)?.as_str();
        Some(Instruction::Sound { operand: Self::get_operand(capture) })
    }

    fn get_set_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"set (\w) ((?:[-+]?\d+)|\w)").unwrap();
        }
        let register = RE.captures(string)?.get(1)?.as_str().as_bytes()[0];
        let second_capture = RE.captures(string)?.get(2)?.as_str();
        let value = Self::get_operand(second_capture);
        Some(Instruction::Set { dest: register, value })
    }

    fn get_add_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"add (\w) ((?:[-+]?\d+)|\w)").unwrap();
        }
        let register = RE.captures(string)?.get(1)?.as_str().as_bytes()[0];
        let second_capture = RE.captures(string)?.get(2)?.as_str();
        let value = Self::get_operand(second_capture);
        Some(Instruction::Add { dest: register, value })
    }

    fn get_mul_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"mul (\w) ((?:[-+]?\d+)|\w)").unwrap();
        }
        let register = RE.captures(string)?.get(1)?.as_str().as_bytes()[0];
        let second_capture = RE.captures(string)?.get(2)?.as_str();
        let value = Self::get_operand(second_capture);
        Some(Instruction::Mul { dest: register, value })
    }

    fn get_mod_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"mod (\w) ((?:[-+]?\d+)|\w)").unwrap();
        }
        let register = RE.captures(string)?.get(1)?.as_str().as_bytes()[0];
        let second_capture = RE.captures(string)?.get(2)?.as_str();
        let value = Self::get_operand(second_capture);
        Some(Instruction::Mod { dest: register, value })
    }

    fn get_recover_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"rcv ((?:[-+]?\d+)|\w)").unwrap();
        }
        let capture = RE.captures(string)?.get(1)?.as_str();
        Some(Instruction::Recover { operand: Self::get_operand(capture) })
    }

    fn get_jump_instruction(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"jgz (\w) ((?:[-+]?\d+)|\w)").unwrap();
        }
        let first_capture = RE.captures(string)?.get(1)?.as_str();
        let second_capture = RE.captures(string)?.get(2)?.as_str();
        let test = Self::get_operand(first_capture);
        let offset = Self::get_operand(second_capture);
        Some(Instruction::Jump { test, offset })
    }

    fn get_instruction(string: &str) -> Option<Instruction> {
        Self::get_sound_instruction(string)
            .or(Self::get_set_instruction(string))
            .or(Self::get_add_instruction(string))
            .or(Self::get_mul_instruction(string))
            .or(Self::get_mod_instruction(string))
            .or(Self::get_recover_instruction(string))
            .or(Self::get_jump_instruction(string))
    }
}

struct Computer {
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    registers: HashMap<u8, i64>,
    last_sound: Option<i64>,
    last_recoverd_sound: Option<i64>
}

impl Computer {
    fn new(instructions: &[Instruction]) -> Self {
        let mut registers: HashMap<u8, i64> = HashMap::new();
        for chr in b'a'..=b'z' {
            registers.insert(chr, 0);
        }
        let instructions_vec: Vec<_> = instructions.iter().map(|&instr| instr).collect();
        Computer {
            instruction_pointer: 0,
            instructions: instructions_vec,
            registers,
            last_sound: None,
            last_recoverd_sound: None
        }
    }

    fn get_operand_value(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Value(value) => value,
            Operand::Register(register) => self.registers[&register]
        }
    }

    fn set_register(&mut self, dest: u8, value: i64) {
        if let Some(mut_ref) = self.registers.get_mut(&dest) {
            *mut_ref = value;
        }
    }

    fn get_register(&self, dest: u8) -> i64 {
        self.registers.get(&dest).map_or(0, |&value| value)
    }

    fn execute(&mut self) {
        let mut has_jumped = false;
        match self.instructions[self.instruction_pointer] {
            Instruction::Sound { operand } => self.last_sound = Some(self.get_operand_value(operand)),
            Instruction::Set { dest, value: operation_value } => {
                let value = self.get_operand_value(operation_value);
                self.set_register(dest, value);
            },
            Instruction::Add { dest, value: operation_value } => {
                let value = self.get_operand_value(operation_value) + self.get_register(dest);
                self.set_register(dest, value);
            }
            Instruction::Mul { dest, value: operation_value } => {
                let value = self.get_operand_value(operation_value) * self.get_register(dest);
                self.set_register(dest, value);
            },
            Instruction::Mod { dest, value: operation_value } => {
                let value = self.get_register(dest) % self.get_operand_value(operation_value);
                self.set_register(dest, value);
            }
            Instruction::Recover { operand } => {
                if self.get_operand_value(operand) != 0 {
                    self.last_recoverd_sound = self.last_sound;
                }
            }
            Instruction::Jump {test, offset} => {
                if self.get_operand_value(test) > 0 {
                    has_jumped = true;
                    self.instruction_pointer = 
                        ((self.instruction_pointer as i64) + self.get_operand_value(offset)) as usize;
                }
            }
        }
        if !has_jumped {
            self.instruction_pointer += 1;
        }
    }

    fn get_last_recovered_sound(&mut self) -> i64 {
        while self.last_recoverd_sound.is_none() {
            self.execute();
        }
        self.last_recoverd_sound.unwrap_or(0)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum ProcessId {
    First,
    Second
}

impl ProcessId {
    fn other(&self) -> Self {
        match &self {
            ProcessId::First => ProcessId::Second,
            ProcessId::Second => ProcessId::First
        }
    }
}

#[derive(Copy, Clone)]
enum MainMessage {
    Blocked { id: ProcessId },
    Unblocked { id: ProcessId },
    Terminated { id: ProcessId },
    SentMessage { id: ProcessId }
}

#[derive(Copy, Clone)]
struct ProcessState {
    id: ProcessId,
    is_blocked: bool,
    is_terminated: bool,
    num_sent_msg: u64
}

impl ProcessState {
    fn new(id: ProcessId) -> Self {
        ProcessState {id, is_blocked: false, is_terminated: false, num_sent_msg: 0}
    }

    fn is_halted(&self) -> bool {
        self.is_blocked || self.is_terminated
    }

    fn update(&mut self, message: MainMessage) {
        match message {
            MainMessage::Blocked { id } if id == self.id => self.is_blocked = true,
            MainMessage::Unblocked { id } if id == self.id => self.is_blocked = false,
            MainMessage::Terminated { id } if id == self.id => self.is_terminated = true,
            MainMessage::SentMessage { id } if id == self.id => self.num_sent_msg += 1,
            _ => ()
        }
    }
}

struct Process {
    id: ProcessId,
    instruction_pointer: usize,
    instructions: Vec<Instruction>,
    registers: HashMap<u8, i64>,
    main_sender: mpsc::Sender<MainMessage>,
    other_sender: mpsc::Sender<i64>,
    receiver: mpsc::Receiver<i64>
}

impl Process {
    fn new(
        id: ProcessId, 
        instructions_slice: &[Instruction], 
        main_sender: mpsc::Sender<MainMessage>,
        other_sender: mpsc::Sender<i64>,
        receiver: mpsc::Receiver<i64>
    ) -> Self {
        let mut registers: HashMap<u8, i64> = HashMap::new();
        for chr in b'a'..=b'z' {
            registers.insert(chr, 0);
        }
        let value: i64 = match id {
            ProcessId::First => 0,
            ProcessId::Second => 1
        };
        registers.get_mut(&('p' as u8)).map(|mut_ref| *mut_ref = value);
        let instructions: Vec<Instruction> = instructions_slice.iter().map(|&instr| instr).collect();
        Process {
            id,
            instruction_pointer: 0,
            instructions,
            registers,
            main_sender,
            other_sender,
            receiver
        }
    }

    fn get_operand_value(&self, operand: Operand) -> i64 {
        match operand {
            Operand::Value(value) => value,
            Operand::Register(register) => self.registers[&register]
        }
    }

    fn set_register(&mut self, dest: u8, value: i64) {
        if let Some(mut_ref) = self.registers.get_mut(&dest) {
            *mut_ref = value;
        }
    }

    fn get_register(&self, dest: u8) -> i64 {
        self.registers.get(&dest).map_or(0, |&value| value)
    }

    fn execute(&mut self) {
        if self.instruction_pointer >= self.instructions.len() {
            self.main_sender.send(MainMessage::Terminated { id: self.id }).unwrap();
            return;
        }
        let mut has_jumped = false;
        match self.instructions[self.instruction_pointer] {
            Instruction::Sound { operand } => {
                self.main_sender.send(MainMessage::Unblocked { id: self.id.other() }).unwrap();
                self.main_sender.send(MainMessage::SentMessage{ id: self.id }).unwrap();
                let value = self.get_operand_value(operand);
                self.other_sender.send(value).unwrap();
            },
            Instruction::Set { dest, value: operation_value } => {
                let value = self.get_operand_value(operation_value);
                self.set_register(dest, value);
            },
            Instruction::Add { dest, value: operation_value } => {
                let value = self.get_operand_value(operation_value) + self.get_register(dest);
                self.set_register(dest, value);
            }
            Instruction::Mul { dest, value: operation_value } => {
                let value = self.get_operand_value(operation_value) * self.get_register(dest);
                self.set_register(dest, value);
            },
            Instruction::Mod { dest, value: operation_value } => {
                let value = self.get_register(dest) % self.get_operand_value(operation_value);
                self.set_register(dest, value);
            }
            Instruction::Recover { operand } => {
                self.main_sender.send(MainMessage::Blocked { id: self.id}).unwrap();
                let value = self.receiver.recv().unwrap();
                if let Operand::Register(register_char) = operand {
                    self.set_register(register_char, value);
                }
            }
            Instruction::Jump {test, offset} => {
                if self.get_operand_value(test) > 0 {
                    has_jumped = true;
                    let updated_instruction_pointer = (self.instruction_pointer as i64) + self.get_operand_value(offset);
                    if updated_instruction_pointer < 0 || updated_instruction_pointer >= (self.instructions.len()as i64) {
                        self.main_sender.send(MainMessage::Terminated { id: self.id }).unwrap();
                        return;
                    }
                    self.instruction_pointer = updated_instruction_pointer as usize;
                }
            }
        }
        if !has_jumped {
            self.instruction_pointer += 1;
        }
    }

    fn execute_loop(&mut self) {
        loop {
            self.execute();
        }
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

pub fn solve_part_one() {
    let instructions = read_instructions("day_eighteen.txt");
    let mut computer = Computer::new(&instructions);
    println!("{}", computer.get_last_recovered_sound());
}

pub fn solve_part_two() {
    let instructions = read_instructions("day_eighteen.txt");
    let (first_process_main_sender, main_receiver) = mpsc::channel::<MainMessage>();
    let second_process_main_sender = first_process_main_sender.clone();
    let (first_process_sender, first_process_receiver) = mpsc::channel::<i64>();
    let (second_process_sender, second_process_receiver) = mpsc::channel::<i64>();
    let mut first_process = Process::new(
        ProcessId::First, 
        &instructions, 
        first_process_main_sender, 
        second_process_sender,
        first_process_receiver
    );
    let mut second_process = Process::new(
        ProcessId::Second, 
        &instructions, 
        second_process_main_sender, 
        first_process_sender,
        second_process_receiver
    );
    let mut first_process_state = ProcessState::new(ProcessId::First);
    let mut second_process_state = ProcessState::new(ProcessId::Second);
    thread::spawn(move || first_process.execute_loop());
    thread::spawn(move || second_process.execute_loop());
    loop {
        let message = main_receiver.recv().unwrap();
        first_process_state.update(message);
        second_process_state.update(message);
        if first_process_state.is_halted() && second_process_state.is_halted() {
            break;
        }
    }
    println!("{}", first_process_state.num_sent_msg);
}