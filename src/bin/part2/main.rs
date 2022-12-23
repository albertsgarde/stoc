use std::env;

mod question9;
mod question10;
mod question11;

const SEED: u128 = 0;
const MAX_THREADS: u32 = 8;

pub struct ModelParameters {
    pub lambda1: f64,
    pub lambda2: f64,
    pub p1: f64,
    pub p2: f64,
    pub mu: f64,
}

impl Default for ModelParameters {
    fn default() -> Self {
        ModelParameters { lambda1: 1./33., lambda2: 10./33., p1: 1./11., p2: 10./11., mu: 1. }
    }
}

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        9 => question9::main(),
        10 => question10::main(),
        11 => question11::main(),
        _ => panic!("Unrecognized question number"),
    }
}
