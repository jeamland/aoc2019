use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use pancurses::{endwin, initscr, noecho, Window};

type Addr = u32;
type Cell = i32;

struct Intcode {
    memory: Vec<Cell>,
    pc: Addr,
    relative_base: Cell,
    game: Game,
}

impl Intcode {
    pub fn new(contents: &Vec<Cell>) -> Self {
        Intcode {
            memory: contents.clone(),
            pc: 0,
            relative_base: 0,
            game: Game::new(),
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

    fn addr(&mut self, addr: Addr, mode: u32) -> usize {
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

    fn get(&mut self, addr: Addr, mode: u32) -> Cell {
        let addr = self.addr(addr, mode);
        self.memory[addr]
    }

    fn set(&mut self, addr: Addr, mode: u32, value: Cell) {
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
            self.pc = self.get(self.pc + 2, modes.0[1]) as Addr;
        } else {
            self.pc += 3;
        }
    }

    fn read_stdin(&mut self, mut modes: Modes) {
        modes.ensure(1);
        std::thread::sleep(std::time::Duration::from_millis(1));
        let value = if self.game.ball < self.game.paddle {
            -1
        } else if self.game.ball > self.game.paddle {
            1
        } else {
            0
        };
        self.set(self.pc + 1, modes.0[0], value);
        self.pc += 2;
    }

    fn write_stdout(&mut self, mut modes: Modes) {
        modes.ensure(1);
        let value = self.get(self.pc + 1, modes.0[0]);
        self.game.input(value);
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

struct Game {
    screen: HashMap<(Cell, Cell), Cell>,
    x_buffer: Option<Cell>,
    y_buffer: Option<Cell>,
    window: Window,
    ball: Cell,
    paddle: Cell,
    score: Cell,
}

impl Game {
    pub fn new() -> Self {
        let window = initscr();
        noecho();
        window.clear();

        Game {
            screen: HashMap::new(),
            x_buffer: None,
            y_buffer: None,
            window,
            ball: 0,
            paddle: 0,
            score: 0,
        }
    }

    pub fn input(&mut self, data: Cell) {
        if let None = self.x_buffer {
            self.x_buffer = Some(data);
        } else if let None = self.y_buffer {
            self.y_buffer = Some(data);
        } else {
            let x = self.x_buffer.unwrap();
            let y = self.y_buffer.unwrap();

            if x == -1 && y == 0 {
                self.window.mvaddstr(0, 60, format!("{}", data));
                self.score = data;
            } else {
                self.screen.insert((x, y), data);

                let ch = match data {
                    1 => '+',
                    2 => '#',
                    3 => {
                        self.paddle = x;
                        '-'
                    }
                    4 => {
                        self.ball = x;
                        '*'
                    }
                    _ => ' ',
                };

                self.window.mvaddch(y, x, ch);
            }

            self.window.refresh();

            self.x_buffer = None;
            self.y_buffer = None;
        }
    }
}

fn main() -> std::io::Result<()> {
    let prog = std::env::args().nth(1).unwrap();

    let file = File::open(prog)?;
    let mut buf_reader = BufReader::new(file);
    let mut data = String::new();
    buf_reader.read_to_string(&mut data)?;

    let mut memory: Vec<Cell> = data
        .trim()
        .split(',')
        .map(|e| Cell::from_str_radix(e, 10).unwrap())
        .collect();

    memory[0] = 2;

    let mut cpu = Intcode::new(&memory);
    cpu.run();
    cpu.game.window.getch();
    endwin();

    println!("{}", cpu.game.score);

    Ok(())
}
