use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Quantity {
    amount: u64,
    name: String,
}

impl Quantity {
    pub fn new<S>(amount: u64, name: S) -> Self
    where
        S: ToString,
    {
        Quantity {
            amount,
            name: name.to_string(),
        }
    }
}

impl From<&str> for Quantity {
    fn from(value: &str) -> Self {
        let parts: Vec<&str> = value.split(' ').collect();
        Quantity::new(u64::from_str_radix(parts[0], 10).unwrap(), parts[1])
    }
}

fn iterate(
    rules: &HashMap<String, (u64, Vec<Quantity>)>,
    production: &HashMap<String, u64>,
    inventory: &mut HashMap<String, u64>,
) -> HashMap<String, u64> {
    let mut new_production: HashMap<String, u64> = HashMap::new();
    new_production.insert("ORE".to_string(), *production.get("ORE").unwrap_or(&0));

    for (name, amount) in production.iter() {
        let mut amount = *amount;
        if name == "ORE" {
            continue;
        }

        let (a, rule) = rules.get(name).unwrap();
        let q = inventory.entry(name.clone()).or_insert(0);
        if *q > 0 {
            if *q >= amount {
                *q -= amount;
                continue;
            }
            amount -= *q;
            *q = 0;
        }
        let multiplier = if *a < amount {
            let mut m = amount / *a;
            if amount % *a != 0 {
                m += 1;
            }
            m
        } else {
            1
        };
        for quantity in rule {
            let new_amount = new_production.entry(quantity.name.clone()).or_insert(0);
            *new_amount += multiplier * quantity.amount;
        }
        if (*a * multiplier) > amount {
            let inv = inventory.entry(name.clone()).or_insert(0);
            *inv += (*a * multiplier) - amount;
        }
    }

    new_production
}

fn ore_required(rules: &HashMap<String, (u64, Vec<Quantity>)>, fuel: u64) -> u64 {
    let mut inventory = HashMap::new();
    let mut production: HashMap<String, u64> = HashMap::new();
    production.insert("FUEL".to_string(), fuel);

    loop {
        if production.len() == 1 {
            if let Some(_) = production.get("ORE") {
                break;
            }
        }
        production = iterate(&rules, &production, &mut inventory);
    }

    *production.get("ORE").unwrap()
}

fn most_efficient(rules: &HashMap<String, (u64, Vec<Quantity>)>) -> (u64, u64) {
    let mut min_ratio = 0;
    let mut min_ore = 0;
    let mut max_fuel = 0;

    for fuel in 1..101 {
        let ore = ore_required(&rules, fuel);

        let ratio = ore / fuel;
        if min_ratio == 0 || ratio < min_ratio {
            min_ratio = ratio;
            min_ore = ore;
            max_fuel = fuel;
        }
    }

    (min_ore, max_fuel)
}

fn main() {
    let stdin = io::stdin();

    let mut rules: HashMap<String, (u64, Vec<Quantity>)> = HashMap::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(" => ").collect();
        let output = Quantity::from(parts[1]);
        let inputs: Vec<Quantity> = parts[0].split(", ").map(|s| Quantity::from(s)).collect();

        rules.insert(output.name, (output.amount, inputs));
    }

    let (ore_inc, fuel_inc) = most_efficient(&rules);

    let mut ore = 0;
    let mut fuel = fuel_inc * 1000000000000 / ore_inc;
    loop {
        let res = ore_required(&rules, fuel);
        if res >= 1000000000000 {
            fuel -= 1;
            break;
        }
        ore = res;
        fuel += 1;
    }

    println!("{} ORE -> {} FUEL", ore, fuel);
}
