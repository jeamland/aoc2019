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
}

struct Wire {
    spans: Vec<(char, i32)>,
    x: i32,
    y: i32,
    pub points: HashSet<Point>,
}

impl Wire {
    pub fn new(spans: Vec<(char, i32)>) -> Self {
        let mut wire = Wire {
            spans: spans.clone(),
            x: 0,
            y: 0,
            points: HashSet::new(),
        };

        for span in spans {
            wire.add_span(span.0, span.1);
        }

        wire
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

    pub fn steps_to_intersection(&self, point: Point) -> i32 {
        let mut steps = 0;
        let mut x = 0;
        let mut y = 0;

        for span in self.spans.clone() {
            match span.0 {
                'R' => {
                    if point.y == y && x < point.x && point.x < (x + span.1) {
                        return steps + (point.x - x);
                    } else {
                        steps += span.1;
                        x += span.1;
                    }
                }
                'L' => {
                    if point.y == y && x > point.x && point.x > (x - span.1) {
                        return steps + (x - point.x);
                    } else {
                        steps += span.1;
                        x -= span.1;
                    }
                }
                'U' => {
                    if point.x == x && y < point.y && point.y < (y + span.1) {
                        return steps + (point.y - y);
                    } else {
                        steps += span.1;
                        y += span.1;
                    }
                }
                'D' => {
                    if point.x == x && y > point.y && point.y > (y - span.1) {
                        return steps + (y - point.y);
                    } else {
                        steps += span.1;
                        y -= span.1;
                    }
                }
                _ => (),
            }
        }

        steps
    }
}

fn main() {
    let stdin = io::stdin();
    let mut wires: Vec<Wire> = Vec::new();

    for line in stdin.lock().lines() {
        let mut spans = Vec::new();

        let line = line.unwrap();
        for bit in line.split(',') {
            let direction = bit.chars().nth(0).unwrap();
            let distance = i32::from_str_radix(&bit[1..], 10).unwrap();
            spans.push((direction, distance));
        }

        wires.push(Wire::new(spans));
    }

    let crossings: Vec<Point> = wires[0]
        .points
        .intersection(&wires[1].points)
        .map(|p| *p)
        .collect();

    let mut distances: Vec<i32> = crossings.iter().map(|_| 0).collect();

    for wire in wires {
        distances = crossings
            .iter()
            .map(|c| wire.steps_to_intersection(*c))
            .zip(distances.iter())
            .map(|(s, d)| s + d)
            .collect();
    }

    distances.sort();
    println!("{:?}", distances[0]);
}
