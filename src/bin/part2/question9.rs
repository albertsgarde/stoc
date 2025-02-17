use rand::Rng;
use rand_distr::{Bernoulli, Distribution};
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, ContinuousMarkovProcess, Matrix, MatrixTransitions};

use crate::{ModelParameters, MAX_THREADS, SEED};

struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
    time: f64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters:
            ModelParameters {
                lambda1,
                lambda2,
                p1,
                p2,
                mu,
            },
        min_run_time,
        max_run_time,
        time,
    } = parameters;

    let sample_time = rng.gen_range(min_run_time..max_run_time);

    let start_state = if Bernoulli::new(p1).unwrap().sample(rng) { 0} else {1};

    let transition_matrix = Matrix::from_shape_vec((4, 4), vec![
        -lambda1, 0., lambda1, 0.,
        0., -lambda2, lambda2, 0.,
        0., 0., -mu, mu,
        mu*p1, mu*p2, 0., -mu,
    ]).unwrap();
    let mut process = ContinuousMarkovProcess::new(MatrixTransitions::new(transition_matrix), start_state);
    
    while process.time() < sample_time {
        process.step(rng);
    }
    
    while process.state() != 0 && process.state() != 1 {
        process.step(rng);
    }
    
    if process.time() - sample_time > time {
        1. 
    } else {
        0.
    }
}

fn theory(_parameters: &Parameters) -> f64 {
    0.381
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        model_parameters: ModelParameters::default(),
        min_run_time: 1000.,
        max_run_time: 2000.,
        time: 8.,
    };

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        10_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
