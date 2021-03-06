use args::process_params;
use args::Params;
use std::f64::consts::PI;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
mod args;

fn main() {
    let params = process_params(std::env::args().collect());

    if params.help {
        print_help_and_exit();
    } else if params.filenames.is_empty() {
        process_standard_input(params);
    } else {
        process_files(params);
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

fn process_files(params: Params) {
    for filename in params.filenames.iter() {
        if process_file(filename, params.frequency, params.spread).is_err() {
            println!("Error opening file {}", filename);
        }
    }
}

fn process_file(filename: &str, frequency: f64, spread: f64) -> Result<(), io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);

    for line in file.lines() {
        rainbow_println(&line.unwrap(), frequency, spread);
    }

    Ok(())
}

fn process_standard_input(params: Params) {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        rainbow_println(&line.unwrap(), params.frequency, params.spread);
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
