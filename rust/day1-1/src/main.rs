use std::io;
use std::io::prelude::*;

fn fuel_for_mass(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn main() {
    let stdin = io::stdin();
    let mut total: u32 = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let val = u32::from_str_radix(&line, 10).unwrap();
        total += fuel_for_mass(val);
    }

    println!("Total: {:?}", total);
}
