use rand::Rng;
use rand_distr::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;
use stoc::test_theory;

use crate::{ModelParameters, Process, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        min_run_time,
        max_run_time,
    } = parameters;

    let mut process = Process::new(model_parameters);
    let run_time = Uniform::new(min_run_time, max_run_time).sample(rng);
    process.run_until(rng, run_time);
    let start_time = process.time();
    process.add_failure(rng);
    process.pause_arrivals();
    process.run_until_state(rng, 0);
    let end_time = process.time();
    end_time - start_time
}

fn theory(_parameters: &Parameters) -> f64 {
    6.
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        model_parameters: ModelParameters {
            units: 9,
            failure_rate: 4.,
            service_time: 2.,
        },
        min_run_time: 100.,
        max_run_time: 200.,
    };

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        1_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
