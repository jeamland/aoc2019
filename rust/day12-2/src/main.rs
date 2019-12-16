use std::io;
use std::io::prelude::*;

use num::Integer;
use regex::Regex;

#[derive(Clone)]
struct Body {
    pub position: (i64, i64, i64),
    pub velocity: (i64, i64, i64),
}

impl Body {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
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

#[derive(Clone)]
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

    #[allow(dead_code)]
    pub fn dump(&self) {
        for body in self.0.iter() {
            println!("{}", body);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();

    let mut bodies = Bodies::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let captures = re.captures(&line).unwrap();

        let x = i64::from_str_radix(captures.get(1).unwrap().as_str(), 10).unwrap();
        let y = i64::from_str_radix(captures.get(2).unwrap().as_str(), 10).unwrap();
        let z = i64::from_str_radix(captures.get(3).unwrap().as_str(), 10).unwrap();

        bodies.add(Body::new(x, y, z));
    }

    let start_bodies = bodies.clone();

    let mut counter = 0;
    let mut rx: i64 = 0;
    let mut ry: i64 = 0;
    let mut rz: i64 = 0;

    loop {
        bodies.iterate();
        counter += 1;

        let mut repeat_x = true;
        let mut repeat_y = true;
        let mut repeat_z = true;

        for (b, s) in bodies.0.iter().zip(start_bodies.0.iter()) {
            if b.velocity.0 != 0 || b.position.0 != s.position.0 {
                repeat_x = false;
            }
            if b.velocity.1 != 0 || b.position.1 != s.position.1 {
                repeat_y = false;
            }
            if b.velocity.2 != 0 || b.position.2 != s.position.2 {
                repeat_z = false;
            }
        }

        if repeat_x && rx == 0 {
            rx = counter;
            println!("x repeat at {}", counter);
        }
        if repeat_y && ry == 0 {
            ry = counter;
            println!("y repeat at {}", counter);
        }
        if repeat_z && rz == 0 {
            rz = counter;
            println!("z repeat at {}", counter);
        }

        if rx != 0 && ry != 0 && rz != 0 {
            println!("Cycle at {}", rx.lcm(&ry.lcm(&rz)));
            break;
        }
    }
}
