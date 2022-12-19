#![allow(unused_imports)]
use std::{
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

const SEED: u128 = 5;
const MAX_THREADS: u32 = 8;

struct Parameters {
    alpha: f64,
    beta: f64,
    n: usize,
    sample_start: f64,
    sample_end: f64,
}

fn advance_state(
    parameters: &Parameters,
    cur_state: &mut usize,
    cur_time: &mut f64,
    rng: &mut impl Rng,
) -> f64 {
    let &Parameters {
        alpha,
        beta,
        n,
        sample_start: _,
        sample_end: _,
    } = parameters;
    let lambda = alpha * ((n - *cur_state) as f64);
    let mu = beta * (*cur_state as f64);
    let sojourn_time = Exp::new(lambda + mu).unwrap().sample(rng);
    let death = Bernoulli::new(mu / (lambda + mu)).unwrap().sample(rng);
    *cur_time += sojourn_time;
    if death {
        *cur_state -= 1;
    } else {
        *cur_state += 1;
    }
    sojourn_time
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> Vector {
    let &Parameters {
        alpha: _,
        beta: _,
        n,
        sample_start,
        sample_end,
    } = parameters;
    let mut cur_time = 0.;
    let mut cur_state = 0;
    let mut prev_state = 0;
    while cur_time < sample_start {
        prev_state = cur_state;
        advance_state(parameters, &mut cur_state, &mut cur_time, rng);
    }

    let mut time_in_states = Vector::from_elem(n + 1, 0.);
    time_in_states[cur_state] += cur_time - sample_start;

    while cur_time < sample_end {
        prev_state = cur_state;
        let time = advance_state(parameters, &mut cur_state, &mut cur_time, rng);
        time_in_states[prev_state] += time;
    }
    time_in_states[prev_state] -= cur_time - sample_end;

    time_in_states / (sample_end - sample_start)
}

fn theory(parameters: &Parameters) -> Vector {
    let &Parameters {
        alpha,
        beta,
        n,
        sample_start: _,
        sample_end: _,
    } = parameters;
    let pi_0 = 1. / (1. + alpha / beta).powi(n as i32);

    let mut pi = Vector::from_elem(n + 1, 0.);
    pi[0] = pi_0;
    for k in 1..=n {
        pi[k] = pi[k - 1] * (alpha / beta) * (n - k + 1) as f64 / k as f64;
    }

    pi
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        alpha: 4.,
        beta: 7.,
        n: 3,
        sample_start: 100.,
        sample_end: 200.,
    };

    assert!(parameters.sample_start < parameters.sample_end);
    let result = test_theory(
        experiment,
        theory,
        &parameters,
        1_000_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
