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
    t: f64,
    num_samples: usize,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters { t, num_samples } = parameters;
    let x_distr = Normal::new(0.0, 1.0).unwrap();
    let b_t_distr = x_distr.map(|x| t.sqrt() * x);
    let expr_distr = b_t_distr.map(|b_t| b_t.abs());
    expr_distr.sample_iter(rng).take(num_samples).sum::<f64>() / num_samples as f64
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters { t, num_samples: _ } = parameters;

    2. * t.sqrt() / (2. * PI).sqrt()
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        t: 2.0,
        num_samples: 10_000,
    };

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
