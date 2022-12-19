use std::{cmp::Reverse, collections::BinaryHeap, env};

use ordered_float::OrderedFloat;
use rand::Rng;
use rand_distr::{Distribution, Exp, Poisson, Normal};

mod question20;

const SEED: u128 = 4;
const MAX_THREADS: u32 = 8;

type Hf64 = Reverse<OrderedFloat<f64>>;
fn to_hf(x: f64) -> Hf64 {
    Reverse(OrderedFloat(x))
}
fn from_hf(x: Hf64) -> f64 {
    x.0 .0
}

#[derive(Debug, Clone, Copy)]
pub struct ModelParameters {
    pub det_mean: f64,
    pub det_var: f64,
    pub rep_mean: f64,
    pub rep_var: f64,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            det_mean: 3.,
            det_var: 2.,
            rep_mean: 2.,
            rep_var: 1.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    parameters: ModelParameters,
    step_size: f64,
    state: f64,
    cur_time: f64,
    det_distr: Normal<f64>,
    rep_distr: Normal<f64>,
}

impl Process {
    fn new(parameters: ModelParameters, start_state: f64, step_size: f64) -> Self {
        Self {
            parameters,
            step_size,
            state: start_state,
            cur_time: 0.,
            det_distr: Normal::new(parameters.det_mean*step_size, (parameters.det_var*step_size).sqrt()).unwrap(),
            rep_distr: Normal::new(parameters.rep_mean*step_size, (parameters.rep_var*step_size).sqrt()).unwrap(),
        }
    }

    fn state(&self) -> f64 {
        self.state
    }

    fn time(&self) -> f64 {
        self.cur_time
    }

    fn step(&mut self, rng: &mut impl Rng) {
        let det_step = self.det_distr.sample(rng);
        let rep_step = self.rep_distr.sample(rng);
        self.state += det_step;
        self.state -= rep_step;
        self.cur_time += self.step_size;
    }
}

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        20 => question20::main(),
        _ => panic!("Unrecognized question number"),
    }
}
