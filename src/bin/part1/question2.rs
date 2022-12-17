use rand::Rng;
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, Vector};

use crate::{ModelParameters, MAX_THREADS, SEED, Process};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
    start_state: u64,
    days: u64,
    target: u64,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
        start_state,
        days,
        target,
    } = parameters;
    let mut process = Process::new(model_parameters, start_state);
    for _ in 0..days {
        process.step(rng);
    }

    if process.state() == target {
        1.
    } else {
        0.
    }
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        model_parameters,
        start_state,
        days,
        target,
    } = parameters;
    let mut start = vec![0.; model_parameters.a as usize+1];
    start[start_state as usize]= 1.;
    let start = Vector::from_shape_vec(model_parameters.a as usize+1, start).unwrap();
    let p = model_parameters.transition_matrix();
    let p10 = (0..days).fold(start, |cur_p, _| cur_p.dot(&p));
    dbg!(p10)[target as usize]
    
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);
        
    let parameters = Parameters {
        model_parameters: ModelParameters::default(),
        start_state: 0,
        days: 10,
        target: 4,
    };

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
