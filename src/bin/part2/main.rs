use std::env;

mod question8;

const SEED: u128 = 4;
const MAX_THREADS: u32 = 8;

pub struct ModelParameters {
    pub lambda1: f64,
    pub lambda2: f64,
    pub p1: f64,
    pub p2: f64,
    pub mu: f64,
}

impl ModelParameters {
    fn from_p1(p1: f64) -> Self {
        let p2 = 1. - p1;
        let lambda1 = (1. + 9. * p1) / 60.;
        let lambda2 = 10. * lambda1;
        assert!(p1 >= 0.);
        assert!(p1 <= 1.);
        assert!(lambda1 > 0.);
        assert!(lambda2 > 0.);
        ModelParameters {
            lambda1,
            lambda2,
            p1,
            p2,
            mu: 1.,
        }
    }
}

fn main() {
    let arg = env::args().nth(1).expect("No question number given.");
    let question = arg
        .parse::<u32>()
        .expect("Could not parse question number.");
    match question {
        8 => question8::main(),
        _ => panic!("Unrecognized question number"),
    }
}
