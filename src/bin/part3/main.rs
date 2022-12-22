use std::env;

mod question14;
mod question15;
mod question16;
mod question18;

const SEED: u128 = 4;
const MAX_THREADS: u32 = 8;

#[derive(Debug, Clone, Copy)]
pub struct ModelParameters {
    pub units: u64,
    pub failure_rate: f64,
    pub service_time: f64,
    pub service_startup_time: f64,
}


fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        14 => question14::main(),
        15 => question15::main(),
        16 => question16::main(),
        18 => question18::main(),
        _ => panic!("Unrecognized question number"),
    }
}
