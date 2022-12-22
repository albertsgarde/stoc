use std::{env};
use rand::Rng;
use rand_distr::{Distribution, Normal};

mod question20;
mod question21;
mod question23;
mod question26;
mod question27;

const SEED: u128 = 0;
const MAX_THREADS: u32 = 8;

#[derive(Debug, Clone, Copy)]
pub struct ModelParameters {
    pub det_mean: f64,
    pub det_var: f64,
    pub rep_mean: f64,
    pub rep_var: f64,
    pub self_reversion: f64,
}

impl Default for ModelParameters {
    fn default() -> Self {
        Self {
            det_mean: 3.,
            det_var: 2.,
            rep_mean: 2.,
            rep_var: 1.,
            self_reversion: 0.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Process {
    step_size: f64,
    state: f64,
    cur_time: f64,
    det_distr: Normal<f64>,
    rep_distr: Normal<f64>,
    discount_factor: f64,
}

impl Process {

    fn from_params(parameters: ModelParameters, start_state: f64, step_size: f64) -> Self {
        Self {
            step_size,
            state: start_state,
            cur_time: 0.,
            det_distr: Normal::new(parameters.det_mean*step_size, (parameters.det_var*step_size).sqrt()).unwrap(),
            rep_distr: Normal::new(parameters.rep_mean*step_size, (parameters.rep_var*step_size).sqrt()).unwrap(),
            discount_factor: (-parameters.self_reversion*step_size).exp(),
        }
    }

    fn time(&self) -> f64 {
        self.cur_time
    }

    fn state(&self) -> f64 {
        self.state
    }

    fn step(&mut self, rng: &mut impl Rng) {
        let det_step = self.det_distr.sample(rng);
        let rep_step = self.rep_distr.sample(rng);
        self.state *= self.discount_factor;
        self.state += det_step;
        self.state -= rep_step;
        self.cur_time += self.step_size;
    }
}

struct OuProcess{
    start_state: f64,
    diffusion: f64,
    drift: f64,
}

impl OuProcess {

    fn from_params(model_parameters: ModelParameters, start_state: f64) -> Self {
        let ModelParameters { det_mean:_, det_var, rep_mean: _, rep_var, self_reversion } = model_parameters;
        let drift = self_reversion;
        let diffusion = (drift/(1.-(-drift).exp())*(det_var + rep_var)).sqrt();
        
        Self {
            start_state,
            diffusion,
            drift,
        }
    }

    fn sample(&self, time: f64, rng: &mut impl Rng) -> f64 {
        let &Self { start_state, diffusion, drift } = self;
        let normal = Normal::new(0., ((2.*drift*time).exp()-1.).sqrt()).unwrap();
        (-drift*time).exp()*(start_state + diffusion/(2.*drift).sqrt()*normal.sample(rng))
    }
}

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        20 => question20::main(),
        21 => question21::main(),
        23 => question23::main(),
        26 => question26::main(),
        27 => question27::main(),
        _ => panic!("Unrecognized question number"),
    }
}
