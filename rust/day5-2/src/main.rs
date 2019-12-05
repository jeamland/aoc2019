use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

struct Memory(Vec<i32>);

impl Memory {
    pub fn new(contents: Vec<i32>) -> Self {
        Memory(contents)
    }

    pub fn get_op(&self, pc: u32) -> (u32, Modes) {
        let mut op = self.0[pc as usize];
        let mut modes = Vec::new();

        let opcode = op % 100;
        op /= 100;

        while op != 0 {
            modes.push((op % 10) as u32);
            op /= 10;
        }

        (opcode as u32, Modes::new(modes))
    }

    pub fn get(&self, addr: u32, mode: u32) -> i32 {
        match mode {
            0 => self.0[self.0[addr as usize] as usize],
            1 => self.0[addr as usize],
            _ => panic!("wtf addr"),
        }
    }

    pub fn set(&mut self, addr: u32, mode: u32, value: i32) {
        let addr = match mode {
            0 => self.0[addr as usize] as u32,
            1 => addr,
            x => panic!("unknown addressing mode: {}", x),
        };
        self.0[addr as usize] = value;
    }

    pub fn op_3<F>(&mut self, pc: u32, mut modes: Modes, op: F)
    where
        F: Fn(i32, i32) -> i32,
    {
        modes.ensure(3);

        let arg1 = self.get(pc + 1, modes.0[0]);
        let arg2 = self.get(pc + 2, modes.0[1]);
        self.set(pc + 3, modes.0[2], op(arg1, arg2));
    }

    pub fn dump(&self) {
        let contents: Vec<String> = self.0.iter().map(|e| e.to_string()).collect();
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
    let mut memory = Memory::new(memory);

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut pc: u32 = 0;

    loop {
        let (opcode, mut modes) = memory.get_op(pc);

        match opcode {
            1 => {
                memory.op_3(pc, modes, |a, b| a + b);
                pc += 4;
            }
            2 => {
                memory.op_3(pc, modes, |a, b| a * b);
                pc += 4;
            }
            3 => {
                modes.ensure(1);
                print!("> ");
                stdout.flush()?;
                let mut value = String::new();
                stdin.read_line(&mut value)?;
                let value = i32::from_str_radix(value.trim(), 10).unwrap();
                memory.set(pc + 1, modes.0[0], value);
                pc += 2;
            }
            4 => {
                modes.ensure(1);
                println!("# {}", memory.get(pc + 1, modes.0[0]));
                pc += 2;
            }
            5 => {
                modes.ensure(2);
                let value = memory.get(pc + 1, modes.0[0]);
                if value != 0 {
                    pc = memory.get(pc + 2, modes.0[1]) as u32;
                } else {
                    pc += 3;
                }
            }
            6 => {
                modes.ensure(2);
                let value = memory.get(pc + 1, modes.0[0]);
                if value == 0 {
                    pc = memory.get(pc + 2, modes.0[1]) as u32;
                } else {
                    pc += 3;
                }
            }
            7 => {
                memory.op_3(pc, modes, |a, b| if a < b { 1 } else { 0 });
                pc += 4;
            }
            8 => {
                memory.op_3(pc, modes, |a, b| if a == b { 1 } else { 0 });
                pc += 4;
            }
            99 => break,
            x => panic!("unknown opcode: {}", x),
        };
    }

    memory.dump();

    Ok(())
}
