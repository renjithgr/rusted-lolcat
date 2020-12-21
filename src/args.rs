
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
