use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn orbit_count(body: &String, bodies: &HashMap<String, String>) -> u32 {
    let mut count = 0;
    let mut body = body;

    loop {
        count += 1;
        body = bodies.get(body).unwrap();

        if body.as_str() == "COM" {
            break;
        }
    }

    count
}

fn main() {
    let mut bodies: HashMap<String, String> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let pair: Vec<&str> = line.split(')').collect();
        bodies.insert(pair[1].to_string(), pair[0].to_string());
    }

    let mut total = 0;

    for body in bodies.keys() {
        total += orbit_count(&body, &bodies);
    }

    println!("{}", total);
}
