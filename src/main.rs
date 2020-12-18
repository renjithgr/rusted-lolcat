use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::f64::consts::PI;


mod cli {
    pub struct Params {
        pub filenames: Vec<String>,
        pub frequency: f64,
        pub spread: f64
    }

    pub fn process_params(input: Vec<String>) -> Params {
        let mut frequency = 0.1;    
        let spread = 3.0;
        let mut filenames = vec![];

        let mut iter = input.iter();
        iter.next();
        while let Some(param) = iter.next() {
            if param == "-f" {
                match iter.next() {
                    Some(n) => frequency = n.parse::<f64>().unwrap(),
                    None => panic!("Invalid frequence value")
                }
            } else {
                filenames.push(param.to_string());
            }
        }
        return Params {
            filenames: filenames,
            frequency: frequency,
            spread: spread
        };
    }
}


fn main() {

    if std::env::args().len() == 1 {
        process_standard_input();
    } else {
        let args: Vec<String> = std::env::args().collect();
        let params = cli::process_params(args);
        
        for filename in params.filenames.iter() {
            if process_file(filename, params.frequency, params.spread).is_err() {
                println!("Error opening file {}", filename);
            }
        }
    }   
}

fn process_file(filename: &str, frequency: f64, spread: f64)-> Result<(), io::Error>{
    let f = File::open(filename)?;
    let file = BufReader::new(&f);
    for line in file.lines() {
        for (i, c) in line.unwrap().char_indices() {
            let (r, g, b) = rgb(frequency, i as f64 / spread );
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