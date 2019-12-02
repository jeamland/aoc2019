use std::io;
use std::io::prelude::*;

fn fuel_for_mass(mass: u32) -> u32 {
    let third = mass / 3;
    if third <= 2 {
        0
    } else {
        third - 2
    }
}

fn total_fuel_for_mass(mass: u32) -> u32 {
    let mut fuel = fuel_for_mass(mass);
    let mut extra = fuel;

    while extra > 0 {
        extra = fuel_for_mass(extra);
        fuel += extra;
    }

    fuel
}

fn main() {
    let stdin = io::stdin();
    let mut total: u32 = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let val = u32::from_str_radix(&line, 10).unwrap();
        total += total_fuel_for_mass(val);
    }

    println!("Total: {:?}", total);
}
