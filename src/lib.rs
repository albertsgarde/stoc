use std::{
    fmt::Display,
    ops::{Add, Div},
};

use ndarray::Array1;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

mod brownian_motion;
pub use brownian_motion::{BrownianMotion, GeometricBrownianMotion};

pub type Vector = Array1<f64>;

pub type ExperimentRng = Pcg64Mcg;

pub trait Sample:
    Sized + Add<Self, Output = Self> + Div<f64, Output = Self> + Send + Sync + Display
{
    fn mean<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>;
}

impl<T> Sample for T
where
    T: Sized + Add<Self, Output = Self> + Div<f64, Output = Self> + Send + Sync + Display,
{
    fn mean<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
    {
        let mut iter = iter.into_iter();

        let mut sum = iter.next().unwrap();
        let mut count = 1;
        for x in iter {
            sum = sum + x;
            count += 1;
        }
        sum / count as f64
    }
}

fn run_experiment<P, E, S>(
    experiment: E,
    parameters: &P,
    num_samples: u32,
    max_threads: u32,
    rng: &mut impl Rng,
) -> S
where
    P: Sync,
    E: Fn(&P, &mut ExperimentRng) -> S + Sync + Send,
    S: Sample,
{
    let num_samples_per_thread = num_samples / max_threads;
    let seeds: Vec<_> = (0..max_threads).map(|_| rng.next_u64()).collect();

    let total: Vec<_> = seeds
        .into_par_iter()
        .map(|seed| {
            let mut rng = ExperimentRng::seed_from_u64(seed);
            let experiment = &experiment;
            (0..num_samples_per_thread).map(move |_| experiment(parameters, &mut rng))
        })
        .flatten_iter()
        .collect();

    S::mean(total)
}

#[derive(Debug)]
pub struct TestTheoryResult<S: Sample> {
    theoretical_result: S,
    empirical_mean: S,
}

impl<S> TestTheoryResult<S>
where
    S: Sample,
{
    pub fn parts(&self) -> (&S, &S) {
        (&self.theoretical_result, &self.empirical_mean)
    }
}

pub fn test_theory<P, E, S, T, R>(
    experiment: E,
    theory: T,
    parameters: &P,
    samples: u32,
    max_threads: u32,
    rng: &mut R,
) -> TestTheoryResult<S>
where
    P: Sync,
    E: Fn(&P, &mut ExperimentRng) -> S + Sync + Send,
    S: Sample,
    T: Fn(&P) -> S,
    R: Rng,
{
    let empirical_mean = run_experiment(experiment, parameters, samples, max_threads, rng);
    let theoretical_result = theory(parameters);
    TestTheoryResult {
        theoretical_result,
        empirical_mean,
    }
}
