use std::io::{self, Read};

struct Intcode {
    pub memory: Vec<u32>,
    pc: usize,
}

impl Intcode {
    pub fn new(memory: Vec<u32>) -> Self {
        Intcode {
            memory: memory.clone(),
            pc: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.memory[self.pc] {
                1 => {
                    let arg1 = self.memory[self.memory[self.pc + 1] as usize];
                    let arg2 = self.memory[self.memory[self.pc + 2] as usize];
                    let dest = self.memory[self.pc + 3] as usize;
                    self.memory[dest] = arg1 + arg2;
                    self.pc += 4;
                }
                2 => {
                    let arg1 = self.memory[self.memory[self.pc + 1] as usize];
                    let arg2 = self.memory[self.memory[self.pc + 2] as usize];
                    let dest = self.memory[self.pc + 3] as usize;
                    self.memory[dest] = arg1 * arg2;
                    self.pc += 4;
                }
                99 => break,
                _ => panic!("wtf"),
            };
        }
    }
}

fn main() {
    let mut data = String::new();

    io::stdin().read_to_string(&mut data).unwrap();

    let memory: Vec<u32> = data
        .trim()
        .split(',')
        .map(|e| u32::from_str_radix(e, 10).unwrap())
        .collect();

    let mut found = false;

    for noun in 0..99 {
        for verb in 0..99 {
            let mut test_memory = memory.clone();
            test_memory[1] = noun;
            test_memory[2] = verb;

            let mut cpu = Intcode::new(test_memory);
            cpu.run();

            println!("{}, {} -> {}", noun, verb, cpu.memory[0]);
            if cpu.memory[0] == 19690720 {
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }
}
