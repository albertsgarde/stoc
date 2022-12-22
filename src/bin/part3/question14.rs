use rand::Rng;
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, MarkovQueueProbabilities, ContinuousMarkovProcess};

use crate::{ModelParameters, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
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
        min_run_time,
        max_run_time,
        start_state,
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
    old_state as f64
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        model_parameters: ModelParameters {
            units,
            failure_rate,
            service_time,
            service_startup_time,
        },
        min_run_time: _,
        max_run_time: _,
        start_state: _
    } = parameters;
    assert_eq!(service_startup_time, 0.);
    let rho = failure_rate * service_time;
    let units_fac = stoc::factorial(units) as f64;
    let pi_0 = (0..units).map(|j| rho.powi(j as i32) / stoc::factorial(j) as f64).sum::<f64>()+rho.powi(units as i32)/(units_fac *(1.-rho/(units as f64))).recip();
    
    rho + pi_0/units_fac * rho.powi(units as i32) / ((units as f64)*(1.-rho/(units as f64)))
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
        start_state: 14,
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
