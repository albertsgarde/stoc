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
use stoc::{test_theory, Vector};

const SEED: u128 = 1;
const MAX_THREADS: u32 = 8;

struct Parameters {
    a: f64,
    b: f64,
    step_size: f64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters { a, b, step_size } = parameters;
    let step_distr = Normal::new(0., step_size.sqrt()).unwrap();
    let mut x = 0.;
    let mut step_num = 0u32;
    let line_value = |step_num: u32| a + b * (step_num as f64) * step_size;

    while line_value(step_num) - x < 5. {
        x += step_distr.sample(rng);
        step_num += 1;
        if x > line_value(step_num) {
            return 1.;
        }
    }
    0.
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters { a, b, step_size: _ } = parameters;

    (-2. * a * b).exp()
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        a: 1.,
        b: 1.,
        step_size: 0.0001,
    };

    assert!(parameters.a > 0.);
    assert!(parameters.b > 0.);
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
