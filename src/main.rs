use std::io;
use std::io::prelude::*;
use std::f64::consts::PI;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for (i, c) in line.unwrap().char_indices() {
            let (r, g, b) = rgb(0.1, i as f64);
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
        }
        println!();
    }
}

fn rgb(freq: f64, i: f64) -> (u8, u8, u8) {
    let red = (freq * i + 0.0).sin() * 127.0 + 128.0;
    let green = (freq * i + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (freq * i + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    
    (red as u8, green as u8, blue as u8)
}


// red   = Math.sin(freq*i + 0) * 127 + 128
// green = Math.sin(freq*i + 2*Math::PI/3) * 127 + 128
// blue  = Math.sin(freq*i + 4*Math::PI/3) * 127 + 128