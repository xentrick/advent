// use failure::{format_err, Error};
use failure::Error;
use std::collections::HashSet;
use std::env;
use std::fs;

fn main() -> Result<(), Error>{
    let argv: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&argv[1])?;

    let freq = calibrate(&input)?;
    println!("Frequency: {}", freq);
    let uniq = calibrate_once(&input)?;
    println!("Unique Frequency: {}", uniq);

    Ok(())
}

fn calibrate(input: &String) -> Result<i32, Error> {
    let mut freq = 0;

    for l in input.lines() {
        let variance: i32 = l.parse()?;
        freq += variance;
    }
    Ok(freq)
}

fn calibrate_once(input: &String) -> Result<i32, Error> {
    let mut uniq = HashSet::new();
    let mut freq = 0;

    uniq.insert(0);
    loop {
        for l in input.lines() {
            let variance: i32 = l.parse()?;
            freq += variance;

            if uniq.contains(&freq) {
                return Ok(freq)
            }
            uniq.insert(freq);
        }
    }
}
