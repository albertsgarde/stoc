use rand::Rng;
use rand_distr::{Exp, Distribution};
use rand_pcg::Pcg64Mcg;
use stoc::test_theory;

use crate::{ModelParameters, Process, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    start_state: f64,
    critical_value: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            model_parameters: ModelParameters {
                det_mean: 0.,
                det_var: 3.,
                rep_mean: 0.,
                rep_var: 0.,
                self_reversion: 0.,
            },
            start_state: 4.,
            critical_value: 10.,
        }
    }
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        start_state,
        critical_value: b,
    } = parameters;

    let mut process = Process::from_params(model_parameters, start_state, 0.001);

    let inspection_time = Exp::new(1./6.).unwrap().sample(rng);

    while process.time() <= inspection_time {
        process.step(rng);
    }

    if process.state() >= b {
        1.
    } else {
        0.
    }
}

fn theory(_parameters: &Parameters) -> f64 {
    0.91
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters::default();

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
