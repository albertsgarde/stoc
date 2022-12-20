use rand::Rng;
use rand_pcg::Pcg64Mcg;
use statrs::distribution::ContinuousCDF;
use stoc::test_theory;

use crate::{ModelParameters, Process, MAX_THREADS, SEED, OuProcess};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    start_state: f64,
    time: f64,
    critical_value: f64,
    step_size: f64,
    use_ou_process: bool,
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            model_parameters: ModelParameters {
                det_mean: 3.,
                det_var: 2.,
                rep_mean: 2.,
                rep_var: 1.,
                self_reversion: 0.1,
            },
            start_state: 4.,
            critical_value: 10.,
            time: 3.,
            step_size: 0.001,
            use_ou_process: false,
        }
    }
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        start_state,
        time,
        critical_value: b,
        step_size,
        use_ou_process,
    } = parameters;

    if use_ou_process {
        let process = OuProcess::from_params(model_parameters, start_state);
        if process.sample(time, rng) > b{
            1.
        } else {
            0.
        }
    } else {
        let mut process = Process::new(model_parameters, start_state, step_size);
        while process.time() < time {
            process.step(rng);
        }
        if process.state() > b {
            1.
        } else {
            0.
        }
    }
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        model_parameters: ModelParameters{
                det_mean: _,
                det_var,
                rep_mean: _,
                rep_var,
            self_reversion: beta
        },
        start_state: nu,
        time: t,
        critical_value: b,
        step_size: _,
        use_ou_process: _,
    } = parameters;
    let old_sigma_squared = det_var+rep_var;
    let sigma_squared = beta/(1.-(-beta).exp())*old_sigma_squared;
    let mean = nu*(-beta*t).exp();
    let variance = sigma_squared*(1.-(-2.*beta*t).exp())/(2.*beta);

    let distr = statrs::distribution::Normal::new(mean, variance.sqrt()).unwrap();
    1.-distr.cdf(b)
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        step_size: 0.001,
        use_ou_process: true,
        ..Parameters::default()
    };

    let result = test_theory(
        experiment,
        theory,
        &parameters,
        400_000,
        MAX_THREADS,
        &mut rng,
    );
    println!("{result:?}");
}
