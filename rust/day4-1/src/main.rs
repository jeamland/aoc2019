const START: u32 = 231832;
const END: u32 = 767346;

fn decompose(value: u32) -> Vec<u8> {
    let mut value = value;
    let mut digits = Vec::new();

    for _ in 0..6 {
        digits.insert(0, (value % 10) as u8);
        value = value / 10;
    }

    digits
}

fn valid(value: u32) -> bool {
    let digits = decompose(value);

    let mut dedup = digits.clone();
    dedup.dedup();

    if dedup.len() == digits.len() {
        return false;
    }

    for x in 1..digits.len() {
        if digits[x - 1] > digits[x] {
            return false;
        }
    }

    true
}

fn main() {
    let valid: Vec<u32> = (START..END).filter(|v| valid(*v)).collect();
    println!("{}", valid.len());
}
