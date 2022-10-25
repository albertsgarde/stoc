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
use rand_distr::{Bernoulli, Distribution, Exp, Poisson, Uniform};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

const SEED: u128 = 1;
const MAX_THREADS: u32 = 8;

struct Parameters {
    lambda: f64,
    nu: f64,
    servers: u32,
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
        nu,
        servers,
        sample_start,
        sample_end,
    } = parameters;

    let mu = nu * (servers.min(*cur_state as u32) as f64);
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

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        lambda,
        nu,
        servers,
        sample_start,
        sample_end,
    } = parameters;
    let mut cur_time = 0.;
    let mut cur_state = 0;
    let mut prev_state;

    let mut times = VecDeque::new();

    let mut total = 0.;
    let mut total_customers = 0;

    while cur_time < sample_start {
        prev_state = cur_state;
        advance_state(parameters, &mut cur_state, &mut cur_time, rng);
        if cur_state > prev_state {
            times.push_back(cur_time);
        } else {
            times.pop_front().unwrap();
        }
        assert_eq!(times.len(), cur_state);
    }

    while cur_time < sample_end {
        prev_state = cur_state;
        advance_state(parameters, &mut cur_state, &mut cur_time, rng);
        if cur_state > prev_state {
            times.push_back(cur_time);
        } else {
            total_customers += 1;
            let waiting_time = cur_time - times.pop_front().unwrap();
            total += waiting_time;
        }
        assert_eq!(times.len(), cur_state);
    }
    total / (total_customers as f64)
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        lambda,
        nu,
        servers,
        sample_start,
        sample_end,
    } = parameters;

    if servers == 1 {
        1. / (nu - lambda)
    } else if servers == 2 {
        let pi_0 = 1.
            / (1. + (lambda / nu) + lambda.powi(2) / (2. * nu.powi(2) * (1. - lambda / (2. * nu))));
        let l_0 = pi_0 / 2. * (lambda / nu).powi(2) * lambda
            / (2. * nu * (1. - lambda / (2. * nu)).powi(2));
        let w_0 = l_0 / lambda;
        let w = l_0 + 1. / nu;
        w
    } else {
        unreachable!()
    }
}

fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        lambda: 1.2,
        nu: 2.,
        servers: 2,
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
