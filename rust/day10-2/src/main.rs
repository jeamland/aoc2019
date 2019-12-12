use std::collections::HashSet;
use std::f64::consts::PI;
use std::io;
use std::io::prelude::*;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Asteroid(i32, i32);

impl Asteroid {
    pub fn new(x: i32, y: i32) -> Self {
        Asteroid(x, y)
    }

    pub fn bearing(&self, other: &Asteroid) -> (f64, f64) {
        let vector = (f64::from(other.0 - self.0), f64::from(other.1 - self.1));
        let angle = f64::atan(vector.0 / vector.1);
        let distance = f64::sqrt(vector.0.powi(2) + vector.1.powi(2));

        match (
            vector.0,
            vector.0.is_sign_positive(),
            vector.1.is_sign_positive(),
        ) {
            (x, _, false) if x == 0.0 => (0.0, distance),
            (_, true, false) => (-angle, distance),
            (_, false, false) => (2.0 * PI - angle, distance),
            (_, _, true) => (PI - angle, distance),
        }
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
        let (angle, distance) = first.bearing(second);

        for other in asteroids.iter().filter(|a| *a != first && *a != second) {
            let (other_angle, other_distance) = first.bearing(other);

            if (angle * other_angle).is_sign_positive()
                && angle == other_angle
                && distance > other_distance
            {
                occlusions.insert((first, second));
                occlusions.insert((second, first));
            }
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

    let mut counter = 0;
    let mut vapourised = HashSet::new();

    loop {
        let mut occlusions = HashSet::new();

        for pair in asteroids
            .iter()
            .filter(|a| !vapourised.contains(*a))
            .combinations(2)
        {
            let first = pair[0];
            let second = pair[1];
            let (angle, distance) = first.bearing(second);
            for other in asteroids
                .iter()
                .filter(|a| !vapourised.contains(*a) && *a != first && *a != second)
            {
                let (other_angle, other_distance) = first.bearing(other);
                if (angle * other_angle).is_sign_positive()
                    && angle == other_angle
                    && distance > other_distance
                {
                    occlusions.insert((first, second));
                    occlusions.insert((second, first));
                }
            }
        }
        let mut victims: Vec<&Asteroid> = asteroids
            .iter()
            .filter(|a| *a != site)
            .filter(|a| !occlusions.contains(&(*a, site)))
            .filter(|a| !vapourised.contains(a))
            .collect();

        victims.sort_by(|a, b| {
            let aa = site.bearing(a).0;
            let bb = site.bearing(b).0;
            aa.partial_cmp(&bb).unwrap_or(std::cmp::Ordering::Equal)
        });

        for victim in victims {
            counter += 1;
            println!(
                "{} {:?} {}",
                counter,
                victim,
                site.bearing(victim).0.to_degrees()
            );
            vapourised.insert(victim);
        }

        if asteroids.len() == vapourised.len() + 1 {
            break;
        }
    }
}
