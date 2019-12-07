use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use permutohedron::Heap;

struct Intcode {
    memory: Vec<i32>,
    pc: u32,
    input: Vec<i32>,
    pub output: Vec<i32>,
}

impl Intcode {
    pub fn new(contents: &Vec<i32>, input: Vec<i32>) -> Self {
        Intcode {
            memory: contents.clone(),
            pc: 0,
            input: input.clone(),
            output: Vec::new(),
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
        let value = self.input.remove(0);
        self.set(self.pc + 1, modes.0[0], value);
        self.pc += 2;
    }

    fn write_stdout(&mut self, mut modes: Modes) {
        modes.ensure(1);
        self.output.push(self.get(self.pc + 1, modes.0[0]));
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
    let mut input = 0;

    for phase in phases {
        let mut cpu = Intcode::new(memory, vec![*phase, input]);
        cpu.run();
        input = cpu.output[0];
    }

    input
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

    let mut phases = vec![0, 1, 2, 3, 4];
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
