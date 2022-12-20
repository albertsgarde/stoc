use rand::Rng;
use rand_distr::{Distribution, Uniform, Exp, Poisson};
use rand_pcg::Pcg64Mcg;
use stoc::test_theory;

use crate::{ModelParameters, MAX_THREADS, SEED};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    min_run_time: f64,
    max_run_time: f64,
}

struct Process {
    state: u64,
    time: f64,
    service_startup_time: f64,
    service_exp_distr: Exp<f64>,
    failure_rate: f64,
}

impl Process {
    fn new(model_parameters: ModelParameters) -> Self {
        let ModelParameters {
            units,
            failure_rate,
            service_time,
            service_startup_time,
        } = model_parameters;
        assert_eq!(units, 1, "This model only supports 1 unit");
        Self {
            state: 0,
            time: 0.,
            service_startup_time: service_startup_time,
            service_exp_distr: Exp::new(1./(service_time-service_startup_time)).unwrap(),
            failure_rate,
        }
    }

    fn state(&self) -> u64 {
        self.state
    }

    fn time(&self) -> f64 {
        self.time
    }

    fn step(&mut self, rng: &mut impl Rng) {
        if self.state == 0 {
            self.state = 1;
            self.time += Exp::new(self.failure_rate).unwrap().sample(rng);
        } else {
            let delta_time = self.service_startup_time + self.service_exp_distr.sample(rng);
            let new_failures = Poisson::new(self.failure_rate * delta_time).unwrap().sample(rng) as u64;
            self.state -= 1;
            self.state += new_failures;
            self.time += delta_time;
        }
    }
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        min_run_time,
        max_run_time,
    } = parameters;

    let mut process = Process::new(model_parameters);
    let run_time = Uniform::new(min_run_time, max_run_time).sample(rng);
    let mut old_state = 0;
    while process.time() < run_time {
        old_state = process.state();
        process.step(rng);
    }
    let x = old_state as f64;
    let rho = process.failure_rate*model_parameters.service_time;
    let mean_x = rho+rho*rho/(1.-rho);
    (x-mean_x)*(x-mean_x)
}

fn theory(_parameters: &Parameters) -> f64 {
    f64::NAN
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        model_parameters: ModelParameters {
            units: 1,
            failure_rate: 4.,
            service_time: 2./9.,
            service_startup_time: 1./8.,
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
