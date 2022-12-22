use rand::Rng;
use rand_distr::{Distribution, Uniform};
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, MarkovQueueProbabilities};

use crate::{ModelParameters, Process, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters: ModelParameters {
            units,
            failure_rate,
            service_time,
            service_startup_time,
        },
        min_run_time,
        max_run_time,
    } = parameters;

    let markov_queue_probabilities = MarkovQueueProbabilities::new(failure_rate, 1./service_time, units);
    
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
            service_startup_time: 0.,
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
