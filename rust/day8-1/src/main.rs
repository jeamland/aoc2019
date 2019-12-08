use std::io;

fn main() {
    let width = std::env::args().nth(1).unwrap();
    let width = u32::from_str_radix(&width, 10).unwrap();
    let height = std::env::args().nth(2).unwrap();
    let height = u32::from_str_radix(&height, 10).unwrap();

    let stdin = io::stdin();

    let mut data = String::new();

    stdin.read_line(&mut data).unwrap();

    let mut pixels: Vec<u8> = Vec::new();

    for ch in data.trim().chars() {
        let pixel = u8::from_str_radix(&ch.to_string(), 10).unwrap();
        pixels.push(pixel);
    }

    let mut layers: Vec<Vec<u8>> = Vec::new();
    let layer_size = (width * height) as usize;

    for i in (0..pixels.len()).step_by(layer_size) {
        let layer = Vec::from(&pixels[i..i + layer_size]);
        layers.push(layer);
    }

    let mut zero_layer: Vec<u8> = Vec::new();
    let mut min_zeroes: usize = 150;

    for layer in layers {
        let zero_count = layer.iter().filter(|x| **x == 0).count();
        if zero_count < min_zeroes {
            zero_layer = layer;
            min_zeroes = zero_count;
        }
    }

    let ones = zero_layer.iter().filter(|x| **x == 1).count();
    let twos = zero_layer.iter().filter(|x| **x == 2).count();

    println!("{} * {} = {}", ones, twos, ones * twos);
}
