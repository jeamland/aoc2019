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
            (f64::from(amount as u32) / f64::from(*a as u32)).ceil() as u64
        } else {
            1
        };
        for _ in 0..multiplier {
            for quantity in rule {
                let new_amount = new_production.entry(quantity.name.clone()).or_insert(0);
                *new_amount += quantity.amount;
            }
        }
        if (*a * multiplier) > amount {
            let inv = inventory.entry(name.clone()).or_insert(0);
            *inv += (*a * multiplier) - amount;
        }
    }

    new_production
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

    let mut inventory = HashMap::new();
    let mut production: HashMap<String, u64> = HashMap::new();
    production.insert("FUEL".to_string(), 1);

    loop {
        if production.len() == 1 {
            if let Some(_) = production.get("ORE") {
                break;
            }
        }
        production = iterate(&rules, &production, &mut inventory);
    }

    println!("ORE: {}", production.get("ORE").unwrap());
}
