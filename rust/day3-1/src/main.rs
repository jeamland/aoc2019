use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn manhattan(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    }
}

struct Wire {
    x: i32,
    y: i32,
    pub points: HashSet<Point>,
}

impl Wire {
    pub fn new() -> Self {
        Wire {
            x: 0,
            y: 0,
            points: HashSet::new(),
        }
    }

    pub fn add_span(&mut self, direction: char, distance: i32) {
        match direction {
            'R' => {
                for wx in 0..distance {
                    self.points.insert(Point::new(self.x + wx, self.y));
                }
                self.x += distance;
            }
            'L' => {
                for wx in 0..distance {
                    self.points.insert(Point::new(self.x - wx, self.y));
                }
                self.x -= distance;
            }
            'U' => {
                for wy in 0..distance {
                    self.points.insert(Point::new(self.x, self.y + wy));
                }
                self.y += distance;
            }
            'D' => {
                for wy in 0..distance {
                    self.points.insert(Point::new(self.x, self.y - wy));
                }
                self.y -= distance;
            }
            _ => (),
        };
    }
}

fn main() {
    let stdin = io::stdin();
    let mut wires: Vec<Wire> = Vec::new();

    for line in stdin.lock().lines() {
        let mut wire = Wire::new();

        let line = line.unwrap();
        for bit in line.split(',') {
            let direction = bit.chars().nth(0).unwrap();
            let distance = i32::from_str_radix(&bit[1..], 10).unwrap();

            wire.add_span(direction, distance);
        }
        wires.push(wire);
    }

    let crossings = wires[0].points.intersection(&wires[1].points);
    let mut distances: Vec<u32> = crossings
        .map(|p| p.manhattan())
        .filter(|d| *d != 0)
        .collect();
    distances.sort();
    println!("{:?}", distances[0]);
}
