use rand::Rng;
use rand_distr::{Bernoulli, Distribution, Exp};
use rand_pcg::Pcg64Mcg;
use stoc::test_theory;

use crate::{ModelParameters, MAX_THREADS, SEED};

struct Parameters {
    model_parameters: ModelParameters,
}

fn experiment(parameters: &Parameters, rng: &mut impl Rng) -> f64 {
    let &Parameters {
        model_parameters:
            ModelParameters {
                lambda1,
                lambda2,
                p1,
                p2,
                mu,
            },
    } = parameters;

    let distr = if Bernoulli::new(p1).unwrap().sample(rng) {
        Exp::new(lambda1).unwrap()
    } else {
        Exp::new(lambda2).unwrap()
    };

    distr.sample(rng)
}

fn theory(_parameters: &Parameters) -> f64 {
    6.
}

pub fn main() {
    let mut rng = Pcg64Mcg::new(SEED);

    let parameters = Parameters {
        model_parameters: ModelParameters::from_p1(0.3),
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
