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

struct Parameters {
    lambda: f64,
    nu: f64,
    capacity: u64,
    sample_start: f64,
    sample_end: f64,
}

fn advance_state(
    parameters: &Parameters,
    cur_state: &mut u64,
    cur_time: &mut f64,
    rng: &mut impl Rng,
) -> f64 {
    let &Parameters {
        lambda,
        nu,
        capacity,
        sample_start: _,
        sample_end: _,
    } = parameters;

    let mu = if *cur_state > 0 { nu } else { 0. };
    let lambda = if *cur_state < capacity { lambda } else { 0. };
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
        lambda: _,
        nu: _,
        capacity,
        sample_start,
        sample_end,
    } = parameters;
    let mut cur_time = 0.;
    let mut cur_state = 0;

    let mut idle_time = 0.;
    let mut cap_time = 0.;

    while cur_time < sample_start {
        advance_state(parameters, &mut cur_state, &mut cur_time, rng);
    }

    if cur_state == 0 {
        idle_time += cur_time - sample_start;
    }
    if cur_state == capacity {
        cap_time += cur_time - sample_start;
    }

    while cur_time < sample_end {
        let time = advance_state(parameters, &mut cur_state, &mut cur_time, rng);
        if cur_state == 0 {
            idle_time += time;
        }
        if cur_state > capacity {
            cap_time += time;
        }
    }
    if cur_state == 0 {
        idle_time -= cur_time - sample_end;
    }
    if cur_state == capacity {
        cap_time -= cur_time - sample_end;
    }
    Vector::from_shape_vec(2, vec![idle_time, cap_time]).unwrap() / (sample_end - sample_start)
}

fn theory(parameters: &Parameters) -> Vector {
    let &Parameters {
        lambda,
        nu,
        capacity,
        sample_start: _,
        sample_end: _,
    } = parameters;

    let rho = lambda / nu;
    let mut total = 0.;
    let mut power = 1.;
    for _ in 0..=capacity {
        total += power;
        power *= rho;
    }
    Vector::from_shape_vec(2, vec![1. / total, rho.powi(capacity as i32) / total]).unwrap()
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        lambda: 1.8,
        nu: 6.,
        capacity: 5,
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
