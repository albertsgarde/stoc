use rand::Rng;
use rand_distr::{Bernoulli, Distribution};
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, ContinuousMarkovProcess, Matrix, MatrixTransitions};

use crate::{ModelParameters, MAX_THREADS, SEED};

struct Parameters {
    model_parameters: ModelParameters,
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
        time,
    } = parameters;

    let start_state = if Bernoulli::new(p1).unwrap().sample(rng) { 0} else {1};

    let transition_matrix = Matrix::from_shape_vec((4, 4), vec![
        -lambda1, 0., lambda1, 0.,
        0., -lambda2, lambda2, 0.,
        0., 0., -mu, mu,
        mu*p1, mu*p2, 0., -mu,
    ]).unwrap();
    let mut process = ContinuousMarkovProcess::new(MatrixTransitions::new(transition_matrix), start_state);
    
    let mut total_repairs = 0;
    while process.time() < time {
        if process.state() == 0 || process.state() == 1 {
            total_repairs += 1;
        }
        process.step(rng);
    }
    
    (total_repairs - 1) as f64
}

fn theory(_parameters: &Parameters) -> f64 {
    5.752
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        model_parameters: ModelParameters::default(),
        time: 40.,
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
