#![allow(unused_imports)]
use std::{
    collections::VecDeque,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div},
};

use float_cmp::approx_eq;
use ndarray::Array1;
use num::{traits::Pow, Float};
use rand::{Rng, SeedableRng};
use rand_distr::{Bernoulli, Distribution, Exp, Poisson, Uniform};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use stoc::{test_theory, Vector};

const SEED: u128 = 1;
const MAX_THREADS: u32 = 8;

const TIME: f64 = 1000.;

struct Parameters;

fn sample_lifetime(rng: &mut impl Rng) -> f64 {
    let x = Uniform::new(0., 1.);
    let y = Uniform::new(0., 2.);
    let x = x.sample(rng);
    let y = y.sample(rng);
    if y < 2. * x {
        x
    } else {
        1. - x
    }
}

fn experiment(_parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let mut cur_time = 0.;
    let mut cur_epochs = 0;

    while cur_time <= TIME {
        cur_time += sample_lifetime(rng);
        cur_epochs += 1;
    }
    cur_epochs as f64 - 1.
}

fn theory(parameters: &Parameters) -> f64 {
    TIME / (2. / 3.) - 7. / 24.
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let result = test_theory(
        experiment,
        theory,
        &Parameters,
        1_000_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
