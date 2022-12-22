use std::env;

use rand::Rng;
use rand_distr::{Poisson, Binomial, Distribution};
use stoc::Matrix;

mod question2;
mod question3;

const SEED: u128 = 4;
const MAX_THREADS: u32 = 8;

#[derive(Debug, Clone, Copy)]
pub struct ModelParameters {
    pub mu: f64,
    pub p: f64,
    pub a: u64,
}

impl ModelParameters {
    fn failure_pdf(&self, prev_failures: u64, next_failures: u64) -> f64 {
        let (k, l) = (prev_failures, next_failures);
        let &Self { mu, p, a: _ } = self;
        (0..=(u64::min(k, l)))
            .map(|h| {
                let binomial = (num_integer::binomial(k, h) as f64)
                * (1.-p).powi(h as i32)
                * p.powi((k - h) as i32);
                let poisson = (((1.-p) * mu).powi((l - h) as i32) * f64::exp(-(1.-p) * mu))
                / (stoc::factorial(l - h) as f64);
                binomial * poisson
            })
            .sum::<f64>()
    }

    fn transition_matrix(&self) -> Matrix {
        let &Self { mu: _, p: _, a } = self;
        let a = a as usize;
        Matrix::from_shape_fn((a + 1, a + 1), |(k, l)| {
            if k < a && l < a {
                self.failure_pdf(k as u64, l as u64)
            } else if k < a && l == a {
                1. - (0..a)
                    .map(|h| self.failure_pdf(k as u64, h as u64))
                    .sum::<f64>()
            } else if k == a && l < a {
                0.
            } else if k == a && l == a {
                1.
            } else {
                unreachable!()
            }
        })
    }
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            mu: 4.,
            p: 0.5,
            a: 10,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    parameters: ModelParameters,
    state: u64,
    cur_day: u64,
    failure_distribution: Poisson<f64>,
}

impl Process {
    fn new(parameters: ModelParameters, start_state: u64) -> Self {
        Self {
            parameters,
            state: start_state,
            cur_day: 0,
            failure_distribution: Poisson::new(parameters.mu*(1.-parameters.p)).unwrap(),
        }

    }

    fn state(&self) -> u64 {
        self.state
    }

    fn step(&mut self, rng: &mut impl Rng) {
        let &mut Self { parameters, state, cur_day: _, failure_distribution } = self;
        let &ModelParameters { mu: _, p, a: _ } = &parameters;
    
        let new_failures = failure_distribution.sample(rng) as u64;
        let kept_failures = Binomial::new(state, 1.-p).unwrap().sample(rng);

        self.state = new_failures + kept_failures;
        self.cur_day += 1;
    }


}

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u64>()
        .expect("Could not parse question number.");
    match question {
        2 => question2::main(),
        3 => question3::main(),
        _ => panic!("Unrecognized question number"),
    }
}
