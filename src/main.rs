use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::f64::consts::PI;

fn main() {

    let filename = match std::env::args().nth(1) {
        Some(n) =>  n,
        None => String::from("")
    };

    if filename == "" {
        process_standard_input();
    } else if process_file(&filename).is_err() {
        println!("Error opening file {}", filename);
    }
        
}

fn process_file(filename: &str)-> Result<(), io::Error>{
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        for (i, c) in line.unwrap().char_indices() {
            let (r, g, b) = rgb(0.1, i as f64);
            print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
        }
        println!();
    }

    Ok(())
}

fn process_standard_input() {
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