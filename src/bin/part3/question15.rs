use rand::Rng;
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, MarkovQueueProbabilities, ContinuousMarkovProcess};

use crate::{ModelParameters, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    critical_value: u64,
    start_state: u64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters: ModelParameters {
            units,
            failure_rate,
            service_time,
            service_startup_time,
        },
        critical_value,
        start_state,
    } = parameters;
    assert_eq!(service_startup_time, 0.);

    let markov_queue_probabilities = MarkovQueueProbabilities::new(failure_rate, 1./service_time, units);
    let mut process = ContinuousMarkovProcess::new(markov_queue_probabilities, start_state);
    while process.state() < critical_value {
        process.step(rng);
    }
    process.time()
}

fn theory(_parameters: &Parameters) -> f64 {
    6.14
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
        critical_value: 10,
        start_state: 0,
    };

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
