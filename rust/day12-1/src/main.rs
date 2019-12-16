use std::io;
use std::io::prelude::*;

use regex::Regex;

#[derive(Clone)]
struct Body {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}

impl Body {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Body {
            position: (x, y, z),
            velocity: (0, 0, 0),
        }
    }

    pub fn adjust_velocity(&mut self, other: &mut Body) {
        if self.position.0 < other.position.0 {
            self.velocity.0 += 1;
            other.velocity.0 -= 1;
        } else if self.position.0 > other.position.0 {
            self.velocity.0 -= 1;
            other.velocity.0 += 1;
        }
        if self.position.1 < other.position.1 {
            self.velocity.1 += 1;
            other.velocity.1 -= 1;
        } else if self.position.1 > other.position.1 {
            self.velocity.1 -= 1;
            other.velocity.1 += 1;
        }
        if self.position.2 < other.position.2 {
            self.velocity.2 += 1;
            other.velocity.2 -= 1;
        } else if self.position.2 > other.position.2 {
            self.velocity.2 -= 1;
            other.velocity.2 += 1;
        }
    }

    pub fn adjust_position(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.position.2 += self.velocity.2;
    }

    pub fn energy(&self) -> i32 {
        let p = self.position.0.abs() + self.position.1.abs() + self.position.2.abs();
        let k = self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs();

        p * k
    }
}

impl std::fmt::Display for Body {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        formatter.write_fmt(format_args!(
            "pos=<x={}, y={}, z={}>, vel=<x={}, y={}, z={}>",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2
        ))?;
        Ok(())
    }
}

struct Bodies(Vec<Body>);

impl Bodies {
    pub fn new() -> Self {
        Bodies(Vec::new())
    }

    pub fn add(&mut self, body: Body) {
        self.0.push(body);
    }

    pub fn iterate(&mut self) {
        let mut new_bodies = Vec::new();

        while self.0.len() > 0 {
            let mut body = self.0.remove(0);
            for other in self.0.iter_mut() {
                body.adjust_velocity(other);
            }
            new_bodies.push(body);
        }
        for body in new_bodies.iter_mut() {
            body.adjust_position();
        }

        self.0 = new_bodies;
    }

    pub fn dump(&self) {
        for body in self.0.iter() {
            println!("{}", body);
        }
    }

    pub fn total_energy(&self) -> i32 {
        self.0.iter().map(|b| b.energy()).sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();

    let mut bodies = Bodies::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();

        let x = i32::from_str_radix(captures.get(1).unwrap().as_str(), 10).unwrap();
        let y = i32::from_str_radix(captures.get(2).unwrap().as_str(), 10).unwrap();
        let z = i32::from_str_radix(captures.get(3).unwrap().as_str(), 10).unwrap();

        bodies.add(Body::new(x, y, z));
    }

    println!("Iteration 0");
    bodies.dump();
    println!("Total energy: {}", bodies.total_energy());

    for i in 1..1001 {
        bodies.iterate();
        if i % 100 == 0 {
            println!("\nIteration {}", i);
            bodies.dump();
            println!("Total energy: {}", bodies.total_energy());
        }
    }
}
