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
    lambda: f64,
    mu: f64,
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
        lambda,
        mu,
        n,
        sample_start,
        sample_end,
    } = parameters;
    if *cur_state == 0 {
        let birth_time = Exp::new(lambda).unwrap().sample(rng);
        *cur_time += birth_time;
        *cur_state += 1;
        birth_time
    } else {
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
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> Vector {
    let &Parameters {
        lambda,
        mu,
        n,
        sample_start,
        sample_end,
    } = parameters;
    let mut cur_time = 0.;
    let mut cur_state = 0usize;
    let mut prev_state = 0usize;
    while cur_time <= sample_start {
        prev_state = cur_state;
        advance_state(parameters, &mut cur_state, &mut cur_time, rng);
    }
    let mut time_in_states = Vector::from_elem(n, 0.);
    if prev_state < n {
        time_in_states[prev_state] += cur_time - sample_start;
    }

    while cur_time <= sample_end {
        prev_state = cur_state;
        let time = advance_state(parameters, &mut cur_state, &mut cur_time, rng);
        if prev_state < n {
            time_in_states[prev_state] += time;
        }
    }
    if prev_state < n {
        time_in_states[prev_state] -= cur_time - sample_end;
    }
    time_in_states / (sample_end - sample_start)
}

fn theory(parameters: &Parameters) -> Vector {
    let &Parameters {
        lambda,
        mu,
        n,
        sample_start,
        sample_end,
    } = parameters;
    let theta = lambda / mu;
    let pi_0 = 1. - theta;
    Vector::from_shape_fn(n, |k| pi_0 * theta.powi(k as i32))
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        lambda: 4.,
        mu: 7.,
        n: 5,
        sample_start: 1000.,
        sample_end: 2000.,
    };

    assert!(parameters.lambda < parameters.mu);
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
