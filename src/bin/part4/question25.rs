use rand::Rng;
use rand_pcg::Pcg64Mcg;
use stoc::test_theory;

use crate::{ModelParameters, Process, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    start_state: f64,
    critical_value: f64,
    time1: f64,
    time2: f64,
    return_time: f64,
    step_size: f64,
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
            start_state: 0.,
            critical_value: 4.,
            time1: 1.,
            time2: 3.,
            return_time: 4.,
            step_size: 0.001,
        }
    }
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        start_state,
        critical_value: b,
        time1,
        time2,
        return_time,
        step_size,
    } = parameters;

    let mut process = Process::from_params(model_parameters, start_state, step_size);
    while process.time() <= time1 {
        process.step(rng);
    }
    let value1 = process.state();
    while process.time() <= time2 {
        process.step(rng);
    }
    let value2 = process.state();
    while process.time() <= return_time {
        process.step(rng);
    }
    let return_state = process.state();
    if value1-time1*return_state/return_time > b && value2 - time2*return_state/return_time > b {
        1.
    } else {
        0.
    }
}

fn theory(_parameters: &Parameters) -> f64 {
    1.37e-4
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
