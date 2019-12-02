use std::io::{self, Read};

fn main() {
    let mut data = String::new();

    io::stdin().read_to_string(&mut data).unwrap();

    let mut memory: Vec<u32> = data
        .trim()
        .split(',')
        .map(|e| u32::from_str_radix(e, 10).unwrap())
        .collect();

    let mut pc: usize = 0;

    loop {
        match memory[pc] {
            1 => {
                let arg1 = memory[memory[pc + 1] as usize];
                let arg2 = memory[memory[pc + 2] as usize];
                let dest = memory[pc + 3] as usize;
                memory[dest] = arg1 + arg2;
                pc += 4;
            }
            2 => {
                let arg1 = memory[memory[pc + 1] as usize];
                let arg2 = memory[memory[pc + 2] as usize];
                let dest = memory[pc + 3] as usize;
                memory[dest] = arg1 * arg2;
                pc += 4;
            }
            99 => break,
            _ => panic!("wtf"),
        };
    }

    let memory: Vec<String> = memory.iter().map(|e| e.to_string()).collect();
    println!("{}", memory.join(","));
}
