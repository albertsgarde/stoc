use rand::Rng;
use rand_pcg::Pcg64Mcg;
use stoc::test_theory;

use crate::{ModelParameters, Process, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
    start_state: f64,
    critical_value: f64,
    interval: f64,
    step_size: f64,
}

impl Default for Parameters {
    fn default() -> Self {
        let beta = 0.1f64;
        let det_var = beta/(1.-(-beta).exp())*3.;
        Self {
            model_parameters: ModelParameters {
                det_mean: 0.,
                det_var: det_var,
                rep_mean: 0.,
                rep_var: 0.,
                self_reversion: beta,
            },
            min_run_time: 100.,
            max_run_time: 200.,
            start_state: 4.,
            critical_value: 10.,
            interval: 2.,
            step_size: 0.001,
        }
    }
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        min_run_time,
        max_run_time,
        start_state,
        critical_value: b,
        interval,
        step_size,
    } = parameters;

    let sample_time = rng.gen_range(min_run_time..max_run_time);
        let mut process = Process::from_params(model_parameters, start_state, step_size);
        while process.time() < sample_time {
            process.step(rng);
        }
        if process.state() < b {
            0.
        } else {
            while process.time() < sample_time + interval {
                process.step(rng);
            }
            if process.state() > b {
                1.
            } else {
                0.
            }
        }
}

fn theory(_parameters: &Parameters) -> f64 {
    0.09919869
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        min_run_time: 100.,
        max_run_time: 200.,
        step_size: 0.001,
        ..Parameters::default()
    };

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        4_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
