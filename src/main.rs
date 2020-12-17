use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        for c in line.unwrap().chars() {
            let (r, g, b) = rgb(c as i16);
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
        }
        println!();
    }
}

fn rgb(c: i16) -> (u8, u8, u8) {
    let freq = 0.1;
    let i = c as f64;

    let r = (freq * i + 0.0).sin() * 127.0 + 128.0;
    let g = (freq * i + 2.0).sin() * 127.0 + 128.0;
    let b = (freq * i + 4.0).sin() * 127.0 + 128.0;
    
    (r as u8, g as u8, b as u8)
}
