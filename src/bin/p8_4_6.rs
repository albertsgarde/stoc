#![allow(unused_imports)]
use std::{
    collections::VecDeque,
    f64::consts::PI,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div},
};

use float_cmp::approx_eq;
use ndarray::Array1;
use num::{traits::Pow, Float};
use rand::{Rng, SeedableRng};
use rand_distr::{Bernoulli, Distribution, Exp, Normal, Poisson, Uniform};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use stoc::{test_theory, GeometricBrownianMotion, Vector};

const SEED: u128 = 1;
const MAX_THREADS: u32 = 8;

struct Parameters {
    std_dev: f64,
    start_value: f64,
    step_size: f64,
    stop_t: f64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        std_dev,
        start_value,
        step_size,
        stop_t,
    } = parameters;
    let mut process = GeometricBrownianMotion::initialize(start_value, 0., std_dev * std_dev);

    while process.cur_t() < stop_t {
        if process.step(step_size, rng) > start_value * 2. {
            return 1.;
        }
    }
    0.
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        std_dev,
        start_value: _,
        step_size: _,
        stop_t: _,
    } = parameters;

    (-std_dev * 2f64.ln()).exp()
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        std_dev: 5.,
        start_value: 0.5,
        step_size: 0.001,
        stop_t: 30.,
    };

    assert!(parameters.std_dev > 0.);
    assert!(parameters.step_size > 0.);

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        100_000,
        MAX_THREADS,
        &mut rng,
    );
    let (&theory, &empirical) = result.parts();
    println!("ratio: {}", theory / empirical);
    println!("{result:?}");
}
