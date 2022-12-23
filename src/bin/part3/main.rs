use std::env;

mod question13;
mod question14;
mod question15;
mod question16;
mod question18;

const SEED: u128 = 0;
const MAX_THREADS: u32 = 8;

#[derive(Debug, Clone, Copy)]
pub struct ModelParameters {
    pub units: u64,
    pub failure_rate: f64,
    pub service_time: f64,
    pub service_startup_time: f64,
}

/*
pub struct SingleUnitSystem {
    start_up_time: f64,
    service_rate: f64,
    failure_rate: f64,
    cur_startup_time: f64,
    cur_time: f64,
    cur_state: u64,
}

impl SingleUnitSystem {
    pub fn new(
        start_up_time: f64,
        service_rate: f64,
        failure_rate: f64,
        start_state: u64,
    ) -> Self {
        assert!(start_up_time >= 0.);
        assert!(service_rate >= 0.);
        assert!(failure_rate >= 0.);
        Self {
            start_up_time,
            service_rate,
            failure_rate,
            cur_startup_time: start_up_time,
            cur_time: 0.,
            cur_state: start_state,
        }
    }

    pub fn time(&self) -> f64 {
        self.cur_time
    }

    pub fn state(&self) -> u64 {
        self.cur_state
    }
}*/

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        13 => question13::main(),
        14 => question14::main(),
        15 => question15::main(),
        16 => question16::main(),
        18 => question18::main(),
        _ => panic!("Unrecognized question number"),
    }
}
