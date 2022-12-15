#![allow(unused_imports)]
use std::{
    collections::VecDeque,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div},
};

use stoc::{test_theory, Vector};

use float_cmp::approx_eq;
use ndarray::Array1;
use num::{traits::Pow, Float};
use rand::{Rng, SeedableRng};
use rand_distr::{Bernoulli, Distribution, Exp, Normal, Poisson, Uniform};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const SEED: u128 = 1;
const MAX_THREADS: u32 = 8;

struct Parameters {
    lambda: f64,
    t: f64,
    num_samples: usize,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        lambda,
        t,
        num_samples,
    } = parameters;
    let x_distr = Normal::new(0.0, 1.0).unwrap();
    let b_t_distr = x_distr.map(|x| t.sqrt() * x);
    let expr_distr = b_t_distr.map(|b_t| (b_t * lambda).exp());
    expr_distr.sample_iter(rng).take(num_samples).sum::<f64>() / num_samples as f64
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        lambda,
        t,
        num_samples: _,
    } = parameters;

    (0.5 * lambda * lambda * t).exp()
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        lambda: 0.1,
        t: 9.0,
        num_samples: 1_000,
    };

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        100_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
