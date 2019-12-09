use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

type Cell = i64;

struct Intcode {
    memory: Vec<Cell>,
    pc: u32,
    relative_base: Cell,
}

impl Intcode {
    pub fn new(contents: &Vec<Cell>) -> Self {
        Intcode {
            memory: contents.clone(),
            pc: 0,
            relative_base: 0,
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

    fn addr(&mut self, addr: u32, mode: u32) -> usize {
        let addr = match mode {
            0 => self.memory[addr as usize] as usize,
            1 => addr as usize,
            2 => (self.memory[addr as usize] + self.relative_base) as usize,
            x => panic!("unknown addressing mode: {}", x),
        };

        while self.memory.len() <= addr {
            self.memory.push(0);
        }

        addr
    }

    fn get(&mut self, addr: u32, mode: u32) -> Cell {
        let addr = self.addr(addr, mode);
        self.memory[addr]
    }

    fn set(&mut self, addr: u32, mode: u32, value: Cell) {
        let addr = self.addr(addr, mode);
        self.memory[addr] = value;
    }

    fn op_3<F>(&mut self, mut modes: Modes, op: F)
    where
        F: Fn(Cell, Cell) -> Cell,
    {
        modes.ensure(3);

        let arg1 = self.get(self.pc + 1, modes.0[0]);
        let arg2 = self.get(self.pc + 2, modes.0[1]);
        self.set(self.pc + 3, modes.0[2], op(arg1, arg2));
        self.pc += 4;
    }

    fn conditional_jump<F>(&mut self, mut modes: Modes, op: F)
    where
        F: Fn(Cell) -> bool,
    {
        modes.ensure(2);
        if op(self.get(self.pc + 1, modes.0[0])) {
            self.pc = self.get(self.pc + 2, modes.0[1]) as u32;
        } else {
            self.pc += 3;
        }
    }

    fn read_stdin(&mut self, mut modes: Modes) {
        let stdin = io::stdin();
        let mut stdout = io::stdout();

        modes.ensure(1);
        print!("> ");
        stdout.flush().unwrap();
        let mut value = String::new();
        stdin.read_line(&mut value).unwrap();
        let value = Cell::from_str_radix(value.trim(), 10).unwrap();
        self.set(self.pc + 1, modes.0[0], value);
        self.pc += 2;
    }

    fn write_stdout(&mut self, mut modes: Modes) {
        modes.ensure(1);
        println!("# {}", self.get(self.pc + 1, modes.0[0]));
        self.pc += 2;
    }

    fn set_relative_base(&mut self, mut modes: Modes) {
        modes.ensure(1);
        self.relative_base += self.get(self.pc + 1, modes.0[0]);
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
                9 => self.set_relative_base(modes),
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

fn main() -> std::io::Result<()> {
    let prog = std::env::args().nth(1).unwrap();

    let file = File::open(prog)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data)?;

    let memory: Vec<Cell> = data
        .trim()
        .split(',')
        .map(|e| Cell::from_str_radix(e, 10).unwrap())
        .collect();

    let mut cpu = Intcode::new(&memory);
    cpu.run();

    Ok(())
}
