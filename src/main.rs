use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::f64::consts::PI;
mod args;

fn main() {

    if std::env::args().len() == 1 {
        process_standard_input(0.1, 3.0);
    } else {
        let args: Vec<String> = std::env::args().collect();
        let params = args::process_params(args);

        if params.help == true {
            print_help_and_exit();
        }
        
        for filename in params.filenames.iter() {
            if process_file(filename, params.frequency, params.spread).is_err() {
                println!("Error opening file {}", filename);
            }
        }
    }   
}

fn print_help_and_exit() {
    let help = "usage: rusted-lolcat [option ...] [file ...]
    --spread, -s: Rainbow spread (default: 3.0) 
      --freq, -f: Rainbow frequency (default: 0.1)
      --help, -h: Show this message";
    println!("{}", help);
    std::process::exit(0);
}

fn process_file(filename: &str, frequency: f64, spread: f64)-> Result<(), io::Error>{
    let f = File::open(filename)?;
    let file = BufReader::new(&f);

    for line in file.lines() {
        rainbow_println(&line.unwrap(), frequency, spread);
    }

    Ok(())
}

fn process_standard_input(frequency: f64, spread: f64) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        rainbow_println(&line.unwrap(), frequency, spread);
    }
}

fn rainbow_println(line: &str, frequency: f64, spread: f64) {
    for (i, c) in line.char_indices() {
        let (r, g, b) = rgb(frequency, spread, i as f64);
        print!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, c);
    }
    println!();
}

fn rgb(freq: f64, spread: f64, i: f64) -> (u8, u8, u8) {
    let j = i / spread;
    let red = (freq * j + 0.0).sin() * 127.0 + 128.0;
    let green = (freq * j + 2.0 * PI / 3.0).sin() * 127.0 + 128.0;
    let blue = (freq * j + 4.0 * PI / 3.0).sin() * 127.0 + 128.0;
    
    (red as u8, green as u8, blue as u8)
}