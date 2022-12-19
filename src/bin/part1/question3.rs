use ndarray::s;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use stoc::{test_theory, linux};

use crate::{ModelParameters, MAX_THREADS, SEED, Process};

#[derive(Debug, Clone, Copy)]
struct Parameters {
    model_parameters: ModelParameters,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters,
    } = parameters;
    let mut process = Process::new(model_parameters, 0);

    (0..).find(|_| {process.step(rng); process.state() == model_parameters.a}).unwrap() as f64
}

fn theory(parameters: &Parameters) -> f64 {
    let &Parameters {
        model_parameters,
    } = parameters;
    let p = model_parameters.transition_matrix();
    let _p = p.slice(s![0..model_parameters.a as usize, 0..model_parameters.a as usize]);
    linux!{
        {
        use ndarray_linalg::solve::Inverse;
        (Matrix::from_diag_elem(model_parameters.a as usize, 1.) - p).inv().unwrap().dot(&Vector::from_elem(model_parameters.a as usize, 1.))[0]
        }
    }
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);
        
    let parameters = Parameters {
        model_parameters: ModelParameters::default(),
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
