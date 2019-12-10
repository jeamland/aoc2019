use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Asteroid(i32, i32);

impl Asteroid {
    pub fn new(x: i32, y: i32) -> Self {
        Asteroid(x, y)
    }
}

fn main() {
    let stdin = io::stdin();
    let mut y: i32 = 0;
    let mut asteroids = Vec::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                asteroids.push(Asteroid::new(x as i32, y));
            }
        }

        y += 1;
    }

    let mut occlusions = HashSet::new();

    for pair in asteroids.iter().combinations(2) {
        let first = pair[0];
        let second = pair[1];
        let vector = (second.0 - first.0, second.1 - first.1);

        for other in asteroids.iter().filter(|a| *a != first && *a != second) {
            let other_vector = (other.0 - first.0, other.1 - first.1);

            // Exclude differences in sign.
            if vector.0 * other_vector.0 < 0 {
                continue;
            } else if vector.1 * other_vector.1 < 0 {
                continue;
            }

            if vector.0 == 0 {
                if other_vector.0 == 0 && other_vector.1 < vector.1 {
                    occlusions.insert((first, second));
                    occlusions.insert((second, first));
                }
                continue;
            } else if other_vector.0 == 0 {
                continue;
            }

            if vector.1 == 0 {
                if other_vector.1 == 0 && other_vector.0 < vector.0 {
                    occlusions.insert((first, second));
                    occlusions.insert((second, first));
                }
                continue;
            } else if other_vector.1 == 0 {
                continue;
            }

            let x_multiple = f64::from(vector.0) / f64::from(other_vector.0);
            let y_multiple = f64::from(vector.1) / f64::from(other_vector.1);

            if x_multiple < 1.0 || y_multiple < 1.0 {
                continue;
            } else if x_multiple != y_multiple {
                continue;
            }

            occlusions.insert((first, second));
            occlusions.insert((second, first));
        }
    }

    let mut site = &asteroids[0];
    let mut max_visible = 0;

    for asteroid in asteroids.iter() {
        let visible = asteroids
            .iter()
            .filter(|a| **a != *asteroid)
            .filter(|a| !occlusions.contains(&(*a, asteroid)))
            .count();

        if visible > max_visible {
            max_visible = visible;
            site = asteroid;
        }
    }

    println!("{:?} {}", site, max_visible);
}
