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
    let mut prev_digit: u8 = 0;
    let mut run_length: u8 = 1;
    let mut runs: [u8; 10] = [0; 10];

    for x in 0..digits.len() {
        if x < 5 && digits[x] > digits[x + 1] {
            return false;
        }

        if digits[x] == prev_digit {
            run_length += 1;
        } else if prev_digit != 0 {
            runs[prev_digit as usize] = run_length;
            run_length = 1;
        }

        prev_digit = digits[x];
    }
    runs[prev_digit as usize] = run_length;

    return runs.contains(&2);
}

fn main() {
    let valid: Vec<u32> = (START..END).filter(|v| valid(*v)).collect();
    println!("{}", valid.len());
}
