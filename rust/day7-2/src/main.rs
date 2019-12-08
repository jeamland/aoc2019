use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;

use permutohedron::Heap;

struct Intcode {
    id: u32,
    memory: Vec<i32>,
    pc: u32,
    pub input: Receiver<i32>,
    pub output: Sender<i32>,
    pub last_output: Arc<AtomicI32>,
}

impl Intcode {
    pub fn new(id: u32, contents: &Vec<i32>) -> Self {
        let (output, input) = channel();
        Intcode {
            id,
            memory: contents.clone(),
            pc: 0,
            input,
            output,
            last_output: Arc::new(AtomicI32::new(0)),
        }
    }

    fn configure(&mut self, phase: i32, other: &mut Intcode, input: Option<i32>) {
        let (their_output, my_input) = channel();
        self.input = my_input;
        other.output = their_output.clone();

        their_output.send(phase).unwrap();

        if let Some(value) = input {
            their_output.send(value).unwrap();
        }
    }

    fn get_op(&self) -> (u32, Modes) {
        let mut op = self.memory[self.pc as usize];
        let mut modes = Vec::new();

        let opcode = op % 100;
        op /= 100;

        while op != 0 {
            modes.push((op % 10) as u32);
            op /= 10;
        }

        (opcode as u32, Modes::new(modes))
    }

    fn get(&self, addr: u32, mode: u32) -> i32 {
        match mode {
            0 => self.memory[self.memory[addr as usize] as usize],
            1 => self.memory[addr as usize],
            _ => panic!("wtf addr"),
        }
    }

    fn set(&mut self, addr: u32, mode: u32, value: i32) {
        let addr = match mode {
            0 => self.memory[addr as usize] as u32,
            1 => addr,
            x => panic!("unknown addressing mode: {}", x),
        };
        self.memory[addr as usize] = value;
    }

    fn op_3<F>(&mut self, mut modes: Modes, op: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        modes.ensure(3);

        let arg1 = self.get(self.pc + 1, modes.0[0]);
        let arg2 = self.get(self.pc + 2, modes.0[1]);
        self.set(self.pc + 3, modes.0[2], op(arg1, arg2));
        self.pc += 4;
    }

    fn conditional_jump<F>(&mut self, mut modes: Modes, op: F)
    where
        F: Fn(i32) -> bool,
    {
        modes.ensure(2);
        if op(self.get(self.pc + 1, modes.0[0])) {
            self.pc = self.get(self.pc + 2, modes.0[1]) as u32;
        } else {
            self.pc += 3;
        }
    }

    fn read_stdin(&mut self, mut modes: Modes) {
        modes.ensure(1);
        let value = self.input.recv().unwrap();
        self.set(self.pc + 1, modes.0[0], value);
        self.pc += 2;
    }

    fn write_stdout(&mut self, mut modes: Modes) {
        modes.ensure(1);
        let value = self.get(self.pc + 1, modes.0[0]);
        match self.output.send(value) {
            Ok(_) => (),
            Err(_) => {
                println!("{} - other end closed", self.id);
            }
        };
        self.last_output.store(value, Ordering::Release);
        self.pc += 2;
    }

    pub fn run(&mut self) {
        loop {
            let (opcode, modes) = self.get_op();
            match opcode {
                1 => self.op_3(modes, |a, b| a + b),
                2 => self.op_3(modes, |a, b| a * b),
                3 => self.read_stdin(modes),
                4 => self.write_stdout(modes),
                5 => self.conditional_jump(modes, |v| v != 0),
                6 => self.conditional_jump(modes, |v| v == 0),
                7 => self.op_3(modes, |a, b| if a < b { 1 } else { 0 }),
                8 => self.op_3(modes, |a, b| if a == b { 1 } else { 0 }),
                99 => break,
                x => panic!("unknown opcode: {}", x),
            };
        }
    }

    #[allow(dead_code)]
    pub fn dump(&self) {
        let contents: Vec<String> = self.memory.iter().map(|e| e.to_string()).collect();
        println!("{}", contents.join(","));
    }
}

#[derive(Debug)]
struct Modes(pub Vec<u32>);

impl Modes {
    pub fn new(values: Vec<u32>) -> Self {
        Modes(values)
    }

    pub fn ensure(&mut self, size: usize) {
        while self.0.len() < size {
            self.0.push(0);
        }
    }
}

fn run_setting(memory: &Vec<i32>, phases: &Vec<i32>) -> i32 {
    let mut unconfigured_cpus = Vec::new();

    for i in 0..phases.len() {
        let cpu = Intcode::new(i as u32, memory);
        unconfigured_cpus.push(cpu);
    }

    let mut cpus = Vec::new();

    for phase in phases.iter().rev() {
        let mut this_cpu = unconfigured_cpus.pop().unwrap();
        let (other_cpu, input) = match unconfigured_cpus.len() {
            0 => (cpus.last_mut().unwrap(), Some(0)),
            _ => (unconfigured_cpus.last_mut().unwrap(), None),
        };

        this_cpu.configure(*phase, other_cpu, input);
        cpus.insert(0, this_cpu);
    }

    let final_value = Arc::clone(&cpus[4].last_output);

    let mut handles = Vec::new();

    for mut cpu in cpus {
        let handle = thread::spawn(move || {
            cpu.run();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    final_value.load(Ordering::Acquire)
}

fn main() -> std::io::Result<()> {
    let prog = std::env::args().nth(1).unwrap();

    let file = File::open(prog)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data)?;

    let memory: Vec<i32> = data
        .trim()
        .split(',')
        .map(|e| i32::from_str_radix(e, 10).unwrap())
        .collect();

    let mut phases = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut phases);

    let mut max_output = 0;
    let mut max_phases: Vec<i32> = vec![0, 0, 0, 0, 0];

    for permutation in heap {
        let output = run_setting(&memory, &permutation);
        if output > max_output {
            max_output = output;
            max_phases = permutation.clone();
        }
    }

    println!("{} {:?}", max_output, max_phases);

    Ok(())
}
