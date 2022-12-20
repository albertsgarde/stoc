use std::{cmp::Reverse, collections::BinaryHeap, env};

use ordered_float::OrderedFloat;
use rand::{Rng};
use rand_distr::{Distribution, Exp, Poisson};

mod question13;
mod question18;

const SEED: u128 = 4;
const MAX_THREADS: u32 = 8;

type Hf64 = Reverse<OrderedFloat<f64>>;
fn to_hf(x: f64) -> Hf64 {
    Reverse(OrderedFloat(x))
}

#[derive(Debug, Clone, Copy)]
pub struct ModelParameters {
    pub units: u32,
    pub failure_rate: f64,
    pub service_time: f64,
    pub service_startup_time: f64,
}

#[derive(Debug, Clone)]
pub struct Process {
    parameters: ModelParameters,
    state: u32,
    cur_time: f64,
    units: BinaryHeap<Hf64>,
    no_arrivals: bool,
}

impl Process {
    fn new(parameters: ModelParameters) -> Self {
        Self {
            parameters,
            state: 0,
            cur_time: 0.,
            units: BinaryHeap::default(),
            no_arrivals: false,
        }
    }

    fn time(&self) -> f64 {
        self.cur_time
    }

    fn pause_arrivals(&mut self) {
        self.no_arrivals = true
    }

    fn start_available_units(&mut self, rng: &mut impl Rng) {
        while self.units.len() < self.parameters.units.min(self.state) as usize {
            let service_time = Exp::new(1. / self.parameters.service_time)
                .unwrap()
                .sample(rng);
            self.units.push(to_hf(self.cur_time + service_time));
        }
        assert!(
            self.units.len() == self.state.min(self.parameters.units) as usize,
            "Inconsistent state. Length of units queue is {}, but state is {}.",
            self.units.len(),
            self.state
        );
    }

    fn add_failure(&mut self, rng: &mut impl Rng) {
        self.state += 1;
        self.start_available_units(rng)
    }

    fn step(&mut self, rng: &mut impl Rng, stop_time: f64) {
        assert!(stop_time >= 0., "Stop time must be non-negative.");
        assert!(
            self.units.len() == self.state.min(self.parameters.units) as usize,
            "Inconsistent state. Length of units queue is {}, but state is {}.",
            self.units.len(),
            self.state
        );

        let prev_time = self.cur_time;
        self.cur_time = if let Some(&Reverse(OrderedFloat(next_service))) = self.units.peek() {
            if next_service <= stop_time {
                self.state -= 1;
                self.units.pop().unwrap().0 .0
            } else {
                stop_time
            }
        } else {
            stop_time
        };
        let elapsed_time = self.cur_time - prev_time;
        let arrivals = Poisson::new(self.parameters.failure_rate * elapsed_time)
            .unwrap()
            .sample(rng) as u32;
        if !self.no_arrivals {
            self.state += arrivals;
        }
        self.start_available_units(rng)
    }

    fn run_until(&mut self, rng: &mut impl Rng, stop_time: f64) {
        while self.cur_time < stop_time {
            self.step(rng, stop_time);
        }
    }

    fn run_until_state(&mut self, rng: &mut impl Rng, stop_state: u32) {
        while self.state != stop_state {
            self.step(rng, f64::INFINITY);
        }
    }
}

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        13 => question13::main(),
        18 => question18::main(),
        _ => panic!("Unrecognized question number"),
    }
}
