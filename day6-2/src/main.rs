use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn orbit_list<S>(body: S, bodies: &HashMap<String, String>) -> Vec<String>
where
    S: ToString,
{
    let mut orbit_bodies: Vec<String> = vec![];
    let mut body: &String = &body.to_string();

    loop {
        body = bodies.get(body).unwrap();
        orbit_bodies.push(body.clone());

        if body.as_str() == "COM" {
            break;
        }
    }

    orbit_bodies
}

fn main() {
    let mut bodies: HashMap<String, String> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let pair: Vec<&str> = line.split(')').collect();
        bodies.insert(pair[1].to_string(), pair[0].to_string());
    }

    let you_orbits = orbit_list("YOU", &bodies);
    let san_orbits = orbit_list("SAN", &bodies);

    let mut junction: String = "???".to_string();

    for (you, san) in you_orbits.iter().rev().zip(san_orbits.iter().rev()) {
        if you == san {
            junction = you.clone();
        } else {
            break;
        }
    }

    let you_to_junction = you_orbits
        .iter()
        .take_while(|x| x.as_str() != junction)
        .count();
    let san_to_junction = san_orbits
        .iter()
        .take_while(|x| x.as_str() != junction)
        .count();

    println!("{}", you_to_junction + san_to_junction);
}
