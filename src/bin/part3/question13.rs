use rand::Rng;
use rand_distr::{Distribution, Exp};
use rand_pcg::Pcg64Mcg;
use statrs::distribution::Erlang;
use stoc::{test_theory, MarkovQueueProbabilities, ContinuousMarkovProcess};

use crate::{ModelParameters, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
    start_state: u64,
    time: f64,
    use_erlang: bool,
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
        start_state,
        time,
        use_erlang,
    } = parameters;

    assert_eq!(service_startup_time, 0.);

    let markov_queue_probabilities = MarkovQueueProbabilities::new(failure_rate, 1./service_time, units);
    let sample_time = rng.gen_range(min_run_time..max_run_time);
    let mut process = ContinuousMarkovProcess::new(markov_queue_probabilities, start_state);
    let mut old_state = process.state();
    while process.time() < sample_time {
        old_state = process.state();
        process.step(rng);
    }
    let queue_length = old_state;

    let total_time = if use_erlang {
        let wait_time = if queue_length < units {
            0.
        } else {
            Erlang::new(queue_length-units+1, units as f64/service_time).unwrap().sample(rng)
        };
        let repair_time = Exp::new(1./service_time).unwrap().sample(rng);
        wait_time + repair_time
    } else {
        let mut queue_length = queue_length;
        let mut total_time = 0.;
        while queue_length >= 9 {
            total_time += Exp::new(units as f64/service_time).unwrap().sample(rng);
            queue_length -= 1;
        }
        total_time += Exp::new(1./service_time).unwrap().sample(rng);
        total_time
    };
    if total_time > time {
        1.
    } else {
        0.
    }
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
        min_run_time: 1000.,
        max_run_time: 2000.,
        start_state: 0,
        time: 2.,
        use_erlang: true,
    };

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        50_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
