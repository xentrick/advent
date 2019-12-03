use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let src = env::args().nth(1).expect("Please provide a file to parse.");

    // Solve challenge one.
    one(src.clone());
    // Solve challenge two.
    two(src);
}

fn one(src: String) {
    let fpath = File::open(src).expect("Unable to open file...");
    let input = BufReader::new(fpath);

    let total_fuel: usize = input.lines()
        .map(|m| {
            match m.unwrap().parse::<usize>() {
                // We don't need to round anything
                // Rust chops off the decimal during u64 conversion
                Ok(n) => (n / 3) - 2,
                // Panic if input is bad.
                Err(e) => panic!("Bad mass value in input! {}", e),
            }
        })
        .sum();
    println!("Part One Total Consumption: {}", total_fuel);
}

fn two(src: String) {
    let fpath = File::open(src).expect("Unable to open file...");
    let input = BufReader::new(fpath).lines();

    let total_fuel: isize = input
        .map(|m| {
            match m.unwrap().parse::<isize>() {
                Ok(m) => {
                    // Parse recursively
                    calculate(m)
                },
                // Panic if input is bad.
                Err(e) => panic!("Bad mass value in input! {}", e),
            }
        })
        .sum();
    println!("Part Two Total Consumption: {}", total_fuel);
}

fn calculate(mass: isize) -> isize {
    // We don't need to round anything
    // Rust chops off the decimal during u64 conversion
    let fuel = (mass / 3) - 2;
    let mut total: isize = 0;

    if mass < 0 {
        return total
    } else {
        if fuel > 0 {
            total += fuel;
        }
        total += calculate(fuel);
    }
    total
}

