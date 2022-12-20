use rand::Rng;
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
            model_parameters: ModelParameters::default(),
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

    let mut process = Process::new(model_parameters, start_state, 0.001);
    while process.state() >= 0. && process.state() <= b {
        process.step(rng);
    }

    if process.state() >= b {
        1.
    } else {
        0.
    }
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        model_parameters: ModelParameters{
                det_mean,
                det_var,
                rep_mean,
                rep_var,
                self_reversion,
        },
        start_state: x,
        critical_value: b,
    } = parameters;
    assert_eq!(self_reversion, 0., "Self-reversion is not supported in this question.");
    let mu = det_mean-rep_mean;
    let sigma_squared = det_var+rep_var;
    (1.-(-2.*mu*x/sigma_squared).exp())/(1.-(-2.*mu*b/sigma_squared).exp())
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters::default();

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        1_000_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
