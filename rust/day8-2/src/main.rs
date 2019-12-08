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

    let mut image: Vec<u8> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = layers
                .iter()
                .map(|l| l[(width * y + x) as usize])
                .skip_while(|p| *p == 2)
                .next()
                .unwrap();
            image.push(pixel);
        }
    }

    for (i, pixel) in image.iter().enumerate() {
        match pixel {
            0 => print!("#"),
            1 => print!(" "),
            _ => (),
        };

        if (i as u32) % width == width - 1 {
            println!("");
        }
    }
}
